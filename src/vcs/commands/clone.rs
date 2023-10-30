
use std::{fs::{OpenOptions, self, File}, self, io::{Write, self, Read}, collections::HashMap, net::TcpStream, str::from_utf8, path::{Path, PathBuf}};
use chrono::{DateTime, Local};

use crate::{vcs::version_control_system::VersionControlSystem, utils::random::random::Random, handlers::{cat_file::handler_cat_file, branch::handler_branch}};
use super::{init::Init, branch::BranchOptions, hash_object::WriteOption};
use crate::packfile::{decompress_data, to_pkt_line};

pub struct Clone;

impl Clone{
    pub fn clone(stream: &mut TcpStream) -> Result<(), std::io::Error> {
        let init_path = Path::new("/home/amoralejo/TEST4");
        let mut vcs = VersionControlSystem::init(init_path, Vec::new());
        Self::receive_pack(stream, &mut vcs)?;
        Ok(())
    }

    fn read_packet(stream: &mut TcpStream, len: usize) -> String {
        let mut packet_buf = vec![0; len - 4];
        let _ = stream.read_exact(&mut packet_buf);
        String::from_utf8_lossy(&packet_buf).to_string()
    }

    pub fn request_branch(list_refs: &Vec<String>, vcs: &VersionControlSystem, objects: Vec<(u8,Vec<u8>)>) -> Result<(),std::io::Error> {
        for item in list_refs {
            if item.contains("HEAD") {
                continue;
            }

            let parts: Vec<&str> = item.splitn(2, ' ').collect(); // Divide el elemento en dos partes.
            if parts.len() == 2 {
                let commit = parts[0];
                let ref_part = parts[1];
    
                // Realiza una acción con la parte que sigue después de "refs/". Por ejemplo:
                if ref_part.starts_with("refs/") {
                    let branch_name = ref_part.trim_start_matches("refs/heads/");
                    let _ = vcs.branch(BranchOptions::NewBranch(branch_name.trim_end_matches('\n')));
                    println!("Commit: {}, Branch: {}", commit, branch_name);
                    // Realiza aquí la acción que desees con `branch_name`.
                    
                    let mut tree_hash = String::new();
                    let mut files = Vec::new();
                    let mut file_hashes: Vec<(String,String)> = Vec::new();
                    for object in &objects {
                        println!("for de object: {:?}-{:?}",object.0,String::from_utf8_lossy(&object.1));
                        match object.0 {
                            1 => {
                                Self::write_commit_log_file(vcs, commit, branch_name)?;
                                tree_hash = Self::create_commit_folder(vcs, &object.1, commit)?
                                },
                            2 => files = Self::create_tree_folder(vcs, object.1.to_owned(), &tree_hash)?,
                            3 => {
                                let file_name = files.remove(0);
                                let file_name_hash = Self::create_blob_folder(vcs, object.1.to_owned(), file_name)?;
                                Self::add_hash_to_tree(vcs, file_name_hash, &tree_hash)?
                                },
                            _ => Err(io::Error::new(io::ErrorKind::NotFound, "Objeto desconocido"))?,
                        };
                    }
                }
            }
        }
        Ok(())
    }

    // Falta ver que el archivo de logs tiene varios commits, capaz sale por el lado del parent?
    fn write_commit_log_file(vcs: &VersionControlSystem, commit: &str, branch_name: &str) -> Result<(),std::io::Error>{
        let logs_path = vcs.path.join(".rust_git").join("logs").join(branch_name); 
        
        let current_time: DateTime<Local> = Local::now();
        let format_commit = format!("{}-{}-{}-{}", Random::random(), commit, "clone", current_time);
        
        let mut file = OpenOptions::new().write(true).create(true).open(logs_path).expect("No se pudo abrir el archivo");
        file.write(format_commit.as_bytes())?;

        Ok(())
    }

    fn add_hash_to_tree(vcs: &VersionControlSystem, file_hash: (String,String), tree_hash: &str) -> Result<(),std::io::Error> {
        println!("treee hash: {}", tree_hash);
        let path = vcs.path.join(".rust_git").join("objects").join(&tree_hash[0..2]).join(&tree_hash[3..]);
        let mut file = OpenOptions::new().write(true).create(true).append(true).open(path).expect("No se pudo abrir el archivo");
        let format = format!("{}-{}", file_hash.0, file_hash.1);
        file.write(format.as_bytes())?;
        file.write("\n".as_bytes())?;
        Ok(())
    }

