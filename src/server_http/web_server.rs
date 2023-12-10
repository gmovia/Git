use std::fs::File;
use std::io::{prelude::*, self};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::thread;
use serde::{Deserialize, Serialize};

use crate::server_http::validation::send_bad_request_msg;

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
                Ok(mut stream) => {
                    thread::spawn(move || {
                        Self::handle_client(&mut stream);
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

    fn handle_client(stream: &mut TcpStream) {
        let mut buffer = [0; 1024];
    
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                let request_str = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
                println!("Received request: {}", request_str);
    
                if let Some(header_end) = request_str.find("\r\n\r\n") {
                    let _ = Self::parse_request(header_end, &request_str, stream);
                } else {
                    println!("No se encontró el final de las cabeceras en la cadena.");
                    send_bad_request_msg(stream);
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    fn parse_request(header_end: usize, request_str: &str, stream: &mut TcpStream) -> Result<(), std::io::Error> {
        let json_header = &request_str[..header_end]; 
        let json_body = &request_str[header_end + 4..];
    
        let received_request = Self::get_received_request(json_header)?;
        let received_vec: Vec<&str> = received_request.split_whitespace().collect();
        let path: Vec<&str> = received_vec[1].split("/").collect();
        println!("ES ESTE EL RECEIVE_VECCC en 0{:?}\n", received_vec[0]);
    
        match (received_vec[0], path.len() - 1) {
            ("POST", 3) => println!("CREAR UN PULL REQUEST"),
            ("GET", 3) => println!("LISTAR PULL REQUEST"),
            ("GET", 4) => println!("OBTENER UN PULL REQUEST"),
            ("GET", 5) => println!("LISTAR COMMIT EN UN PULL REQUEST"),
            ("PUT", 5) => println!("LISTAR COMMIT EN UN PULL REQUEST"),
            _ => send_bad_request_msg(&stream),
        }
    
        println!("            -----> {}", received_request);
        println!("PATH: {:?}", path);
        Self::send_mesagge(json_body, stream);
        Ok(())
    }

    fn get_received_request(header: &str) -> Result<String, std::io::Error> {
        let header_vec: Vec<&str> = header.split("\n").collect();
        let receive_request = header_vec[0];
        Ok(receive_request.to_string()) 
    }

    fn send_mesagge(json_body: &str, stream: &mut TcpStream) {
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

