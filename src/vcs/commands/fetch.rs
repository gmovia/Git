use std::{net::TcpStream, path::{PathBuf, Path}, io::{Read, Write, self, BufRead}, str::from_utf8, collections::HashMap, fs::{File, OpenOptions, self}};
use crate::{packfiles::packfile::{read_packet, send_done_msg, to_pkt_line, decompress_data}, vcs::{commands::branch::Branch, entities::commit_entity::CommitEntity}, constants::constant::{TREE_CODE_NUMBER, COMMIT_INIT_HASH}, proxies::proxy::Proxy, utils::randoms::random::Random};
use super::{cat_file::CatFile, init::Init};

pub struct Fetch;

impl Fetch {

    pub fn git_fetch(stream: &mut TcpStream, repo: PathBuf) -> Result<(),std::io::Error> {
        println!("FETCH ENTRA");
        Self::receive_pack(stream, repo)?;
        Ok(())
    }


    pub fn receive_pack(socket: &mut TcpStream, repo: PathBuf) -> Result<(), std::io::Error> {
        let mut packets = Vec::new();
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

        let last_commit_per_branch = Self::format_packet(&packets)?;
        println!("LAST COMMIT: {:?}",last_commit_per_branch);
        let message_to_send = Self::packet_manager(last_commit_per_branch, &repo)?;
        println!("Message to send: {:?}", message_to_send);

        Self::send_messages(socket, message_to_send)?;

        let objects = Self::get_socket_response(socket)?;
        println!("OBJETOS: {:?}", objects);
        Self::create_objects(&packets , &objects, &repo)?;
        Ok(()) 
    }

    fn create_objects(packets: &Vec<String> , objects: &Vec<(u8, Vec<u8>)>, clinet_path: &PathBuf) -> Result<(),std::io::Error> {
        println!("PACKETS: {:?}", packets);
        println!("OBJECTS: {:?}", objects);
        println!("CLIENT PATH: {:?}", clinet_path);
        
        let mut branchs: HashMap<String, String> = HashMap::new();
        println!("--------------------LIST REFERENCESSSS ---> {:?}\n", packets);

        let objects_processed = Self::process_folder(objects.to_vec());
        for obj in &objects_processed{
            println!("-->{:?}", obj);
        }
        let commits_created = Self::create_folders(objects_processed.clone(), &clinet_path)?;
        

        for item in packets {
            if item.contains("HEAD") {
                continue;
            }
            let parts: Vec<&str> = item.splitn(2, ' ').collect(); 
            if parts.len() == 2 {
                let commit = parts[0];
                let ref_part = parts[1];
                    if ref_part.starts_with("refs/") {
                        let branch_name = ref_part.trim_start_matches("refs/heads/").to_string();
                        //let _ = VersionControlSystem::branch(BranchOptions::NewBranch(branch_name.clone().trim_end_matches('\n')));
                        let _ = Branch::create_new_branch_with_hash(&clinet_path, &branch_name.trim_end_matches('\n'), commit);
                        println!("Commit: {}, Branch: {}", commit, branch_name);
                        branchs.insert(branch_name, commit.to_owned());
                }
            }
        }
        let _ = Self::write_commit_log(&clinet_path, branchs.clone(), &commits_created, objects_processed.clone());
        Ok(())
    }

    fn process_non_tree_object(number: u8, inner_vec: &Vec<u8>) -> (u8, String) {
        println!("({}, {:?})", number, String::from_utf8_lossy(inner_vec));
        (number, String::from_utf8_lossy(inner_vec).to_string())
    }

