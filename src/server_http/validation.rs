use std::{net::TcpStream, io::Write};

use serde::Serialize;

pub fn send_bad_request_msg(mut stream: &TcpStream){
    let response = "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n";
    let _ = stream.write(response.as_bytes());
}

pub fn send_server_error_msg(mut stream: &TcpStream){
    let response = "HTTP/1.1 500 Internal Server Error\r\nContent-Length: 0\r\n\r\n";
    let _ = stream.write(response.as_bytes());
}

pub fn send_error(mut stream: &TcpStream, error_code: String){
    let response = format!("HTTP/1.1 {}\r\nContent-Length: 0\r\n\r\n", error_code);
    let _ = stream.write(response.as_bytes());
}

pub fn send_response<T: Serialize>(stream: &mut TcpStream, server_response: T) {
    if let Ok(json) = serde_json::to_string(&server_response) {
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}\r\n", json);
        let _ = stream.write(response.as_bytes());    
    }
}
