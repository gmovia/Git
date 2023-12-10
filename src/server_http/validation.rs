use std::{net::TcpStream, io::Write};

pub fn send_bad_request_msg(mut stream: &TcpStream){
    let response = "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n";
    let _ = stream.write(response.as_bytes());
}