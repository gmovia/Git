
use std::{fs::{OpenOptions, self, File}, self, io::{Write, self, Read}, collections::HashMap, net::TcpStream, str::from_utf8, path::{Path, PathBuf}};
use chrono::{DateTime, Local};

use crate::{vcs::version_control_system::VersionControlSystem, utils::random::random::Random, handlers::{cat_file::handler_cat_file, branch::handler_branch}};
use super::{init::Init, branch::BranchOptions};
use crate::packfile::{decompress_data, to_pkt_line};

pub struct Clone;

impl Clone{
    pub fn clone(stream: &mut TcpStream) -> Result<(), std::io::Error> {
        let init_path = Path::new("/home/amoralejo/TEST2");
        let mut vcs = VersionControlSystem::init(init_path, Vec::new());
        Self::receive_pack(stream, &mut vcs)?;
        Ok(())
    }

    fn read_packet(stream: &mut TcpStream, len: usize) -> String {
        let mut packet_buf = vec![0; len - 4];
        let _ = stream.read_exact(&mut packet_buf);
        String::from_utf8_lossy(&packet_buf).to_string()
    }

    pub fn request_branch(list_refs: &Vec<String>, vcs: &VersionControlSystem) {
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
                    if let Err(e) = Self::wite_commit_log_file(vcs.path.clone(), commit, branch_name) {
                        println!("{}",e);
                    }
                }
            }
        }
    }

    fn wite_commit_log_file(path: PathBuf, commit: &str, branch_name: &str) -> Result<(),std::io::Error>{
        let logs_path = path.join(".rust_git").join("logs").join(branch_name); 
        println!("path: {:?}", logs_path);
        
        let current_time: DateTime<Local> = Local::now();
        let format_commit = format!("{}-{}-{}-{}", Random::random(), commit, "clone", current_time);
        
        let mut file = OpenOptions::new().write(true).create(true) .open(logs_path).expect("No se pudo abrir el archivo");
        file.write(format_commit.as_bytes())?;
        Ok(())
    }

    pub fn receive_pack(socket: &mut TcpStream, vcs: &VersionControlSystem)-> Result<(), std::io::Error> {
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
        Self::request_branch(&packets, vcs);

        for want in Self::get_want_msgs(packets) {
            socket.write(want.as_bytes())?;
        }
        
        Self::send_done_msg(socket)?;
        let commit_hash_decompress = Self::print_socket_response(socket)?;
        println!("commit hash decode: {:?}", String::from_utf8_lossy(&commit_hash_decompress));
        let blobs_hash = Self::get_decompress_tree_hash_bytes(vcs, commit_hash_decompress)?;
        println!("tree hash decode: {}", String::from_utf8_lossy(&blobs_hash));
        //manejo el procesamiento de mi query de wants.
        Ok(()) 
    }
    
    /// Recibe el hash del commit
    /// Te devuelve la tira de bytes del hash del tree descomprimido
    fn get_decompress_tree_hash_bytes(vcs: &VersionControlSystem, commit_hash: Vec<u8>) -> Result<Vec<u8>,std::io::Error> {
        let format_hash = format!("{}", String::from_utf8_lossy(&commit_hash[5..45]));
        let blobs_hash = vcs.cat_file_bytes(&format_hash, ".git")?;
        let dec_hash = decompress_data(&blobs_hash)?;
        Ok(dec_hash) 
    }

    fn print_socket_response(socket: &mut TcpStream) -> Result<Vec<u8>,std::io::Error> {
        let mut buffer = Vec::new();
            match socket.read_to_end(&mut buffer) {
                Ok(_) => {
                    match decompress_data(&buffer[22..]) {
                        Ok(decompressed_data) => {
                            let text = String::from_utf8_lossy(&decompressed_data);
                            //println!("Datos descomprimidos: {}", text);
                            return Ok(decompressed_data);
                        },
                        Err(e) => {
                            eprintln!("Error al descomprimir los datos: {}", e);
                            return Err(e)
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to receive data: {}\n", e);
                    return Err(e)
                }
            } 
    }

    fn get_want_msgs(commits_list: Vec<String>) -> Vec<String> {
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