use std::{net::{TcpListener, TcpStream}, io::{Read, Write, self}, thread, path::{Path, PathBuf}};

use rand::Error;

use crate::{vcs::{version_control_system::VersionControlSystem, files::repository}, handlers::{status::handler_status, add::handler_add, hash_object::handler_hash_object, cat_file::handler_cat_file, rm::handler_rm, log::handler_log, commit::handler_commit, branch::handler_branch}};

use crate::packfile::packfile::process_line;
use crate::server::upload_pack::handler_upload_pack;

use super::encoder::{self, Encoder};


pub struct Server {
   pub  path: PathBuf,
}


impl Server {

    pub fn init_server() -> Result<Server, std::io::Error> {
        let path = Path::new("/home/amoralejo/TEST");
        let server = Server { path: path.to_path_buf() };
        let encoder = Encoder::init_encoder((&path).to_path_buf());
        server.handle_connections()?;
        Ok(server)
    }

    fn star_repositorys(){
       // let mut remoteRepository = RemoteRepository::init(Path::new(r"C:\Users\luzmi\OneDrive\Escritorio\RemoteRepository\Probanding"), Vec::new());
        //para despues....
    }


    fn handle_connections(&self) -> Result<(),std::io::Error> {
        let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");
    
        for stream in listener.incoming() {
            match stream {
                Ok(client) => {
                    let read_client = client.try_clone()?;
                    let write_client = client.try_clone()?;
                    let path = self.path.clone();
    
                    thread::spawn(move || {
                        print!("Parada en el server\n");
                        match Server::handle_client(read_client, write_client, &path) {
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

    fn handle_client(mut reader: TcpStream, mut writer: TcpStream, path: &PathBuf) -> Result<(),std::io::Error> {
        loop {
            match process_line(&mut reader) {
                Ok(message) => {
                    println!("Received message from client: {}", &message);
                    let response = Server::parse_response( &message.to_string(), &mut reader, path)?;
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

    fn parse_response(message: &String, reader: &mut TcpStream, path: &PathBuf) -> Result<String, std::io::Error> {
        let response = match message.as_str() {
            s if s.contains("hola") => "Hola".to_string(),
            s if s.contains("chau") => "Chau".to_string(),
            s if s.contains("git-upload-pack") => handler_upload_pack(message.clone(), reader, path)?,
            _ => "No entiendo tu mensaje".to_string(),
        };
        Ok(response)
    }
    
}