    fn process_tree_object(number: u8, inner_vec: &Vec<u8>) -> (u8, String) {
        if let Ok(_) = std::str::from_utf8(inner_vec) {
            let blobs: Vec<String> = String::from_utf8_lossy(inner_vec).split("\n").map(String::from).collect();
            let mut string_to_send = String::new();
            for blob in &blobs {
                let blob_parts: Vec<&str> = blob.split(" ").collect();
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

    fn read_tree_sha1<R: Read>(reader: &mut R) -> io::Result<Vec<(String, String, Vec<u8>)>> {
        let mut entries = Vec::new();
        while let Ok(entry) = Self::read_tree_entry(reader) {
            entries.push(entry);
        }
    
        Ok(entries)
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

    fn create_folders(objects: Vec<(u8, String)>, repo: &PathBuf) -> Result<HashMap<String, CommitEntity>, std::io::Error>{
        let mut commits_created: HashMap<String, CommitEntity> = Self::add_commits(&repo)?;

        for (index, content) in objects.iter() {
            match *index {
                1 => {
                    match Self::create_commit_folder(&content, repo) {
                        Ok((hash, commit_entity)) => {
                            commits_created.insert(hash.clone(), commit_entity);
                        },
                        Err(e) => {
                            println!("Error creating commit: {}", e);
                        },
                    }
                }
                2 => {
                    if let Err(e) = Self::create_tree_folder(&content, repo) {
                        println!("Error creating tree {}", e);
                    }
                },
                3 => Self::create_blob_folder(&content, repo),
                _ => println!("Type not identify {}", index),
            }
        }
        Ok(commits_created)
    }
    
    fn add_commits(client_path: &PathBuf) -> Result<HashMap<String, CommitEntity>, std::io::Error> {
        let mut commits: HashMap<String, CommitEntity> = HashMap::new();
        let logs_path = client_path.join(".rust_git").join("logs");

        if let Ok(entries) = fs::read_dir(&logs_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if entry.file_type()?.is_file() {
                        let content = fs::read_to_string(entry.path())?;
                        let lines = content.split("\n");
                        for line in lines {
                            if line.contains("-") {
                                let parts: Vec<&str> = line.split("-").collect();
                                let commit_hash = parts[2];
                                let commit_entity = CommitEntity::read(client_path, commit_hash)?;
                                commits.insert(commit_hash.to_string(), commit_entity);
                            }
                        }
                    }
                }
            }
        } else {
            return Err(std::io::Error::new(io::ErrorKind::NotFound, "Directory not found"));
        }
        Ok(commits)
    }


    fn create_commit_folder(content: &String, repo: &PathBuf) -> Result<(String, CommitEntity), std::io::Error>{
        let partes: Vec<&str> = content.split("\n").collect();
        let commit_entity: CommitEntity;
        
        if !content.contains("parent"){
            commit_entity = CommitEntity{
                content_type: "commit".trim_end_matches("\n").to_string(),
                tree_hash: partes[0].trim_end_matches("\n").trim_start_matches("tree ").to_string(),
                message: partes[4..].join("\n").trim_start_matches("\n").trim_end_matches("\n").to_string(), 
                author: partes[1].trim_end_matches("\n").trim_start_matches("\n").to_string(), 
                committer: partes[2].trim_end_matches("\n").to_string(),
                parent_hash: COMMIT_INIT_HASH.to_string(),
            };
        }else{
            commit_entity = CommitEntity{
                content_type: "commit".to_string(),
                tree_hash: partes[0].trim_end_matches("\n").trim_start_matches("tree ").to_string(),
                message: partes[5..].join("\n").trim_start_matches("\n").trim_end_matches("\n").to_string(), 
                author: partes[2].trim_end_matches("\n").trim_start_matches("\n").to_string(), 
                committer: partes[3].trim_end_matches("\n").to_string(),
                parent_hash: partes[1].trim_end_matches("\n").trim_start_matches("\n").trim_start_matches("parent ").to_string(),
            };
        }
        let hash_commit = Proxy::write_commit(repo.clone(), &commit_entity)?;

        Ok((hash_commit, commit_entity))
    }
    

    fn create_blob_folder(content: &String, repo: &PathBuf){
        let _ = Proxy::write_blob(repo.clone(),content);
    }

    fn create_tree_folder(content: &String, repo: &PathBuf) -> Result<String, std::io::Error> {
        Ok(Proxy::write_tree(repo.to_path_buf(), content)?)
    }

    fn write_commit_log( repo: &PathBuf, branchs: HashMap<String, String>, commits_created:  &HashMap<String, CommitEntity>, _objects: Vec<(u8, String)>) -> Result<(), std::io::Error> {
        println!("COMMITS CREATEDD ----> {:?}\n", commits_created.keys());
        println!("LEN DE COMMIT CREATED ---< {:?}\n", commits_created.len());
        for (branch_name, hash_commit_branch) in &branchs{ // 2 nombre_rama, hash
            if commits_created.contains_key(hash_commit_branch) {
                let logs_path = repo.join(".rust_git").join("logs").join(branch_name.trim_end_matches("\n"));
                let file = OpenOptions::new().create(true).write(true).append(true).open(&logs_path)?;
                file.set_len(0)?;
                let _ = Self::complete_commit_table(repo, &branch_name.to_string(), &hash_commit_branch.to_string(), commits_created);
            }
        }
        Ok(())
    }

    fn complete_commit_table(repo: &PathBuf, branch_name: &String, hash_commit_branch: &String, commits_created:  &HashMap<String, CommitEntity>) -> Result<(), std::io::Error> {
        //branch_name, hash
        let logs_path = repo.join(".rust_git").join("logs").join(branch_name.trim_end_matches("\n"));
        let mut file = OpenOptions::new().create(true).write(true).append(true).open(&logs_path)?;
        

        let content = CatFile::cat_file(&hash_commit_branch, Init::get_object_path(&repo)?)?;
        let id = Random::random();

        if content.contains("parent"){
            let part:Vec<&str> = content.split("\n").collect();
            let hash_parent = part[1].trim_start_matches("parent ");
            if let Some(commit_entity) = commits_created.get(hash_commit_branch){
                let date = Self::get_date(&commit_entity.author);
                let format_commit = format!("{}-{}-{}-{}-{}\n", id, commit_entity.parent_hash, hash_commit_branch, commit_entity.message, date);
                let _ = Self::complete_commit_table(repo, branch_name, &hash_parent.to_string(), commits_created);
                println!("FORMAT COMMIT PARENT: {}", &format_commit);
                file.write(format_commit.as_bytes())?;
            }
        }else{
            if let Some(commit_entity) = commits_created.get(hash_commit_branch){
                let date = Self::get_date(&commit_entity.author);
                let format_commit = format!("{}-{}-{}-{}-{}\n", id, commit_entity.parent_hash, hash_commit_branch, commit_entity.message, date);
                println!("FORMAT COMMIT: {}", &format_commit);
                file.write(format_commit.as_bytes())?;
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


    /// Esta funcion se encarga de parsear la respuesta del servidor al upload pack. Devuelve la rama y el ultimo commit
    fn format_packet(packets: &Vec<String>) -> Result<Vec<(String,String)>,std::io::Error> {
        let mut branch_commit: Vec<(String,String)> = Vec::new();
        for packet in packets {
            let parts: Vec<&str> = packet.splitn(2, ' ').collect();
            branch_commit.push((parts[1].trim_start_matches("refs/heads/").to_owned(), parts[0].to_owned()));
        }

        let mut last_entries: HashMap<&String, &String> = std::collections::HashMap::new();
        let mut last_commits: Vec<(String, String)> = Vec::new();
    
        for (key, value) in branch_commit.iter().rev() {
            if !last_entries.contains_key(&key) {
                last_entries.insert(key, value);
                last_commits.push((key.clone(), value.clone()));
            }
        }
        last_commits.reverse();
        Ok(last_commits)
    }

    fn send_messages(socket: &mut TcpStream, message_to_send: (Vec<String>,Vec<String>)) -> Result<(), std::io::Error> {
        for want in &message_to_send.0 {
            println!("WANT ENVIADO: {}", want);
            socket.write(want.as_bytes())?;
        }

        if !&message_to_send.0.is_empty() {
            for have in &message_to_send.1 {
                println!("HAVE ENVIADO: {}", have);
                socket.write(have.as_bytes())?;
            }
        }
        send_done_msg(socket)?;
        Ok(())
    }

    fn packet_manager(last_branch_commit_recieve: Vec<(String,String)>, repo: &PathBuf) -> Result<(Vec<String>,Vec<String>), std::io::Error>{
        let mut want_list: Vec<String> = Vec::new();
        let mut have_list: Vec<String> = Vec::new();
        for packet in &last_branch_commit_recieve {
            println!("PACKET HASH: {}", packet.0);
            match File::open(&repo.join(".rust_git").join("logs").join(packet.0.to_string())) { 
                Ok(file) => {
                    let reader = io::BufReader::new(file);
            
                    let mut last_line = String::new();
                    for line in reader.lines() {
                        last_line = line?;
                    }
                    println!("last line: {} -- {}", last_line, &packet.1);
                    let parts: Vec<&str> = last_line.split("-").collect();
                    if parts[2] == packet.1 {
                        have_list.push(to_pkt_line(&format!("have {} refs/heads/{}", packet.1, &packet.0)));
                    }
                    else {
                        want_list.push(to_pkt_line(&format!("want {} refs/heads/{}", packet.1, &packet.0)));
                        have_list.push(to_pkt_line(&format!("have {} refs/heads/{}", parts[2], &packet.0)));
                    }
                }
                Err(_) => {
                    want_list.push(to_pkt_line(&format!("want {} refs/heads/{}", packet.1, &packet.0)));    
                }
            }            
        }
        Ok((want_list,have_list))
    }

    fn get_socket_response(socket: &mut TcpStream) -> Result<Vec<(u8,Vec<u8>)>,std::io::Error> {
        let mut buffer = Vec::new();
            match socket.read_to_end(&mut buffer) {
                Ok(_) => {
                    if buffer.is_empty() {
                        println!("\nAlready up to date\n");
                        return Ok(Vec::new());
                    }
                    else {
                        return Self::manage_pack(&buffer[8..]);    
                    }
                    
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
        let pack_len = pack.len();
        for object in 0..object_number {
            if pack_len <= position {
                break;
            }
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
        for object in &objects {
            println!("------> TYPE: {}, CONTENT: {:?}", object.0, String::from_utf8_lossy(&object.1));
        }
        Ok(objects)
    }


    fn is_bit_set(byte: u8) -> bool {
        let mask = 0b10000000;
        (byte & mask) == mask
    }

    fn parse_number(bytes: &[u8]) -> Result<u8, std::io::Error> {
        let texto: String = bytes.iter().map(|&b| b.to_string()).collect();
        match texto.parse() {
            Ok(numero) => return Ok(numero),
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Can not parse number")),
        }
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

}