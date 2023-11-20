use std::{net::{TcpListener, TcpStream, Shutdown}, thread, path::{Path, PathBuf}, io::Write};

use crate::{constants::constants::{HOST, PUERTO}, packfile::packfile::{process_line, to_pkt_line}};

use super::upload_pack::start_handler_upload;

pub struct Server;

impl Server {
    
    /// Esta funcion sirve para inicializar el servidor. Espera por nuevas conexiones por parte del cliente
    pub fn server(server_path: String) -> Result<(),std::io::Error> {
        let adress = format!("{}:{}", HOST, PUERTO);
        
        let listener = TcpListener::bind(&adress).expect("Failed to bind to address");
        let path = Path::new(&server_path);
        for stream in listener.incoming() {
            match stream {
                Ok(client) => {
                    let read_client = client.try_clone()?;
                    let write_client = client.try_clone()?;
                    let path_clone = path.to_path_buf();
                    let _ =
                    thread::spawn(move || {
                        match Server::handle_client( read_client, write_client, &path_clone) {
                            Ok(_) => Ok(()),
                            Err(e) => Err(e)
                        }
                    });
                }
                Err(e) => {
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, "fatal error: the path is not correct"));
                    //eprintln!("Error: {}", e);
                }
            }
        }
        Ok(())
    }

    /// Esta funcion se queda loopeando constantemente esperando por posibles mensajes que le lleguen desde el cliente.
    fn handle_client(mut reader: TcpStream, mut writer: TcpStream, path: &Path) -> Result<(),std::io::Error> {
        loop {
            match process_line(&mut reader) {
                Ok(message) => {
                    println!("Received message from client: {}", &message);
                    let client_path = message.trim_start_matches("git-upload-pack ");
                    let aux = format!("{}/{}",path.display().to_string(), client_path);
                    let server_path = Path::new(&aux);

                    if !server_path.exists(){
                        let message_error = "fatal error: the path is not correct";
                        let _ = writer.write(to_pkt_line(&message_error).as_bytes());
                        return Err(std::io::Error::new(std::io::ErrorKind::Other, "fatal error: the path is not correct"));
                    }


                    if let Err(e) = Server::parse_response(&message.to_string(), &mut reader, &server_path.to_path_buf()) {
                        println!("Error parsing response: {}",e)
                    }
                    Server::shutdown_server(&reader)?;
                }
                Err(e) => {
                    eprintln!("Error reading from client: {}", e);
                    break;
                }
            }
        }
    
        Ok(())
    }

    /// Esta funcion se encarga de responder al mensaje recibido por parte del cliente
    fn parse_response( message: &String, reader: &mut TcpStream, path: &PathBuf) -> Result<String, std::io::Error> {       
        let response = match message.as_str() {
            s if s.contains("git-upload-pack") => start_handler_upload(reader, path)?,
            _ => "No entiendo tu mensaje".to_string(),
        };
        Ok(response)
    }

    fn shutdown_server(socket: &TcpStream) -> Result<(), std::io::Error> {
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


}