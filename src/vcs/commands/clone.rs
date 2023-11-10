use std::{net::TcpStream, io::{Read, Write, self}, str::from_utf8};

use crate::{packfile::packfile::{read_packet, to_pkt_line, send_done_msg, decompress_data}, vcs::{version_control_system::VersionControlSystem, commands::branch::BranchOptions}};

pub struct Clone;

impl Clone{
    pub fn git_clone(stream: &mut TcpStream) -> Result<(),std::io::Error> {
        Self::receive_pack(stream)?;
        Ok(())
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

        send_done_msg(socket)?;
        let objects = Self::get_socket_response(socket)?;
        //aca recien recibo tipo de objeto y bytes!!!! 
        Self::init_commits(&packets , &objects)?;
        //println!("ANTES DE ENTRAR A REQUEST");
        //Self::request_branch(&packets, objects)?; //CREACION DE MIS FILES INTO CURRENT DIR
        Ok(()) 
    }


    fn init_commits(list_refs: &Vec<String>, objects: &Vec<(u8,Vec<u8>)>) -> Result<(), std::io::Error>  {
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
                    let _ = VersionControlSystem::branch(BranchOptions::NewBranch(branch_name.trim_end_matches('\n')));
                    println!("Commit: {}, Branch: {}", commit, branch_name);

                    let objects_prosessed = Self::process_folder(objects.to_vec());

                    //println!("SALI DE PROCESAR EN INIT COMMIT \n {:?}", objects_prosessed);
                    for obj in &objects_prosessed{
                        println!("-->{:?}", obj);
                    }
                }
            }
        }
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
               print!("CONTENNNNTTTT {:?}\n", content);
                if content.contains(&10064.to_string()) {
                    let mut reader = inner_vec.as_slice();
                    println!("ENTRE AL IF DEL TREEE \n");
                    if let Ok(entries) = Self::read_tree_sha1(&mut reader) {
                        for (mode, name, sha1) in entries {
                            let hex_string: String = sha1.iter().map(|byte| format!("{:02x}", byte)).collect();
                            println!("Mode: {:o}, Name: {}, SHA-1: {}", mode, name, hex_string);  
                            objects_processed.push((*number, format!("{}-{}", name, hex_string))); 
                        }
                    } else {
                        eprintln!("Error al decodificar el objeto tree");
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

/*     fn create_folder(){

    } */

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

                //print!();
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

    fn read_tree_entry<R: Read>(reader: &mut R) -> io::Result<(u32, String, Vec<u8>)> {
        let mut mode_bytes = [0; 6];
        reader.read_exact(&mut mode_bytes)?;
    
        let binding = String::from_utf8_lossy(&mode_bytes[..]);
        let mode_str = binding.trim();
        let mode = u32::from_str_radix(mode_str, 8).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    
        let name = Self::read_cstring(reader)?;
    
        let mut sha1 = vec![0; 20];
        reader.read_exact(&mut sha1)?;
    
        Ok((mode, name, sha1))
    }
    

    fn parse_number(bytes: &[u8]) -> Result<u8, std::io::Error> {
        let texto: String = bytes.iter().map(|&b| b.to_string()).collect();
        match texto.parse() {
            Ok(numero) => return Ok(numero),
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Can not parse number")),
        }
    }

    fn read_tree_sha1<R: Read>(reader: &mut R) -> io::Result<Vec<(u32, String, Vec<u8>)>> {
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