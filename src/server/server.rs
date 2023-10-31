use std::{net::{TcpListener, TcpStream}, io::{Read, Write, self}, thread, path::{Path, PathBuf}};

use rand::Error;

use crate::{vcs::version_control_system::VersionControlSystem, handlers::{status::handler_status, add::handler_add, hash_object::handler_hash_object, cat_file::handler_cat_file, rm::handler_rm, log::handler_log, commit::handler_commit, branch::handler_branch}};

use crate::packfile::packfile::process_line;

use super::encoder::{self, Encoder};


pub struct Server {
    path: PathBuf
}


impl Server {

    pub fn init_server() -> Result<Server,std::io::Error> {
        let server = Server { path: Path::new("/home/amoralejo/TEST").to_path_buf() };
        let path = Path::new("/home/amoralejo/TEST").to_path_buf();
        let encoder = Encoder::init_encoder(path);
        Self::handle_connections(&server)?;
        Ok(server)
    }


    fn handle_connections(&self) -> Result<(),std::io::Error> {
        let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");

        for stream in listener.incoming() {
            match stream {
                Ok(client) => {
                    let read_client = client.try_clone()?;
                    let write_client = client.try_clone()?;
    
                    thread::spawn(move || {
                        print!("Parada en el server\n");
                        match Self::handle_client(read_client, write_client) {
                            Ok(_) => Ok(()),
                            Err(e) => Err(e)
                        }
                    });
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
        Ok(())
    }

/* 
    fn handle_client(mut reader: TcpStream, mut writer: TcpStream) -> Result<(),std::io::Error> {
        let mut buffer = [0; 1024];
        let mut message = String::new();
    
        loop {
            match reader.read(&mut buffer) {
                Ok(0) => {
                    break;
                }
                Ok(n) => {
                    message.push_str(&String::from_utf8_lossy(&buffer[..n]).trim()); 
    
                    let response = Self::parse_respose(&message)?;
    
                    println!("Received message from client: {}", &message);
    
                    writer.write(response.as_bytes())?;
    
                    buffer = [0; 1024];
                    message.clear();
                }
                Err(e) => {
                    eprintln!("Error reading from client: {}", e);
                    break;
                }
            }
        };

        Ok(())
    } */

    fn handle_client(mut reader: TcpStream, mut writer: TcpStream) -> Result<(),std::io::Error> {
        loop {
            match process_line(&mut reader) {
                Ok(message) => {
                    println!("Received message from client: {}", &message);
                    let response = Self::parse_response( &message.to_string())?;
                    writer.write(response.as_bytes())?;
                    writer.flush()?;
                }
                Err(e) => {
                    eprintln!("Error reading from client: {}", e);
                    break;
                }
            }
        }
    
        Ok(())
    }

    fn parse_response(message: &String) -> Result<String, std::io::Error> {
        let response = match message.as_str() {
            s if s.contains("hola") => "Hola".to_string(),
            s if s.contains("chau") => "Chau".to_string(),
            s if s.contains("git-upload-pack") => "Entrando a responder la clonacion...".to_string(),
            //"git clone" => receive_query_clone(response, socket),
            _ => "No entiendo tu mensaje".to_string(),
        };
        Ok(response)
    }
    
}