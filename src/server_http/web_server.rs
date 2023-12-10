use std::fs::File;
use std::io::{prelude::*, self};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::thread;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Mensaje {
    mensaje: String,
}
pub struct WebServer;

impl WebServer {

    pub fn new() -> Result<(),std::io::Error>{
        let port = Self::get_config()?;
        let listener = TcpListener::bind(&port).expect("Error getting port");

        println!("Web server listening on port: {}", port);
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(|| {
                        Self::handle_client(stream);
                    });
                }
                Err(_) => {
                    std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "Error conecting to web server");
                }
            }
        };
        Ok(())
    }
    
    fn get_config() -> Result<String,std::io::Error>{
        let mut port = String::new();
        
        let path = Path::new("src/server_http/web_server_config.txt");
        let file = File::open(&path)?;
    
        let reader = io::BufReader::new(file);
    
        for line in reader.lines() {
            let line_str = line?;

            if line_str.contains("port") {
                let port_vec: Vec<&str> = line_str.split("=").collect();
                port = port_vec[1].to_owned();
            }
        }
        Ok(port)
    }

    fn handle_client(mut stream: TcpStream) {
        let mut buffer = [0; 1024];
    
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                let request_str = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
                println!("Received request: {}", request_str);
    
                if let Some(header_end) = request_str.find("\r\n\r\n") {
                    Self::parse_request(header_end, &request_str, stream);
                } else {
                    println!("No se encontró el final de las cabeceras en la cadena.");
                    let response = "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n";
                    let _ = stream.write(response.as_bytes());
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    fn parse_request(header_end: usize, request_str: &str, stream: TcpStream) -> Result<(), std::io::Error> {
        let json_header = &request_str[..header_end]; 
        let json_body = &request_str[header_end + 4..];

        let received_request = Self::get_received_request(json_header)?;
        let received_vec: Vec<&str> = received_request.split_whitespace().collect();
        let path = received_vec[1];

        println!("            -----> {}", received_request);
        println!("PATH: {}", path);
        Self::send_mesagge(json_body, stream);
        Ok(())
    }

    fn get_received_request(header: &str) -> Result<String, std::io::Error> {
        let header_vec: Vec<&str> = header.split("\n").collect();
        let receive_request = header_vec[0];
        Ok(receive_request.to_string()) 
    }

    fn send_mesagge(json_body: &str, mut stream: TcpStream) {
        println!("JSON Body: {}", json_body);
        if let Ok(mensaje) = serde_json::from_str::<Mensaje>(json_body) {
            println!("El mensaje es: {}", mensaje.mensaje);

            let response = format!("HTTP/1.1 200 OK\r\n\r\nMensaje recibido: {} devuelto ;)\r\n", mensaje.mensaje);
            let _ = stream.write(response.as_bytes());
        } else {
            println!("Error al deserializar el mensaje: trailing characters");
            let response = "HTTP/1.1 500 Internal Server Error\r\nContent-Length: 0\r\n\r\n";
            let _ = stream.write(response.as_bytes());
        }
    }
    
}
