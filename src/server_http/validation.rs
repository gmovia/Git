use std::{net::TcpStream, io::Write};

pub fn send_bad_request_msg(mut stream: &TcpStream){
    let response = "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n";
    let _ = stream.write(response.as_bytes());
}

pub fn send_server_error_msg(mut stream: &TcpStream){
    let response = "HTTP/1.1 500 Internal Server Error\r\nContent-Length: 0\r\n\r\n";
    let _ = stream.write(response.as_bytes());
}

pub fn send_response(stream: &mut TcpStream, server_response: String) {
    let response = format!("HTTP/1.1 200 OK\r\n\r\nMensaje recibido: {} devuelto ;)\r\n", server_response);
    let _ = stream.write(response.as_bytes());
    
}