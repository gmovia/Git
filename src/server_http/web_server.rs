use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Mensaje {
    mensaje: String,
}
pub struct WebServer;

impl WebServer {

    pub fn new() -> Result<String,String>{
        let listener = TcpListener::bind("127.0.0.1:8080").expect("Error al vincular el puerto");

        println!("Servidor escuchando en http://127.0.0.1:8080");

        // Aceptar conexiones y manejarlas en hilos separados
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
    
        // Leer los datos del stream
        match stream.read(&mut buffer) {
            Ok(_) => {
                // Deserializar el mensaje JSON
                let json_str = String::from_utf8_lossy(&buffer).to_string();
                println!("DES: {}", json_str);
                
                if let Some(start_index) = json_str.find("\n\n") {
                    let remaining_part = &json_str[start_index + 2..];
                    
                    println!("Parte después del primer doble salto de línea: {}", remaining_part);
                    if let Ok(mensaje) = serde_json::from_str::<Mensaje>(&remaining_part) {
                        println!("El mensaje es: {}", mensaje.mensaje);
                        let response = format!("HTTP/1.1 200 OK\r\n\r\nMensaje recibido: {}\r\n", mensaje.mensaje);
                        // Enviar la respuesta de vuelta al cliente
                        stream.write(response.as_bytes()).unwrap();
                    } else {
                        // Manejar errores de deserialización
                        println!("Error al deserializar el mensaje");
                        stream.write(b"Error al deserializar el mensaje").unwrap();
                    }
                } else {
                    println!("No se encontró el primer doble salto de línea en la cadena.");
                }
            }
            Err(e) => {
                println!("Error al leer del stream: {}", e);
            }
        }
    }

}
