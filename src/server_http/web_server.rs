use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct Mensaje {
    mensaje: String,
}
pub struct WebServer;

impl WebServer {

    pub fn new() -> Result<String,String>{
        let listener = TcpListener::bind("127.0.0.1:8080").expect("Error al vincular el puerto");

        println!("Servidor escuchando en http://127.0.0.1:8080");
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(|| {
                        Self::handle_client(stream);
                    });
                }
                Err(e) => {
                    println!("Error al aceptar la conexión: {}", e);
                }
            }
        };
        Ok("Okey".to_string())
    }
    
    fn handle_client(mut stream: TcpStream) {
        let mut buffer = [0; 1024];
    
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                let request_str = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
                println!("Received request: {}", request_str);
    
                if let Some(header_end) = request_str.find("\r\n\r\n") {
                    let body_start = header_end + 4; 
    
                        let json_body = &request_str[body_start..];
                    println!("JSON Body: {}", json_body);
    
                    if let Ok(mensaje) = serde_json::from_str::<Mensaje>(json_body) {
                        println!("El mensaje es: {}", mensaje.mensaje);
    
                        let response = format!("HTTP/1.1 200 OK\r\n\r\nMensaje recibido: {}\r\n", mensaje.mensaje);
                        let _ = stream.write(response.as_bytes());
                    } else {
                        println!("Error al deserializar el mensaje: trailing characters");
                        let response = "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n";
                        let _ = stream.write(response.as_bytes());
                    }
                } else {
                    println!("No se encontró el final de las cabeceras en la cadena.");
                    let response = "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n";
                    let _ = stream.write(response.as_bytes());
                }
            }
            Err(e) => {
                println!("Error al leer del stream: {}", e);
            }
        }
    }
    
}
