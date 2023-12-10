
use std::{net::TcpStream, io::Write};

use serde::{Serialize, Deserialize};

use crate::server_http::validation::{send_server_error_msg, send_response};

#[derive(Serialize, Deserialize)]
pub struct CreatePullRequest{
    pub title: Option<String>,
    pub body: Option<String>,
    pub head_repo: String,
    pub base_repo: String,
    pub head: String,
    pub base: String,
    pub username: String,
    pub mergeable: bool,
}


impl CreatePullRequest {

    pub fn response_create_pull_request_object(json_body: &str, stream: &mut TcpStream) -> Result<CreatePullRequest, std::io::Error> {
        println!("JSON Body: {}", json_body);
        if let Ok(request) = serde_json::from_str::<CreatePullRequest>(json_body) {            
            send_response(stream, "RESPUESTA FUNCIONALIDAD".to_string());
            return Ok(request)
        } else {
            println!("Error al deserializar el mensaje: trailing characters");
            send_server_error_msg(stream);
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Error parsing request"))
        };
    }

}