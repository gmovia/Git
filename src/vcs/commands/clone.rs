use std::{net::TcpStream, io::{Read, Write, self, SeekFrom, Seek}, str::from_utf8, path::Path, fs::{OpenOptions, File}, collections::HashMap, ascii::AsciiExt};
use crate::{packfiles::packfile::{read_packet, to_pkt_line, send_done_msg, decompress_data}, vcs::{commands::{branch::Branch, checkout::Checkout}, entities::commit_entity::CommitEntity, files::current_repository::CurrentRepository, version_control_system::VersionControlSystem}, proxies::proxy::Proxy, constants::constant::{TREE_CODE_NUMBER, BLOB_CODE_NUMBER, COMMIT_CODE_NUMBER, COMMIT_INIT_HASH, TAG_CODE_NUMBER, OBJ_REF_DELTA_CODE_NUMBER}, utils::randoms::random::Random, handlers::cat_file::handler_cat_file};
use super::{cat_file::CatFile, init::Init};
pub struct Clone;

impl Clone{
    pub fn git_clone(stream: &mut TcpStream, repo: &Path) -> Result<(),std::io::Error> {
        Self::receive_pack(stream, repo)?;
        Ok(())
    }
    
    pub fn receive_pack(socket: &mut TcpStream, repo: &Path) -> Result<(), std::io::Error> {
        let mut packets = Vec::new();
        
        loop {
            let mut len_buf = [0; 4]; 
            if socket.read_exact(&mut len_buf).is_ok() {
                if let Ok(len_str) = from_utf8(&len_buf) {
                    if let Ok(len) = usize::from_str_radix(len_str, 16) {
                        if len == 0 {
                            break;
                        }
    
                        let packet = read_packet(socket, len);
                        if packet.contains("fatal error") {
                            return Err(std::io::Error::new(std::io::ErrorKind::Other, "fatal error: the path is not correct"));
                        }
                        
                        println!("ACA PACKETTT ---> {:?} \n", packet);
                        packets.push(packet);
                    }
                }
            }
        }

        for want in Self::get_want_msgs(&packets) {
            let _ = socket.write_all(want.as_bytes());
        }

        send_done_msg(socket)?;
        let objects = Self::get_socket_response(socket)?;
        let (list_tags, list_refs) = Self::exclude_tag_ref(packets.clone())?;

        Self::create_tag_files(list_tags)?;
        Self::init_commits(&list_refs , &objects, repo)?;
        Ok(()) 
    }

    fn create_tag_files(list_tags: Vec<String>) -> Result<(), std::io::Error>{
        let path = CurrentRepository::read()?;
        println!("LIST TAGS ---> {:?}", list_tags);
        for string_tag in list_tags{
            println!("STRING_TAG {}", string_tag);
            let tag: Vec<&str> = string_tag.split_whitespace().collect();
            let hash = tag[0];
            let filename = tag[1].trim_end_matches("^{}");

            let file_path = path.join(".rust_git").join(filename);
            let mut file = OpenOptions::new().create(true).write(true).append(true).open(&file_path)?;
            println!("filepath ---> {:?}", file_path.clone());
            file.write_all(hash.as_bytes())?;
        } 
        Ok(())
    }

    fn exclude_tag_ref(packets: Vec<String>) -> Result<(Vec<String>, Vec<String>), std::io::Error> {
        let mut to_remove: Vec<usize> = Vec::new();
        let mut list_tags: Vec<String> = Vec::new();
        
        for (index, refs) in packets.iter().enumerate() {
            if refs.contains("^{}") {
                // Crear folder de tag aquí para esa respectiva refs
            }
            if refs.contains("tags") {
                list_tags.push(refs.clone());
                to_remove.push(index);
            }
        }
        let remaining_packets: Vec<String> = packets
            .into_iter()
            .enumerate()
            .filter(|(index, _)| !to_remove.contains(index))
            .map(|(_, packet)| packet)
            .collect();
    
        Ok((list_tags, remaining_packets))
    }