    fn create_commit_folder(vcs: &VersionControlSystem, commit: &Vec<u8>, commit_folder: &str) -> Result<String,std::io::Error> {
        let object_path = vcs.path.join(".rust_git").join("objects").join(&commit_folder[0..2]);
        fs::create_dir_all(&object_path)?;
        let file_name = &commit_folder[3..];
        let mut file = File::create(&object_path.join(file_name))?;
        file.write(commit)?;
        Ok(String::from_utf8_lossy(&commit[5..45]).to_string())       
    }

    fn create_tree_folder(vcs: &VersionControlSystem, tree_hash: Vec<u8>, tree_hash_folder: &str) -> Result<Vec<(String,String)>,std::io::Error> {
        let object_path = vcs.path.join(".rust_git").join("objects").join(&tree_hash_folder[0..2]);
        fs::create_dir_all(&object_path)?;
        File::create(&object_path.join(&tree_hash_folder[3..]))?;
        let files_names = Self::get_file_names(&tree_hash)?;
        Ok(files_names)
    }

    fn get_file_names(hash: &[u8]) -> Result<Vec<(String,String)>,std::io::Error> {
        let mut files_hash = Vec::new();
        let files: Vec<String> = String::from_utf8_lossy(hash).split_whitespace().map(|s| s.to_owned()).collect();
        println!("{:?}", files);
        for file in files {
            if !file.contains("\0") {
                continue;
            }
            let file_name: Vec<&str> = file.split("\0").collect();
            files_hash.push((file_name[0].to_owned(),file_name[1].to_owned()));
        }
        Ok(files_hash)
    }


    fn create_blob_folder(vcs: &VersionControlSystem, commit: Vec<u8>,  files: (String,String)) -> Result<(String,String),std::io::Error> {
        // con files.1 (es el hash del blob) deberia poder obtener el nombre de las carpetas para guardar el contenido de los archivos
        println!("blob folder: {:?}",files.0);
        println!("blob folder: {:?}",String::from_utf8_lossy(&commit));
        let file_hash = Self::create_file(vcs, files.0, &commit)?;       
        
        let object_path = vcs.path.join(".rust_git").join("objects").join(&file_hash.1[0..2]);
        fs::create_dir_all(&object_path)?;
        let mut file = File::create(&object_path.join(&file_hash.1[3..]))?;
        file.write(String::from_utf8_lossy(&commit).as_bytes())?;        
        
        Ok(file_hash)
    }

    fn create_file(vcs: &VersionControlSystem, file_name: String, file_content: &Vec<u8>) -> Result<(String,String), std::io::Error> {
        let file_path = vcs.path.join(&file_name);
        fs::create_dir_all(&vcs.path)?;
        let mut file = File::create(file_path)?;
        file.write(String::from_utf8_lossy(&file_content).as_bytes())?;
        let hash = vcs.hash_object(vcs.path.join(&file_name).as_path(), WriteOption::NoWrite)?;
        Ok((file_name,hash))
    }

    pub fn receive_pack(socket: &mut TcpStream, vcs: &VersionControlSystem) -> Result<(), std::io::Error> {
        let mut packets = Vec::new();
        loop {
            let mut len_buf = [0; 4]; 
            if socket.read_exact(&mut len_buf).is_ok() {
                let len_str = from_utf8(&len_buf).unwrap();
                let len = usize::from_str_radix(len_str, 16).unwrap();
                if len == 0 {
                    break;
                }
                let packet = Self::read_packet(socket, len);
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
        
        Self::request_branch(&packets, vcs, objects)?;
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
        println!("VERSION: {:?} - {:?}", version, Self::parse_number(version).unwrap());
        let object_number = Self::parse_number(&pack[8..12])?;
        println!("CANTIDAD DE OBJETOS: {}", object_number);
        
        let mut position: u64 = 12;
        let mut objects = Vec::new();
        for object in 0..object_number {
            let mut position_usize: usize = position as usize;
            let objet_type = Self::get_object_type(pack[position_usize]);
            if Self::is_bit_set(pack[position_usize]) {
                position = position + 2 as u64;
                position_usize = position as usize;
            }
            else {
                position = position + 1 as u64;
                position_usize = position as usize;
                
            }
            if let Ok(data) = decompress_data(&pack[position_usize..]) {
                println!("TIPO OBJETO {}: {:?}, TAMAÑO OBJETO {}: {:?}", object+1, objet_type, object+1, data.1);
                println!("DATA OBJETO {}: {}", object+1, String::from_utf8_lossy(&data.0));
                position = position + data.1 as u64; 
                objects.push((objet_type, data.0))   
            }
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