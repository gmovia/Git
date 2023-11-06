use std::{net::{TcpListener, TcpStream, Shutdown}, io::{Write, Error}, thread, path::{Path, PathBuf}};

use crate::{packfile::packfile::to_pkt_line,constants::constants::{PUERTO,HOST}};

use crate::packfile::packfile::process_line;
use crate::server::upload_pack::start_handler_upload;


pub struct Server {
   pub  path: PathBuf,
}


impl Server {

    pub fn init_server(path: String) -> Result<Server, std::io::Error> {
        println!("INIT SERVER--------->");
        let path = Path::new(&path);
        let server = Server { path: path.to_path_buf() };
        //let encoder = Encoder::init_encoder((&path).to_path_buf());
        server.handle_connections()?;
        Ok(server)
    }

    fn _star_repositorys(){
       // let mut remoteRepository = RemoteRepository::init(Path::new(r"C:\Users\luzmi\OneDrive\Escritorio\RemoteRepository\Probanding"), Vec::new());
        //para despues....
    }


    fn handle_connections(&self) -> Result<(),std::io::Error> {
        let adress = format!("{}:{}",HOST,PUERTO);
        let listener = TcpListener::bind(&adress).expect("Failed to bind to address");
    
        for stream in listener.incoming() {
            match stream {
                Ok(client) => {
                    let read_client = client.try_clone()?;
                    let write_client = client.try_clone()?;
                    let path = self.path.clone();
                    let current =
                    thread::spawn(move || {
                        print!("Parada en el server\n");
                        match Server::handle_client( read_client, write_client, &path) {
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

    fn _send_response(response: &String, writer: &mut TcpStream) -> Result<(), std::io::Error> {
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

    fn handle_client( mut reader: TcpStream, mut _writer: TcpStream, path: &PathBuf) -> Result<(),std::io::Error> {
        loop {
            match process_line(&mut reader) {
                Ok(message) => {
                    println!("Received message from client: {}", &message);
                    let client_path = message.trim_start_matches("git-upload-pack ");
                    let _ = Server::parse_response(&message.to_string(), &mut reader, &path.join(client_path))?;
                    Self::shutdown_server(&reader)?;
                }
                Err(e) => {
                    eprintln!("Error reading from client: {}", e);
                    break;
                }
            }
        }
    
        Ok(())
    }

    fn shutdown_server(socket: &TcpStream) -> Result<(), Error> {
        match socket.shutdown(Shutdown::Write) {
            Ok(()) => {
                println!("Conexión cerrada exitosamente.");
                Ok(())
            },
            Err(e) => {
                println!("Error al cerrar la conexión: {:?}", e);
                Err(e)
            }
        }
    }

    fn parse_response( message: &String, reader: &mut TcpStream, path: &PathBuf) -> Result<String, std::io::Error> {
        let response = match message.as_str() {
            s if s.contains("hola") => "Hola".to_string(),
            s if s.contains("chau") => "Chau".to_string(),
            //s if s.contains("git-upload-pack") => "UPLOADDDD".to_string(),

            s if s.contains("git-upload-pack") => start_handler_upload(reader, path)?,
            _ => "No entiendo tu mensaje".to_string(),
        };
        Ok(response)
    }
    
}