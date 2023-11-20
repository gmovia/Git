use std::{net::TcpStream, io::{Read, Write, self, BufWriter}, str::from_utf8, path::{PathBuf, Path}, fs::OpenOptions, collections::HashMap};

use rand::Rng;

use crate::{packfiles::packfile::{read_packet, to_pkt_line, send_done_msg, decompress_data}, vcs::{version_control_system::VersionControlSystem, commands::{branch::BranchOptions, checkout::Checkout}, entities::commit_entity::CommitEntity}, proxies::proxy::Proxy, constants::constant::{TREE_CODE_NUMBER, BLOB_CODE_NUMBER, COMMIT_CODE_NUMBER, COMMIT_INIT_HASH}};

use super::{cat_file::CatFile, init::Init};
pub struct Clone;

impl Clone{
    pub fn git_clone(stream: &mut TcpStream, repo: PathBuf) -> Result<(),std::io::Error> {
        Self::receive_pack(stream, repo)?;
        Ok(())
    }
    
    pub fn receive_pack(socket: &mut TcpStream, repo: PathBuf) -> Result<(), std::io::Error> {
        let mut packets = Vec::new();
        println!("Entro a receive packs ---------------\n");
        loop {
            let mut len_buf = [0; 4]; 
            if socket.read_exact(&mut len_buf).is_ok() {
                let len_str: &str = from_utf8(&len_buf).unwrap();
                let len = usize::from_str_radix(len_str, 16).unwrap();
                if len == 0 {
                    break;
                }

                let packet = read_packet(socket, len);
                packets.push(packet);
            }
        }
        for packet in &packets {
            println!("Paquete: {:?}", packet);
        }
        for want in Self::get_want_msgs(&packets) {
            socket.write_all(want.as_bytes())?;
        }
        send_done_msg(socket)?;
        let objects = Self::get_socket_response(socket)?;
        Self::init_commits(&packets , &objects, repo)?;
        Ok(()) 
    }

