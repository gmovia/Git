
use std::{fs::{OpenOptions, self, File}, self, io::{Write, self, Read}, net::TcpStream, str::from_utf8, path::{Path, PathBuf}};
use chrono::{DateTime, Local};
use rand::Rng;

use crate::{vcs::version_control_system::VersionControlSystem, utils::random::random::Random, packfile::packfile::process_line, handlers::branch};
use super::{branch::BranchOptions, hash_object::WriteOption};
use crate::packfile::packfile::{decompress_data, to_pkt_line, read_packet};

pub struct Clone;

impl Clone{
    pub fn clone(stream: &mut TcpStream) -> Result<(), std::io::Error> {
        Self::receive_pack(stream)?;
        Ok(())
    }

    pub fn request_branch(list_refs: &Vec<String>, objects: Vec<(u8,Vec<u8>)>) -> Result<(),std::io::Error> {
        
        for item in list_refs {
            if item.contains("HEAD") || !item.contains("master") {
                continue;
            }
            let parts: Vec<&str> = item.splitn(2, ' ').collect(); // Divide el elemento en dos partes.
            if parts.len() == 2 {
                let commit = parts[0];
                let ref_part = parts[1];
    
                // Realiza una acción con la parte que sigue después de "refs/". Por ejemplo:
                if ref_part.starts_with("refs/") {
                    let branch_name = ref_part.trim_start_matches("refs/heads/");
                    let _ = VersionControlSystem::branch(BranchOptions::NewBranch(branch_name.trim_end_matches('\n')));
                    println!("Commit: {}, Branch: {}", commit, branch_name);

                    println!("ANTES DE WRITE COMMIT");            
                    if let Err(e) = Self::write_commit_log_file(commit, branch_name) {
                        println!("ERROR: {}",e);
                        return Err(std::io::Error::new(io::ErrorKind::NotFound, e))
                    }            

                    let mut tree_hash = String::new();
                    let mut files = Vec::new();
                    for object in &objects {
                        match object.0 {
                            1 => {
                                tree_hash = Self::create_commit_folder(&object.1, commit)?
                            },
                            2 => files = Self::create_tree_folder(object.1.to_owned(), &tree_hash)?,
                            3 => {
                                if !files.is_empty() {
                                    let file_name = files.remove(0);
                                    let file_name_hash = Self::create_blob_folder(object.1.to_owned(), file_name)?;
                                    Self::add_hash_to_tree(file_name_hash, &tree_hash)?;
                                } else {
                                    break;
                                }
                            },
                            _ => Err(io::Error::new(io::ErrorKind::NotFound, "Unknown object"))?,
                        };
                    }
                }
            }
        }
        Ok(())
    }

    // Falta ver que el archivo de logs tiene varios commits, capaz sale por el lado del parent?
    fn write_commit_log_file(commit: &str, branch_name: &str) -> Result<(),std::io::Error>{
        let current = VersionControlSystem::read_current_repository()?;

        let logs_path = current.join(".rust_git").join("logs").join(format!("{}",branch_name)); 
        println!("CURRENT PATH CLONE: {:?}", logs_path);
        let current_time: DateTime<Local> = Local::now();
        let mut rng = rand::thread_rng();
        let format_commit = format!("{}-{}-{}-{}\n", rng.gen_range(1..9), commit, "clone", current_time);
        let mut file = OpenOptions::new().write(true).create(true).open(logs_path).expect("No se pudo abrir el archivo");
        file.write(format_commit.as_bytes())?;

        Ok(())
    }

    fn add_hash_to_tree(file_hash: (String,String), tree_hash: &str) -> Result<(),std::io::Error> {
        let current = VersionControlSystem::read_current_repository()?;
        let path = current.join(".rust_git").join("objects").join(&tree_hash[0..2]).join(&tree_hash[2..]);
        let mut file = OpenOptions::new().write(true).create(true).append(true).open(path).expect("No se pudo abrir el archivo");
        if let Some(file_name) = Path::new(&file_hash.0).file_name() {
            let format = format!("{}-{}", current.to_string_lossy().to_string()+"/"+&file_name.to_string_lossy().to_string(), file_hash.1);
            file.write(format.as_bytes())?;
            file.write("\n".as_bytes())?;
            Ok(())
        }
        else {
            Err(std::io::Error::new(io::ErrorKind::InvalidData, "Can not add hash treee"))
        }
        
    }

