use std::{fs, io};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use std::net::TcpStream;
use crate::packfiles::packfile::{to_pkt_line, process_line};
use crate::servers::encoder::Encoder;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::net::TcpStream;
use crate::packfiles::packfile::{to_pkt_line, process_line};
use crate::servers::encoder::Encoder;
use crate::vcs::files::current_commit::CurrentCommit;


/// Esta funcion se encarga de procesar la respuesta que el server le entregara al cliente al mensaje de upload pack
pub fn start_handler_upload(stream: &mut TcpStream, path: &Path) -> Result<String, std::io::Error> {
    let first_response = handler_upload_pack(path)?;

    send_response(first_response, stream)?;
    
    let query = receive_wants_and_have_message(stream)?;
    let packfile_result = Encoder::init_encoder(path.to_path_buf(), query);

    match packfile_result {
        Ok( packfile) => {
            let _ = stream.write(&packfile);
            println!("PAQUETE ENVIADO CON EXITO\n");
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
    let log_entries = get_log_entries(&path)?;
    Ok(log_entries)
}

fn get_log_entries(path: &Path) -> Result<Vec<String>, std::io::Error>{
    let mut log_entries = Vec::new();
    let logs_path = path.join(".rust_git").join("logs");
    
    let entries = fs::read_dir(logs_path)?;
    
    for entry in entries {
        let log_file = entry?;
        let _ = fs::File::open(log_file.path())?;
        
        if let Some(branch_name) = log_file.path().file_name() {
            let current_hash  = CurrentCommit::read_for_branch(path, &branch_name.to_string_lossy())?;
            let format = format!("{} refs/heads/{}", current_hash, branch_name.to_string_lossy().to_string());
            log_entries.push(format);

        }
    }
    Ok(log_entries)
}



fn send_response(response: Vec<String>, writer: &mut TcpStream) -> Result<(), std::io::Error> {
    for resp in response {
        if resp.contains('\n'){
            for line in resp.lines(){
                let line_without_newline = line.trim_end().trim_end();
                let msg_response = format!("{}\n", line_without_newline);                
                let pkt_response = to_pkt_line(&msg_response);
                let _ = writer.write(pkt_response.as_bytes());
            }
        } else {
                let _ = writer.write(to_pkt_line(resp.as_str()).as_bytes());
            
        }
    }    
    let _ = writer.write("0000".as_bytes());
    Ok(())
}