use std::{fs, io};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::net::TcpStream;

use crate::packfile;
use crate::packfile::packfile::{to_pkt_line, process_line};
use crate::server::encoder::Encoder;


// reader and writer del mismo socket? por mientras asigno socket para no confiones 
pub fn start_handler_upload(message: String, stream: &mut TcpStream, path: &PathBuf) -> Result<String, std::io::Error> {
    let first_response = handler_upload_pack(path)?;

    send_response(&first_response, stream)?;

    //cliente mandado los wants message

    //puede haber un reader y un writer sin problemas? 
    //porque en esta parte seria llamado reader para responder la solicitud

    //let pack_to_send = receive_wants_and_have_message(stream)?;

/*     println!("SALI DEL receive wants \n");

    for commit in &pack_to_send {   
        println!("Tengo este commit para buscar entre mis objects y mandar {:?}", commit);
    }
 */
    let packfile_result = Encoder::init_encoder((&path).to_path_buf());
    println!("PACKFILE COMPLETO : {:?}", packfile_result);

    //Si el cliente recibe mensaje  0000 y done, envia paquete directamente!
    match packfile_result {
        Ok(mut packfile) => {
            //let extra_bytes = vec![48,48,48,56,78,65,75,10];
            //packfile.splice(0..0, extra_bytes);
            println!("PACKFILE COMPLETO : {:?}", packfile);
            stream.write(&packfile)?;
        },
        Err(e) => {
            println!("Error al inicializar el packfile: {:?}", e);
        }
    }
    //envio paquete como respuesta
    //send done and 0000 mssge
    //salgo de aca para entrar al bucle, de espera de otras consultas
    Ok("0000".to_string())
}


pub fn receive_wants_and_have_message(reader: &mut TcpStream) -> Result<Vec<String>, io::Error> {
    let mut query = vec![];
    loop {
        let msg_received = process_line(reader)?;
        println!("Mensaje recibido ------> {:?}", msg_received);
        query.push(msg_received.clone());
        if msg_received == "0000" {
            break;
        }
    }
    Ok(query)
}


pub fn handler_upload_pack(path: &PathBuf) -> Result<String, std::io::Error> {
    let logs_path = path.join(".rust_git").join("logs");
    let log_entries = get_log_entries(&logs_path)?;
    Ok(log_entries)
}

fn get_log_entries(logs_path: &Path) -> Result<String, std::io::Error>{
    let mut log_entries = String::new();

    let entries = fs::read_dir(logs_path)?;
    for entry in entries {
        let log_file = entry?;
        let file = fs::File::open(log_file.path())?;
        
        let mut reader = BufReader::new(file);
        let mut last_line = String::new();

        for line in reader.by_ref().lines() {
            if let Ok(line) = line {
                last_line = line;
            }
        }
        if let Some(hash) = parse_log_line(&last_line) {
            let filename = log_file.file_name().to_string_lossy().to_string();
            log_entries.push_str(&format!("{} refs/heads/{}\n", hash, filename));
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


fn send_response(response: &String, writer: &mut TcpStream) -> Result<(), std::io::Error> {
    print!("MI RESPONSE ES {}", response);
    if response.contains("\n"){
        for line in response.lines(){
            let line_without_newline = line.trim_end().trim_end();
            let msg_response = format!("{}\n", line_without_newline);                
            let pkt_response = to_pkt_line(&msg_response);
            writer.write(pkt_response.as_bytes())?;
        }
    } else {
        //writer.write(to_pkt_line(response.as_str()).as_bytes())?;
    }
    writer.write("0000".as_bytes())?;
    //writer.flush()?;
    Ok(())
}