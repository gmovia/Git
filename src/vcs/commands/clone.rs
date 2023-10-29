
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
                    if let Err(e) = Self::write_commit_log_file(vcs, commit, branch_name) {
                        println!("{}",e);
                    }
                }
            }
        }
    }

    // Falta ver que el archivo de logs tiene varios commits, capaz sale por el lado del parent?
    fn write_commit_log_file(vcs: &VersionControlSystem, commit: &str, branch_name: &str) -> Result<(),std::io::Error>{
        let logs_path = vcs.path.join(".rust_git").join("logs").join(branch_name); 
        
        let current_time: DateTime<Local> = Local::now();
        let format_commit = format!("{}-{}-{}-{}", Random::random(), commit, "clone", current_time);
        
        let mut file = OpenOptions::new().write(true).create(true).open(logs_path).expect("No se pudo abrir el archivo");
        file.write(format_commit.as_bytes())?;

        Self::create_object_commit_folder(vcs, commit)?;
        Ok(())
    }

    fn create_object_commit_folder(vcs: &VersionControlSystem, commit: &str) -> Result<(),std::io::Error> {
        let object_path = vcs.path.join(".rust_git").join("objects").join(&commit[0..2]);
        fs::create_dir_all(&object_path)?;
        let file_name = &commit[3..];
        let mut file = File::create(&object_path.join(file_name))?;
        // como hacemos? aca el git original guarda el texto descomprimido pero sin pasarlo a string
        // Podemos parsear para que quede como lo tenemos en nuestro TP
        let commit_hash_only = format!("{}", &String::from_utf8_lossy(&Self::get_decompress_hash_bytes(vcs, commit.into())?)[16..56]);
        let format = format!("commit-{}", commit_hash_only); 
        println!("esto imprime: {}", format);
        file.write(format.as_bytes())?;
        Self::create_tree_folder(vcs, &commit_hash_only)?;
        Ok(())
    }

    fn create_tree_folder(vcs: &VersionControlSystem, tree_hash: &str) -> Result<(),std::io::Error> {
        let object_path = vcs.path.join(".rust_git").join("objects").join(&tree_hash[0..2]);
        fs::create_dir_all(&object_path)?;
        let mut file = File::create(&object_path.join(&tree_hash[3..]))?;
        let text_hash_file = format!("{}", &String::from_utf8_lossy(&Self::get_decompress_hash_bytes(vcs, tree_hash.into())?));
        let files_names = Self::get_file_names(&text_hash_file)?;
        for file_name in files_names {
            let format = format!("{}/{}-{}", vcs.path.display(),file_name.0,file_name.1);
            file.write(format.as_bytes())?;
            file.write("\n".as_bytes())?;
        }     
        Ok(())
    }

    fn get_file_names(hash: &str) -> Result<Vec<(String,&str)>,std::io::Error> {
        let mut files_hash = Vec::new();
        let files: Vec<&str> = hash.split_whitespace().collect();

        for file in files {
            if !file.contains("txt") {
                continue;
            }
            let file_name: Vec<&str> = file.split("txt").collect();
            files_hash.push((file_name[0].to_owned()+"txt",file_name[1]));
        }
        Ok(files_hash)
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
        let blobs_hash = Self::get_decompress_hash_bytes(vcs, commit_hash_decompress[5..45].to_vec())?;
        println!("tree hash decode: {}", String::from_utf8_lossy(&blobs_hash));
        //manejo el procesamiento de mi query de wants.
        Ok(()) 
    }
    
    /// Recibe el hash del commit
    /// Te devuelve la tira de bytes del hash descomprimido
    fn get_decompress_hash_bytes(vcs: &VersionControlSystem, commit_hash: Vec<u8>) -> Result<Vec<u8>,std::io::Error> {
        let format_hash = format!("{}", String::from_utf8_lossy(&commit_hash));
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
                            
                            Self::manage_pack(&buffer[8..]);
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


    // ESTA FUNCION ME PARA QUE ESTA PARA ATRAS - LA DEJO POR LAS DUDAS PERO SEGURO HAY QUE SACARLA/MODIFICARLA
    fn calculate_length(bytes: &[u8]) -> u32 {
        let mut length = 0;
        let mut shift = 0;
    
        for (i, &byte) in bytes.iter().enumerate() {
            if i == 0 {
                shift = 3;
            }
    
            length |= ((byte & 0b01111111) as u32) << shift;
            shift += 7;
    
            if (byte & 0b10000000) == 0 {
                break;
            }
        }
    
        length
    }

    fn manage_pack(pack: &[u8])  -> Result<(),std::io::Error> {
        let signature_pack_msg = &pack[0..4];
        println!("SIGNATURE: {:?} - {:?}", signature_pack_msg, String::from_utf8_lossy(signature_pack_msg));
        let version = &pack[4..8];
        println!("VERSION: {:?} - {:?}", version, Self::parse_number(version).unwrap());
        let objetos = &pack[8..12];
        println!("CANTIDAD DE OBJETOS: {:?} - {:?}", objetos, Self::parse_number(objetos).unwrap());
        
        // HASTA ACA ESTAMOS BIEN. EL TIPO DE OBJETO ES UN COMMIT POR LO QUE ENTIENDO DEBE ESTAR BIEN 
        // EL TAMAÑO SE COMPLICO :(
        let start_byte = 12;
        let first_objet_type = Self::get_object_type(pack[start_byte]);
        
        //let lenght = Self::calculate_length(&pack[12..(start_byte+bytes_necesarios as usize)]);
        println!("TIPO PRIMER OBJETO: {:?}, TAMAÑO PRIMER OBJETO: {:?}", first_objet_type, 0);
        
        let bits_to_calc_len = (first_objet_type-1)*7+4;
        println!("bits to calc len: {:?}-bits", bits_to_calc_len);

        let data_first_object = &pack[13..26]; 
        println!("Resultado: {:?}", decompress_data(data_first_object));
    
        let second_object_byte = &pack[26] >> 5;
        println!("second_objet_type: {:?}, {:?}", second_object_byte, String::from_utf8_lossy(&second_object_byte.to_be_bytes()));
        let n = 26; // es esto n?
        let second_object_len = (n-1)*7+4;
        println!("second_objet_len: {:?}-bits", second_object_len);
        let data_second_object = &pack[13..26]; 
        println!("Resultado 2: {:?}", decompress_data(data_second_object));
        Ok(())
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