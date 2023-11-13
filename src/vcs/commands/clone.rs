use std::{net::TcpStream, io::{Read, Write, self, BufWriter}, str::from_utf8, path::{PathBuf, Path}, fs::OpenOptions, collections::HashMap};
use std::str::FromStr;

use rand::Rng;

use crate::{packfile::packfile::{read_packet, to_pkt_line, send_done_msg, decompress_data}, vcs::{version_control_system::VersionControlSystem, commands::{branch::BranchOptions, checkout::Checkout}, entities::{blob_entity::{BlobEntity, self}, entity::Entity, tree_entity::TreeEntity, commit_entity::{CommitEntity, self}}}, proxy::proxy::Proxy, constants::constants::{TREE_CODE_NUMBER, BLOB_CODE_NUMBER, COMMIT_CODE, COMMIT_CODE_NUMBER, NULL}};
pub struct Clone;

impl Clone{
    pub fn git_clone(stream: &mut TcpStream, repo: PathBuf) -> Result<(),std::io::Error> {
        Self::receive_pack(stream, repo)?;
        Ok(())
    }
    
    pub fn receive_pack(socket: &mut TcpStream, repo: PathBuf) -> Result<(), std::io::Error> {
        let mut packets = Vec::new();
        print!("Entro a receive packs ---------------\n");
        loop {
            let mut len_buf = [0; 4]; 
            if socket.read_exact(&mut len_buf).is_ok() {
                let len_str: &str = from_utf8(&len_buf).unwrap();
                let len = usize::from_str_radix(len_str, 16).unwrap();
                if len == 0 {
                    break;
                }
                println!("LEN del packet ---> {:?} \n", len);

                let packet = read_packet(socket, len);
                println!("ACA PACKETTT ---> {:?} \n", packet);
                packets.push(packet);
            }
        }
        for packet in &packets {
            println!("Paquete: {:?}", packet);
        }
        for want in Self::get_want_msgs(&packets) {
            socket.write(want.as_bytes())?;
        }
        send_done_msg(socket)?;
        let objects = Self::get_socket_response(socket)?;
        Self::init_commits(&packets , &objects, repo)?;
        Ok(()) 
    }

    fn init_commits(list_refs: &Vec<String>, objects: &Vec<(u8,Vec<u8>)>, repo: PathBuf) -> Result<(), std::io::Error>  {
        let mut objects_processed: Vec<(u8, String)> = Vec::new();
        let mut branch_name = String::new(); // Initialize branch_name
        let mut branchs: HashMap<String, String> = HashMap::new();
        println!("--------------------LIST REFERENCESSSS ---> {:?}\n", list_refs);

        objects_processed = Self::process_folder(objects.to_vec());
        for obj in &objects_processed{
            println!("-->{:?}", obj);
        }
        let commits_created = Self::create_folders(objects_processed.clone(), &repo);

        for item in list_refs {
            if item.contains("HEAD") {
                continue;
            }
            let parts: Vec<&str> = item.splitn(2, ' ').collect(); 
            if parts.len() == 2 {
                let commit = parts[0];
                let ref_part = parts[1];
                    if ref_part.starts_with("refs/") {
                        branch_name = ref_part.trim_start_matches("refs/heads/").to_string();
                        let _ = VersionControlSystem::branch(BranchOptions::NewBranch(branch_name.trim_end_matches('\n')));
                        println!("Commit: {}, Branch: {}", commit, branch_name);
                        branchs.insert(commit.to_owned(), branch_name);
                }
            }
            let _ = Self::write_commit_log(&repo, branchs, &commits_created, objects_processed.clone());
        }
        Checkout::update_cd(&repo)?;
        Ok(())
    }

    fn process_folder(objects: Vec<(u8,Vec<u8>)>) -> Vec<(u8, String)> {
        let mut objects_processed : Vec<(u8, String)> = Vec::new();
        for (number, inner_vec) in &objects {
            if *number != 2 {
                println!("({}, {:?})", number, String::from_utf8_lossy(inner_vec));
                objects_processed.push((*number, String::from_utf8_lossy(inner_vec).to_string()));
            }else{
               let content = String::from_utf8_lossy(inner_vec);
                if content.contains(&10064.to_string()) {
                    let mut reader = inner_vec.as_slice();
                    if let Ok(entries) = Self::read_tree_sha1(&mut reader) {
                        let entry_string: String = entries
                            .iter()
                            .map(|(mode, name, sha1)| {
                                let hex_string: String = sha1.iter().map(|byte| format!("{:02x}", byte)).collect();
                                format!("{}-{}-{}",mode, name, hex_string)
                            })
                            .collect::<Vec<String>>()
                            .join("\n");
                        objects_processed.push((*number, entry_string));
                    }
                     else {
                        eprintln!("Error decoding the tree object");
                    }
                }else{
                    //aca estaria la rpta de nuestro server no hace falta processed, solo pego directo ;)
                    println!("({}, {:?})", number, String::from_utf8_lossy(inner_vec));
                    objects_processed.push((*number, String::from_utf8_lossy(inner_vec).to_string()));
                }
            }
        }
        objects_processed
    }