    fn init_commits(list_refs: &Vec<String>, objects: &[(u8,Vec<u8>)], repo: &Path) -> Result<(), std::io::Error>  {
        let mut branchs: HashMap<String, String> = HashMap::new();

        let objects_processed = Self::process_folder(objects.to_vec());
        for obj in &objects_processed{
            println!("-->{:?}", obj);
        }
        let commits_created = Self::create_folders(objects_processed.clone(), repo);

        let delta_objects: Vec<(u8, Vec<u8>)> = objects.iter().filter(|&&(first, _)| first == 7).cloned().collect();
        for (number, inner_vec) in delta_objects {
            Self::process_delta_object(number, &inner_vec);
        }

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
                        let _ = Branch::create_new_branch_with_hash(repo, branch_name.trim_end_matches('\n'), commit);
                        println!("Commit: {}, Branch: {}", commit, branch_name);
                        branchs.insert(branch_name, commit.to_owned());
                }
            }
        }
        let _ = Self::write_commit_log(repo, branchs.clone(), &commits_created, objects_processed.clone());
        Checkout::update_cd(repo)?; //Esto recien me crea los files.txt en working directory si tiene en la tabla de commits lleno
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

    pub fn process_folder(objects: Vec<(u8,Vec<u8>)>) -> Vec<(u8, String)> {
        let mut objects_processed : Vec<(u8, String)> = Vec::new();
        for (number, inner_vec) in &objects {
            if *number == OBJ_REF_DELTA_CODE_NUMBER {
                
            }else if *number == TREE_CODE_NUMBER {
                objects_processed.push(Self::process_tree_object(*number, inner_vec));
            }else {
                objects_processed.push(Self::process_non_tree_object(*number, inner_vec));
            }
        }
        objects_processed
    }


    fn process_delta_object(number: u8, inner_vec: &Vec<u8>) -> (u8, String) {
        let base_object = &inner_vec[..20];
        let hash_base_object: String = base_object.iter().map(|b| format!("{:02x}", b)).collect();
        let decompres_data = &inner_vec[20..];
        
        let cat_file = VersionControlSystem::cat_file(&hash_base_object).unwrap();
        println!("CAT FILE: {}", cat_file);
        if Self::is_bit_set(inner_vec[20]) {
            // COPY
            println!("INNER VEC COPY: {}", inner_vec[20]);
        } else {
            // APPEND
            println!("INNER VEC APPEND: {:?}", &inner_vec[20..]);
            println!("DATA IN APPEND: {}", String::from_utf8_lossy(&inner_vec[..]));
            let _ = Self::mostrar_contenido_desde_posicion("/home/amoralejo/23C2-4Rust/test_delta/.rust_git/objects/35/180127f9eb0074cd684cacc8f82bf6fe74d7e3", 79);   
        }
        
        println!("BASE OBJECT: {}", hash_base_object);
        println!("ACA NUMBER: {}", number);
        println!("DATA ACA: {}", String::from_utf8_lossy(&inner_vec[20..]));
        (number, String::from_utf8_lossy(&inner_vec[20..]).to_string())
    }

        
    fn mostrar_contenido_desde_posicion(ruta: &str, posicion: u64) -> io::Result<()> {
        println!("ENTRA");
        // Abre el archivo en modo de lectura
        let mut archivo = File::open(ruta)?;
        println!("NO PASA");
        // Mueve el cursor a la posición especificada
        archivo.seek(SeekFrom::Start(posicion))?;

        // Lee el contenido desde la posición especificada
        let mut contenido = String::new();
        archivo.read_to_string(&mut contenido)?;

        // Imprime el contenido
        println!("IMPRIMIO: {}", contenido);

        Ok(())
    }


    pub fn create_folders(objects: Vec<(u8, String)>, repo: &Path) -> HashMap<String, CommitEntity>{
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
                TAG_CODE_NUMBER => Self::create_tag_folder(content, repo),
                OBJ_REF_DELTA_CODE_NUMBER => Self::create_ref_delta_folder(content, repo),
                _ => println!("Type not identify {}", index),
            }
        }
        commits_created
    }

    fn create_tag_folder(content: &str, repo: &Path){
        println!("PROCESAR UN TAG FOLDER");      
    }

    fn create_ref_delta_folder(content: &str, repo: &Path) {
        println!("CREATE FOLDER DELTAS");
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
        let hash_commit = Proxy::write_commit(repo, &commit_entity)?;

        Ok((hash_commit, commit_entity))
    }
    

    fn create_blob_folder(content: &String, repo: &Path){
        let _ = Proxy::write_blob(repo,content);
    }

    fn create_tree_folder(content: &str, repo: &Path) -> Result<String, std::io::Error> {
        Proxy::write_tree(repo, content)
    }
    

    fn write_commit_log( repo: &Path, branchs: HashMap<String, String>, commits_created:  &HashMap<String, CommitEntity>, _objects: Vec<(u8, String)>) -> Result<(), std::io::Error> {
        for (branch_name, hash_commit_branch) in &branchs{ // 2 nombre_rama, hash
            if commits_created.contains_key(hash_commit_branch) {
                let _ = Self::complete_commit_table(repo, &branch_name.to_string(), &hash_commit_branch.to_string(), commits_created);
            }
        }
        Ok(())
    }

    fn complete_commit_table(repo: &Path, branch_name: &String, hash_commit_branch: &String, commits_created:  &HashMap<String, CommitEntity>) -> Result<(), std::io::Error> {
        //branch_name, hash
        let logs_path = repo.join(".rust_git").join("logs").join(branch_name.trim_end_matches('\n'));
        let mut file = OpenOptions::new().create(true).write(true).append(true).open(logs_path)?;
        
        let content = CatFile::cat_file(hash_commit_branch, Init::get_object_path(repo)?)?;
        let id = Random::random();

        if content.contains("parent"){
            let part:Vec<&str> = content.split('\n').collect();
            let hash_parent = part[1].trim_start_matches("parent ");
            if let Some(commit_entity) = commits_created.get(hash_commit_branch){
                let date = Self::get_date(&commit_entity.author);
                let format_commit = format!("{}-{}-{}-{}-{}\n", id, commit_entity.parent_hash, hash_commit_branch, commit_entity.message, date);
                let _ = Self::complete_commit_table(repo, branch_name, &hash_parent.to_string(), commits_created);
                file.write_all(format_commit.as_bytes())?;
            }
        }else if let Some(commit_entity) = commits_created.get(hash_commit_branch){
                let date = Self::get_date(&commit_entity.author);
                let format_commit = format!("{}-{}-{}-{}-{}\n", id, commit_entity.parent_hash, hash_commit_branch, commit_entity.message, date);
                file.write_all(format_commit.as_bytes())?;
            
        }
        Ok(())
    }

    pub fn get_date(line: &str) -> &str {
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
        want_msgs
    }


    pub fn get_socket_response(socket: &mut TcpStream) -> Result<Vec<(u8,Vec<u8>)>,std::io::Error> {
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
        
        println!("CANTIDAD DE OBJETOS ---> {}\n", object_number);
        println!("ACA EL PACK: {:?}", String::from_utf8_lossy(pack));
        let mut position: usize = 12;
        let mut objects = Vec::new();
        for object in 0..object_number {
            let objet_type = Self::get_object_type(pack[position]);
            println!("TIPO OBJTO: {}", objet_type);
            while Self::is_bit_set(pack[position]) {
                position += 1;
            }
            position += 1;
            if objet_type == 7 {
                let mut base_object = pack[position..position+20].to_vec();
                let hex_representation: String = base_object.iter().map(|b| format!("{:02x}", b)).collect();
                println!("BASE OBJECT: {}", hex_representation);
                position += 20;
                if let Ok(data) = decompress_data(&pack[position..]) {
                    println!("TIPO OBJETO {}: {:?}, TAMAÑO OBJETO {}: {:?}, ARRANCA EN: {}, TERMINA EN: {}", object+1, objet_type, object+1, data.1, position, position+data.1 as usize);
                    println!("DATA OBJETO {}: {}", object+1, String::from_utf8_lossy(&data.0));
                    println!("DATA EN BYTES: {:?}", data); 
                    position += data.1 as usize;
                    base_object.extend_from_slice(&data.0); 
                    println!("BASE + DATA EN BYTES: {:?}", base_object);
                    objects.push((objet_type, base_object));   
                }
            }
            else {
                if let Ok(data) = decompress_data(&pack[position..]) {
                    println!("TIPO OBJETO {}: {:?}, TAMAÑO OBJETO {}: {:?}, ARRANCA EN: {}, TERMINA EN: {}", object+1, objet_type, object+1, data.1, position, position+data.1 as usize);
                    println!("DATA OBJETO {}: {}", object+1, String::from_utf8_lossy(&data.0));
                    position += data.1 as usize; 
                    objects.push((objet_type, data.0))   
                }    
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