    fn init_commits(list_refs: &Vec<String>, objects: &[(u8,Vec<u8>)], repo: PathBuf) -> Result<(), std::io::Error>  {
        let mut branchs: HashMap<String, String> = HashMap::new();

        let objects_processed = Self::process_folder(objects.to_vec());
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
                        let branch_name = ref_part.trim_start_matches("refs/heads/").to_string();
                        let _ = VersionControlSystem::branch(BranchOptions::NewBranch(branch_name.clone().trim_end_matches('\n')));
                        println!("Commit: {}, Branch: {}", commit, branch_name);
                        branchs.insert(branch_name, commit.to_owned());
                }
            }
        }
        let _ = Self::write_commit_log(&repo, branchs.clone(), &commits_created, objects_processed.clone());
        Checkout::update_cd(&repo)?; //Esto recien me crea los files.txt en working directory si tiene en la tabla de commits lleno
        Ok(())
    }

    fn process_non_tree_object(number: u8, inner_vec: &[u8]) -> (u8, String) {
        (number, String::from_utf8_lossy(inner_vec).to_string())
    }

    fn process_tree_object(number: u8, inner_vec: &Vec<u8>) -> (u8, String) {
        if std::str::from_utf8(inner_vec).is_ok() {
            let blobs: Vec<String> = String::from_utf8_lossy(inner_vec).split('\n').map(String::from).collect();
            let mut string_to_send = String::new();
            for blob in &blobs {
                let blob_parts: Vec<&str> = blob.split(' ').collect();
                if blob_parts.len() == 3 {
                    let path = Path::new(blob_parts[1]);
                    if let Some(file_name) = path.file_name() {
                        string_to_send = format!("{}{}-{}-{}\n", string_to_send, blob_parts[0], file_name.to_string_lossy(), blob_parts[2]);  
                    }
                }                      
            }
            (number, string_to_send)
        } else {
            let mut reader = inner_vec.as_slice();
        
            if let Ok(entries) = Self::read_tree_sha1(&mut reader) {
                let entry_string: String = entries
                    .iter()
                    .map(|(mode, name, sha1)| {
                        let hex_string: String = sha1.iter().map(|byte| format!("{:02x}", byte)).collect();
                        format!("{}-{}-{}", mode, name, hex_string)
                    })
                    .collect::<Vec<String>>()
                    .join("\n");
                (number, entry_string)
            } else {
                eprintln!("Error decoding the tree object");
                (number, String::new())
            }
        }
    }

    fn process_folder(objects: Vec<(u8,Vec<u8>)>) -> Vec<(u8, String)> {
        let mut objects_processed : Vec<(u8, String)> = Vec::new();
        for (number, inner_vec) in &objects {
            if *number != TREE_CODE_NUMBER {
                objects_processed.push(Self::process_non_tree_object(*number, inner_vec));
            }else{
                objects_processed.push(Self::process_tree_object(*number, inner_vec));
            }
        }
        objects_processed
    }



     fn create_folders(objects: Vec<(u8, String)>, repo: &Path) -> HashMap<String, CommitEntity>{
        let mut commits_created: HashMap<String, CommitEntity> = HashMap::new();

        for (index, content) in objects.iter() {
            match *index {
                COMMIT_CODE_NUMBER => {
                    match Self::create_commit_folder(content, repo) {
                        Ok((hash, commit_entity)) => {
                            commits_created.insert(hash.clone(), commit_entity);
                        },
                        Err(e) => {
                            println!("Error creating commit: {}", e);
                        },
                    }
                }
                TREE_CODE_NUMBER => {
                    if let Err(e) = Self::create_tree_folder(content, repo) {
                        println!("Error creating tree {}", e);
                    }
                },
                BLOB_CODE_NUMBER => Self::create_blob_folder(content, repo),
                _ => println!("Type not identify {}", index),
            }
        }
        commits_created
    }
    
    fn create_commit_folder(content: &str, repo: &Path) -> Result<(String, CommitEntity), std::io::Error>{
        let partes: Vec<&str> = content.split('\n').collect();
        
        let commit_entity: CommitEntity = if !content.contains("parent"){
                CommitEntity{
                content_type: "commit".trim_end_matches('\n').to_string(),
                tree_hash: partes[0].trim_end_matches('\n').trim_start_matches("tree ").to_string(),
                message: partes[4..].join("\n").trim_start_matches('\n').trim_end_matches('\n').to_string(), 
                author: partes[1].trim_end_matches('\n').trim_start_matches('\n').to_string(), 
                committer: partes[2].trim_end_matches('\n').to_string(),
                parent_hash: COMMIT_INIT_HASH.to_string(),
            }
        }else{
                CommitEntity{
                content_type: "commit".to_string(),
                tree_hash: partes[0].trim_end_matches('\n').trim_start_matches("tree ").to_string(),
                message: partes[5..].join("\n").trim_start_matches('\n').trim_end_matches('\n').to_string(), 
                author: partes[2].trim_end_matches('\n').trim_start_matches('\n').to_string(), 
                committer: partes[3].trim_end_matches('\n').to_string(),
                parent_hash: partes[1].trim_end_matches('\n').trim_start_matches('\n').trim_start_matches("parent ").to_string(),
            }
        };
        let hash_commit = Proxy::write_commit(repo.to_path_buf(), &commit_entity)?;

        Ok((hash_commit, commit_entity))
    }
    

    fn create_blob_folder(content: &String, repo: &Path){
        let _ = Proxy::write_blob(repo.to_path_buf(),content);
    }

    fn create_tree_folder(content: &str, repo: &Path) -> Result<String, std::io::Error> {
        Proxy::write_tree(repo.to_path_buf(), content)
    }
    

    fn write_commit_log( repo: &PathBuf, branchs: HashMap<String, String>, commits_created:  &HashMap<String, CommitEntity>, _objects: Vec<(u8, String)>) -> Result<(), std::io::Error> {
        for (branch_name, hash_commit_branch) in &branchs{ // 2 nombre_rama, hash

            if commits_created.contains_key(hash_commit_branch) {
                let logs_path = repo.join(".rust_git").join("logs").join(branch_name.trim_end_matches('\n'));
                let file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .append(true)
                    .open(&logs_path)?;

                let _writer = BufWriter::new(file);
                let random_number: u8 = rand::thread_rng().gen_range(1..=9);

                if let Some(commit_entity) = commits_created.get(hash_commit_branch) {
                    let date = Self::get_date(&commit_entity.author);
                    let format_commit = format!("{}-{}-{}-{}-{}", random_number, commit_entity.parent_hash, hash_commit_branch, commit_entity.message, date);
                    println!("Format commit ------->{}  EN LA RAMA {} \n", format_commit, hash_commit_branch);
                    let a: Vec<(String, String)> = vec![(branch_name.to_string(), hash_commit_branch.to_string())];
                    let _ = Self::complete_commit_table(repo, a, commits_created);
                }
            }
        }
        Ok(())
    }

    fn complete_commit_table(repo: &PathBuf, branchs: Vec<(String, String)>, commits_created:  &HashMap<String, CommitEntity>) -> Result<(), std::io::Error> {
        //branch_name, hash
        for(branch_name, hash_commit_branch)in branchs {
            let logs_path = repo.join(".rust_git").join("logs").join(branch_name.trim_end_matches('\n'));
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(&logs_path)?;
            let content = CatFile::cat_file(&hash_commit_branch, Init::get_object_path(repo)?)?;

            if content.contains("parent"){
                let part:Vec<&str> = content.split('\n').collect();
                let hash_parent = part[1].trim_start_matches("parent ");
                let mut a: Vec<(String, String)> = Vec::new();
                if let Some(commit_entity) = commits_created.get(&hash_commit_branch){
                    let random_number: u8 = rand::thread_rng().gen_range(1..=9);
                    let date = Self::get_date(&commit_entity.author);
                    let format_commit = format!("{}-{}-{}-{}-{}\n", random_number, commit_entity.parent_hash, hash_commit_branch, commit_entity.message, date);
                    a.push((branch_name.to_string(), hash_parent.to_string()));
                    let _ = Self::complete_commit_table(repo, a, commits_created);
                    file.write_all(format_commit.as_bytes())?;
                }
            }else{
                let part:Vec<&str> = content.split('\n').collect();
                let hash_parent = part[1].trim_start_matches("parent ");
                let mut a: Vec<(String, String)> = Vec::new();
                if let Some(commit_entity) = commits_created.get(&hash_commit_branch){
                    let random_number: u8 = rand::thread_rng().gen_range(1..=9);
                    let date = Self::get_date(&commit_entity.author);
                    let format_commit = format!("{}-{}-{}-{}-{}\n", random_number, commit_entity.parent_hash, hash_commit_branch, commit_entity.message, date);
                    a.push((branch_name.to_string(), hash_parent.to_string()));
                    file.write_all(format_commit.as_bytes())?;
                }
            }
        }
        Ok(())
    }

    fn get_date(line: &str) -> &str {
        let start = match line.find('>') {
            Some(pos) => pos + 2, 
            None => 0, 
        };
        &line[start..] 
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
                    Self::manage_pack(&buffer[8..])
                }
                Err(e) => {
                    println!("Failed to receive data: {}\n", e);
                    Err(e)
                }
            } 
    }

    fn manage_pack(pack: &[u8])  -> Result<Vec<(u8,Vec<u8>)>,std::io::Error> {
        let object_number = Self::parse_number(&pack[8..12])?;
        
        let mut position: usize = 12;
        let mut objects = Vec::new();
        for object in 0..object_number {
            let objet_type = Self::get_object_type(pack[position]);
            while Self::is_bit_set(pack[position]) {
                position += 1;
            }
            position += 1;

            if let Ok(data) = decompress_data(&pack[position..]) {
                println!("TIPO OBJETO {}: {:?}, TAMAÑO OBJETO {}: {:?}", object+1, objet_type, object+1, data.1);
                println!("DATA OBJETO {}: {}", object+1, String::from_utf8_lossy(&data.0));
                position += data.1 as usize; 
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
            Ok(numero) => Ok(numero),
            Err(_) => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Can not parse number")),
        }
    }

    fn read_tree_sha1<R: Read>(reader: &mut R) -> io::Result<Vec<(String, String, Vec<u8>)>> {
        let mut entries = Vec::new();
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