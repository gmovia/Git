use std::{fs, io};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::net::{TcpStream, Shutdown};

use crate::packfile;
use crate::packfile::packfile::{to_pkt_line, process_line};
use crate::server::encoder::Encoder;


// reader and writer del mismo socket? por mientras asigno socket para no confiones 
pub fn start_handler_upload(stream: &mut TcpStream, path: &PathBuf) -> Result<String, std::io::Error> {
    let first_response = handler_upload_pack(path)?;

    send_response(first_response, stream)?;
    
    let query = receive_wants_and_have_message(stream)?;
    let packfile_result = Encoder::init_encoder((&path).to_path_buf(), query);

    match packfile_result {
        Ok( packfile) => {
            stream.write(&packfile)?;
            println!("--------------------------------ENVIADO\n");
        },
        Err(e) => {
            println!("Error al inicializar el packfile: {:?}", e);
        }
    } 
    Ok("0000".to_string())
}



pub fn receive_wants_and_have_message(reader: &mut TcpStream) -> Result<(Vec<String>,Vec<String>), io::Error> {
    let mut query = vec![];
    loop {
        let msg_received = process_line(reader)?;
        println!("Mensaje recibido ------> {:?}", msg_received);
        if msg_received == "done\n" {
            break;
        }
        query.push(msg_received.clone());
    }

    let messages_type = process_messages(query)?;
    Ok(messages_type)
}

pub fn process_messages(messages: Vec<String>) -> Result<(Vec<String>,Vec<String>), io::Error> {
    let mut wants: Vec<String> = Vec::new();
    let mut haves: Vec<String> = Vec::new();
    for message in messages {
        if message.contains("want") {
            wants.push(message);
        }
        else {
            haves.push(message)
        }
    }
    Ok((wants,haves))
}


pub fn handler_upload_pack(path: &PathBuf) -> Result<Vec<String>, std::io::Error> {
    let logs_path = path.join(".rust_git").join("logs");
    let log_entries = get_log_entries(&logs_path)?;
    Ok(log_entries)
}

fn get_log_entries(logs_path: &Path) -> Result<Vec<String>, std::io::Error>{
    let mut log_entries = Vec::new();
    
    let entries = fs::read_dir(logs_path)?;
    for entry in entries {
        let log_file = entry?;
        let file = fs::File::open(log_file.path())?;
        
        let mut reader = BufReader::new(file);
        let mut last_line = String::new();

        for line in reader.by_ref().lines() {
            if let Ok(line) = line {
                last_line = line.clone(); // Clonamos la línea si la necesitas posteriormente
                let last_commit = line[2..42].to_string(); // Clonamos la porción de la línea
                let log_file_name = log_file.file_name().to_string_lossy().to_string();
                let format = format!("{} refs/head/{}", last_commit, log_file_name);
                log_entries.push(format);
 
            }
        }
        if let Some(hash) = parse_log_line(&last_line) {
            let filename = log_file.file_name().to_string_lossy().to_string();
            log_entries.push(format!("{} refs/heads/{}\n", hash, filename));
        }
    }

    Ok(log_entries)
}


fn parse_log_line(line: &str) -> Option<String> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    match parts.get(0) {
        Some(part) => {
            let part = part.replace("--m", "");
            let hash_parts: Vec<&str> = part.splitn(2, '-').collect(); 

            match hash_parts.as_slice() {
                [_, hash] if hash.len() == 40 => {
                    let hash = hash_parts[0..2].join("");
                    Some(hash)
                }
                _ => None,
            }
        }
        None => None,
    }
}


fn send_response(response: Vec<String>, writer: &mut TcpStream) -> Result<(), std::io::Error> {
    print!("MI RESPONSE ES {:?}", response);
    for resp in response {
        if resp.contains("\n"){
            for line in resp.lines(){
                let line_without_newline = line.trim_end().trim_end();
                let msg_response = format!("{}\n", line_without_newline);                
                let pkt_response = to_pkt_line(&msg_response);
                writer.write(pkt_response.as_bytes())?;
            }
        } else {
                writer.write(to_pkt_line(resp.as_str()).as_bytes())?;
            
        }
    }
        
    writer.write("0000".as_bytes())?;
    //writer.flush()?;
    Ok(())
}