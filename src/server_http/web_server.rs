use std::fs::File;
use std::io::{prelude::*, self};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::thread;
use serde::{Deserialize, Serialize};

use crate::handlers::pull;
use crate::pull_request;
use crate::pull_request::controllers::pull_request::PullRequest;
use crate::server_http::requests::create_pull_request::CreatePullRequest;
use crate::server_http::requests::get_pull_request::GetPullRequest;
use crate::server_http::requests::list_commit::ListCommitsPullRequest;
use crate::server_http::requests::list_pull_request::ListPullRequests;
use crate::server_http::requests::merge_pull_request::MergePullRequest;
use crate::server_http::validation::send_bad_request_msg;

#[derive(Serialize, Deserialize)]
pub struct Mensaje {
    mensaje: String,
}
pub struct WebServer;

impl WebServer {

    pub fn new(server_path: PathBuf) -> Result<(),std::io::Error>{
        let port = Self::get_config()?;
        let listener = TcpListener::bind(&port).expect("Error getting port");

        println!("Web server listening on port: {}", port);
        let path = server_path.clone();
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let path = path.clone();
                    thread::spawn(move || {
                        Self::handle_client(&mut stream, path);
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

    fn handle_client(stream: &mut TcpStream, server_path: PathBuf) {
        let mut buffer = [0; 1024];
    
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                let request_str = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
                println!("Received request: {}", request_str);
    
                if let Some(header_end) = request_str.find("\r\n\r\n") {
                    let _ = Self::parse_request(header_end, &request_str, stream, server_path);
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

    fn parse_request(header_end: usize, request_str: &str, stream: &mut TcpStream, server_path: PathBuf) -> Result<(), std::io::Error> {
        let pull_request = PullRequest::init(&server_path);
        let json_header = &request_str[..header_end]; 
        let json_body = &request_str[header_end + 4..];
    
        let received_request = Self::get_received_request(json_header)?;
        let received_vec: Vec<&str> = received_request.split_whitespace().collect();
        let path: Vec<&str> = received_vec[1].split("/").collect();
        println!("PATH LEN: {}",path.len());
        match (received_vec[0], path.len() - 1) {
            ("POST", 4) => {let _ = CreatePullRequest::response_create_pull_request_object(json_body, format!("{}/{}",path[2],path[3]),stream,pull_request);},
            ("GET", 4) => {let _ = ListPullRequests::response_list_pull_request_object(json_body, stream, format!("{}/{}",path[2],path[3]),pull_request);},
            ("GET", 5) => {let _ = GetPullRequest::get_pull_request(stream, pull_request, format!("{}/{}",path[2],path[3]), path[5].to_owned());},
            ("GET", 6) => {let _ = ListCommitsPullRequest::list_commits_pull_request(json_body, stream);},
            ("PUT", 6) => {let _ = MergePullRequest::merge_pull_request(json_body, stream);},
            _ => send_bad_request_msg(&stream),
        }
    
        println!("            -----> {}", received_request);
        println!("PATH: {:?}", path);
        Ok(())
    }

    fn get_received_request(header: &str) -> Result<String, std::io::Error> {
        let header_vec: Vec<&str> = header.split("\n").collect();
        let receive_request = header_vec[0];
        Ok(receive_request.to_string()) 
    }
    
}