     fn create_folders(objects: Vec<(u8, String)>, repo: &PathBuf) -> Vec<String> {
        let mut hash_tree = String::new();     
        let mut hash_commit = String::new();     
        let mut commits_created = Vec::new();

        for (index, content) in objects.iter() {
            match *index {
                COMMIT_CODE_NUMBER => {
                    let result = Self::create_commit_folder(&content, repo);
                    match result{
                        Ok(value) => commits_created.push(value),
                        Err(e) => println!("Error creating commit {}", e),
                    }
                }
                TREE_CODE_NUMBER => {
                    let result = Self::create_tree_folder(&content, repo);
                    match result {
                        Ok(value) => hash_tree = value,
                        Err(e) => println!("Error creating tree {}", e),
                    }
                },
                BLOB_CODE_NUMBER => Self::create_blob_folder(&content, repo),
                _ => println!("Type not identify {}", index),
            }
        }
        commits_created
    }
    
    fn create_commit_folder(content: &String, repo: &PathBuf) -> Result<String, io::Error>{
        // Separar la cadena por \n y guardar las partes en un vector
        let partes: Vec<&str> = content.split("\n").collect();
        // Crear la entidad de commit con los datos de la cadena
        let commit_entity: CommitEntity;
        
        if !content.contains("parent"){
            let commit_entity = CommitEntity{
                    content_type: "commit".to_string(),
                    tree_hash: partes[0].to_string(), // La primera parte es el hash del árbol
                    message: partes[4..].join("\n").trim_start_matches("\n").to_string(), // La quinta parte en adelante es el mensaje del commit, se unen con \n y se elimina el \n inicial
                    author: partes[1].to_string(), // La segunda parte es el autor del commit
                    committer: partes[2].to_string(),
                    parent: "".to_string(), // La tercera parte es el committer del commit
                };
            }else{
                let commit_entity = CommitEntity{
                    content_type: "commit".to_string(),
                    tree_hash: partes[0].to_string(),
                    message: partes[5..].join("\n").trim_start_matches("\n").to_string(), 
                    author: partes[2].to_string(), 
                    committer: partes[3].to_string(),
                    parent: partes[1].to_string(),
                };
        }
 
        // Escribir el commit en el repositorio y devolver el hash del commit
        let hash_commit = Proxy::write_commit(repo.clone(), commit_entity);
        hash_commit
    }
    

    fn create_blob_folder(content: &String, repo: &PathBuf){
        let _ = Proxy::write_blob(repo.clone(),content);
    }


    fn create_tree_folder(content: &String, repo: &PathBuf) -> Result<String, std::io::Error> {
        let mut entities: Vec<Entity> = Vec::new();
    
        let entity_strings: Vec<&str> = content.split('\n').collect();
    
        for entries in entity_strings {
            let parts: Vec<&str> = entries.split('-').collect();
    
            let path = parts[1].trim();
            let entity_hash = parts[2].trim();
            if entries.contains("100644") {
                let blob_entity = BlobEntity {
                    content_type: "blob".to_string(),
                    path: format!("{}", path.to_string()),
                    blob_hash: entity_hash.to_string(),
                };
                entities.push(Entity::Blob(blob_entity));
            } else {
                let tree_entity = TreeEntity {
                    content_type: "tree".to_string(),
                    path: format!("{}/{}", repo.display(), path.to_string()),
                    tree_hash: entity_hash.to_string(),
                    entities: Vec::new(), // Initialize with an empty vector
                };
                entities.push(Entity::Tree(tree_entity));
            }
        }
        let hash_tree = Proxy::write_tree(repo.clone(), entities)?;
        Ok(hash_tree)
    }
    

    fn write_commit_log( repo: &PathBuf, branchs: HashMap<String, String>, commits_created: &Vec<String>, objects: Vec<(u8, String)>) -> Result<(), std::io::Error> {
        
        for (hash_commit_branch, value) in branchs{
            if commits_created.contains(&hash_commit_branch){
                let logs_path = repo.join(".rust_git").join("logs").join(value.trim_end_matches("\n"));
                let file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .append(true)
                    .open(&logs_path)?;
            
                let mut writer = BufWriter::new(file);
                let random_number: u8 = rand::thread_rng().gen_range(1..=9);
        
                let format_commit = format!("{}-{}-{}-{}", random_number, hash_commit_branch, "message", "2023-11-08 19:26:10.805633340 -03:00");
                println!("Format commit ------->{}  EN LA RAMA {} \n", format_commit, hash_commit_branch);
                writeln!(writer, "{}", format_commit)?; 
            }
        }
        Ok(())
    }
    