    fn create_commit_folder(commit: &Vec<u8>, commit_folder: &str) -> Result<String,std::io::Error> {
        let current = VersionControlSystem::read_current_repository()?;
        let object_path = current.join(".rust_git").join("objects").join(&commit_folder[0..2]);
        fs::create_dir_all(&object_path)?;
        let file_name = &commit_folder[2..];
        let mut file = File::create(&object_path.join(file_name))?;
        file.write(commit)?;
        Ok(String::from_utf8_lossy(&commit[5..45]).to_string())       
    }

    fn create_tree_folder(tree_hash: Vec<u8>, tree_hash_folder: &str) -> Result<Vec<(String,String)>,std::io::Error> {
        let current = VersionControlSystem::read_current_repository()?;
        let object_path = current.join(".rust_git").join("objects").join(&tree_hash_folder[0..2]);
        fs::create_dir_all(&object_path)?;
        let file = File::create(&object_path.join(&tree_hash_folder[2..]))?;
        let files_names = Self::get_file_names(&tree_hash)?;
        Ok(files_names)
    }

    fn get_file_names(hash: &[u8]) -> Result<Vec<(String,String)>,std::io::Error> {
        let mut files_hash = Vec::new();
        let files: Vec<String> = String::from_utf8_lossy(hash).split_whitespace().map(|s| s.to_owned()).collect();
        for file in files {
            // EN DAEMON TIENE QUE TENER EL \0 PARA DIFERENCIAR
            //if !file.contains("\0") {
            //    continue;
            //}
            //let file_name: Vec<&str> = file.split("\0").collect();
            let file_name: Vec<&str> = file.split("-").collect();
            files_hash.push((file_name[0].to_owned(),file_name[1].to_owned()));
        }
        Ok(files_hash)
    }


    fn create_blob_folder(commit: Vec<u8>,  files: (String,String)) -> Result<(String,String),std::io::Error> {
        let file_hash = Self::create_file(files.0, &commit)?;       
        let current = VersionControlSystem::read_current_repository()?;
        let object_path = current.join(".rust_git").join("objects").join(&file_hash.1[0..2]);
        fs::create_dir_all(&object_path)?;
        let mut file = File::create(&object_path.join(&file_hash.1[2..]))?;
        file.write(String::from_utf8_lossy(&commit).as_bytes())?;        
        Ok(file_hash)
    }

    fn create_file(file_name: String, file_content: &Vec<u8>) -> Result<(String,String), std::io::Error> {
        let current = VersionControlSystem::read_current_repository()?;
        fs::create_dir_all(&current)?;
        let mut hash = String::new();
        let file_path = Path::new(&file_name);
        if let Some(file_name) = file_path.file_name() {
            let mut file = File::create(current.join(&file_name))?;
            file.write(String::from_utf8_lossy(&file_content).as_bytes())?;
            hash = VersionControlSystem::hash_object(current.join(&file_name).as_path(), WriteOption::NoWrite)?;    
        } else {
            std::io::Error::new(io::ErrorKind::InvalidData, "Can not create the file.");
        }
        Ok((file_name,hash))
    }

    pub fn receive_pack(socket: &mut TcpStream) -> Result<(), std::io::Error> {
        let mut packets = Vec::new();
        print!("Entro a receive packs ---------------\n");
        loop {
            let mut len_buf = [0; 4]; 
            if socket.read_exact(&mut len_buf).is_ok() {
                let len_str = from_utf8(&len_buf).unwrap();
                let len = usize::from_str_radix(len_str, 16).unwrap();
                if len == 0 {
                    break;
                }
                println!("ACA 1 PACKETTT ---> {:?} \n", len);

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

        Self::send_done_msg(socket)?;
        let objects = Self::get_socket_response(socket)?;
        println!("ANTES DE ENTRAR A REQUEST");
        Self::request_branch(&packets, objects)?;
        Ok(()) 
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
        for (number, inner_vec) in &objects {
            println!("({}, {:?})", number, String::from_utf8_lossy(inner_vec));
        }
        
        Ok(objects)
    }

    fn is_bit_set(byte: u8) -> bool {
       let mask = 0b10000000;
       (byte & mask) == mask
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

    fn send_done_msg(socket: &mut TcpStream) -> Result<(), std::io::Error> {
        let msg_done = "0000";
        socket.write(msg_done.as_bytes())?;
    
        let msg_done2 = "0009done\n";
        socket.write(msg_done2.as_bytes())?;
        Ok(())
    }
}