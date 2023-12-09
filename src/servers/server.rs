use std::{
    io::Write,
    net::{Shutdown, TcpListener, TcpStream},
    path::{Path, PathBuf},
    thread,
};

use crate::{
    constants::constant::{HOST, PUERTO},
    packfiles::packfile::{process_line, to_pkt_line},
    protocol::receive_pack::start_handler_receive,
    vcs::commands::push::Push, server_http::web_server::WebServer,
};

use super::upload_pack::start_handler_upload;

/// Este struct representa al nuestro servidor.
pub struct Server;

impl Server {
    /// Esta funcion sirve para inicializar el servidor. Espera por nuevas conexiones por parte del cliente
    pub fn server(server_path: String) -> Result<(), std::io::Error> {
        Self::conect_web_server();
        let adress = format!("{}:{}", HOST, PUERTO);
        let listener = TcpListener::bind(&adress).expect("Failed to bind to address");
        let path = Path::new(&server_path);
        for stream in listener.incoming() {
            match stream {
                Ok(client) => {
                    let read_client = client.try_clone()?;
                    let write_client = client.try_clone()?;
                    let path_clone = path.to_path_buf();
                    let _ = thread::spawn(move || {
                        match Server::handle_client(read_client, write_client, &path_clone) {
                            Ok(_) => Ok(()),
                            Err(e) => Err(e),
                        }
                    });
                }
                Err(_) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "fatal error: the path is not correct",
                    ));
                }
            }
        }
        Ok(())
    }

    fn conect_web_server() {
        let _ = thread::spawn(move || {
            let _ = WebServer::new();
        });
    }

    fn handle_client(
        mut reader: TcpStream,
        mut writer: TcpStream,
        path: &Path,
    ) -> Result<(), std::io::Error> {
        loop {
            match process_line(&mut reader) {
                Ok(message) => {
                    let server_path = Self::extract_path(&message, path)?;
                    if !server_path.exists() {
                        let message_error = "fatal error: the path is not correct";
                        let _ = writer.write(to_pkt_line(message_error).as_bytes());
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "fatal error: the path is not correct",
                        ));
                    }

                    if let Err(e) =
                        Server::parse_response(&message.to_string(), &mut reader, &server_path)
                    {
                        println!("Error parsing response: {}", e)
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

    fn extract_path(message: &str, path: &Path) -> Result<PathBuf, std::io::Error> {
        let client_path = match message {
            s if s.contains("git-upload-pack") => {
                message.trim_start_matches("git-upload-pack ").to_string()
            }
            s if s.contains("git-receive-pack") => {
                Push::parse_query_to_extract_path(message)?.to_string()
            }
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "No entiendo tu mensaje",
                ))
            }
        };
        let aux = format!("{}/{}", path.display(), client_path);
        let server_path = Path::new(&aux).to_path_buf();

        Ok(server_path)
    }

    /// Esta funcion se encarga de responder al mensaje recibido por parte del cliente
    fn parse_response(
        message: &str,
        reader: &mut TcpStream,
        path: &Path,
    ) -> Result<String, std::io::Error> {
        let response = match message {
            s if s.contains("git-upload-pack") => start_handler_upload(reader, path)?,
            s if s.contains("git-receive-pack") => {
                start_handler_receive(reader, path.to_path_buf())?
            }
            _ => "No entiendo tu mensaje".to_string(),
        };
        Ok(response)
    }

    fn shutdown_server(socket: &TcpStream) -> Result<(), std::io::Error> {
        match socket.shutdown(Shutdown::Write) {
            Ok(()) => {
                println!("Conexión cerrada exitosamente.");
                Ok(())
            }
            Err(e) => {
                println!("Error al cerrar la conexión: {:?}", e);
                Err(e)
            }
        }
    }
}