    fn get_want_msgs(commits_list: &Vec<String>) -> Vec<String> {
        let mut want_msgs = Vec::new();
    
        for commit in commits_list {
            let msg_commit = format!("want {}", commit);                
            let pkt_commit = to_pkt_line(&msg_commit);
            if commit.contains("HEAD"){
                continue;
            }
            want_msgs.push(pkt_commit);
        }
        println!("WANTS MESSAGE --> {:?} \n", want_msgs);
        want_msgs
    }


    fn get_socket_response(socket: &mut TcpStream) -> Result<Vec<(u8,Vec<u8>)>,std::io::Error> {
        let mut buffer = Vec::new();
            match socket.read_to_end(&mut buffer) {
                Ok(_) => {
                    return Self::manage_pack(&buffer[8..]);
                }
                Err(e) => {
                    println!("Failed to receive data: {}\n", e);
                    return Err(e)
                }
            } 
    }

    fn manage_pack(pack: &[u8])  -> Result<Vec<(u8,Vec<u8>)>,std::io::Error> {
        let signature_pack_msg = &pack[0..4];
        println!("SIGNATURE: {:?} - {:?}", signature_pack_msg, String::from_utf8_lossy(signature_pack_msg));
        let version = &pack[4..8];
        println!("VERSION: {:?} - {:?}", version, Self::parse_number(version)?);
        let object_number = Self::parse_number(&pack[8..12])?;
        println!("CANTIDAD DE OBJETOS: {}", object_number);
        
        let mut position: usize = 12;
        let mut objects = Vec::new();
        for object in 0..object_number {
            let objet_type = Self::get_object_type(pack[position]);
            while Self::is_bit_set(pack[position]) {
                position = position + 1;
            }
            position = position + 1;

            if let Ok(data) = decompress_data(&pack[position..]) {
                println!("TIPO OBJETO {}: {:?}, TAMAÑO OBJETO {}: {:?}", object+1, objet_type, object+1, data.1);
                println!("DATA OBJETO {}: {}", object+1, String::from_utf8_lossy(&data.0));
                position = position + data.1 as usize; 
                objects.push((objet_type, data.0))   
            }
        }
        objects.sort_by(|a, b| a.0.cmp(&b.0));
        
        Ok(objects)
    }


    fn read_cstring<R: Read>(reader: &mut R) -> io::Result<String> {
        let mut buffer = Vec::new();
        loop {
            let mut byte = [0];
            reader.read_exact(&mut byte)?;
            if byte[0] == 0 {
                break;
            }
            buffer.push(byte[0]);
        }
        Ok(String::from_utf8_lossy(&buffer).to_string())
    }


    fn from_octal_string(s: &str) -> Result<u32, std::num::ParseIntError> {
        u32::from_str_radix(s, 8)
    }

    fn read_tree_entry<R: Read>(reader: &mut R) -> io::Result<(String, String, Vec<u8>)> {
        let mut mode_bytes = [0; 6];
        reader.read_exact(&mut mode_bytes)?;
    
        let binding = String::from_utf8_lossy(&mode_bytes[..]);
        let mode_str = binding.trim();    
        let name = Self::read_cstring(reader)?;
    
        let mut sha1 = vec![0; 20];
        reader.read_exact(&mut sha1)?;

        Ok((mode_str.to_string(), name, sha1))
    }

    

    fn parse_number(bytes: &[u8]) -> Result<u8, std::io::Error> {
        let texto: String = bytes.iter().map(|&b| b.to_string()).collect();
        match texto.parse() {
            Ok(numero) => return Ok(numero),
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Can not parse number")),
        }
    }

    fn read_tree_sha1<R: Read>(reader: &mut R) -> io::Result<Vec<(String, String, Vec<u8>)>> {
        let mut entries = Vec::new();
        //println!("READERRR :::::::::::::{:?}\n", reader);
        while let Ok(entry) = Self::read_tree_entry(reader) {
            entries.push(entry);
        }
    
        Ok(entries)
    }


    fn get_object_type(bytes: u8) -> u8 {
        let mut bits = Vec::new();
        for i in (0..8).rev() {
            if i == 4 || i == 5 || i == 6 {
                let bit = (bytes >> i) & 1;
                bits.push(bit);
            } 
        }
        let mut numero = 0;
        for bit in &bits {
            numero = (numero << 1) | bit;
        }
        numero
    }

    fn is_bit_set(byte: u8) -> bool {
        let mask = 0b10000000;
        (byte & mask) == mask
     }

}