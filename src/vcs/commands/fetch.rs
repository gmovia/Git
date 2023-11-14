use std::{net::TcpStream, path::PathBuf, io::{Read, Write, self, BufRead}, str::from_utf8, collections::HashMap, fs::File};

use crate::packfile::packfile::{read_packet, send_done_msg, to_pkt_line, decompress_data};

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
        let message_to_send = Self::packet_manager(last_commit_per_branch, repo)?;
        println!("Message to send: {:?}", message_to_send);

        Self::send_messages(socket, message_to_send)?;

        let objects = Self::get_socket_response(socket)?;
        println!("OBJETOS: {:?}", objects);
        //Self::init_commits(&packets , &objects, repo)?;
        Ok(()) 
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

    fn packet_manager(last_branch_commit_recieve: Vec<(String,String)>, repo: PathBuf) -> Result<(Vec<String>,Vec<String>), std::io::Error>{
        let mut want_list: Vec<String> = Vec::new();
        let mut have_list: Vec<String> = Vec::new();
        for packet in &last_branch_commit_recieve {
        match File::open(&repo.join(".rust_git").join("logs").join(packet.0.to_string())) { 
            Ok(file) => {
                    let file = File::open(&repo.join(".rust_git").join("logs").join(packet.0.to_string()))?;
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