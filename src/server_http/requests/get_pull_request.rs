
use std::net::TcpStream;

use serde::{Serialize, Deserialize};

use crate::server_http::validation::send_response;

#[derive(Serialize, Deserialize)]
pub struct GetPullRequest{
    pub base_repo: String,
    pub id: String,
}


impl GetPullRequest {

    pub fn get_pull_request(json_body: &str, stream: &mut TcpStream) -> Result<GetPullRequest, std::io::Error> {
        println!("JSON Body: {}", json_body);
        if let Ok(request) = serde_json::from_str::<GetPullRequest>(json_body) {            
            send_response(stream, "RESPUESTA FUNCIONALIDAD".to_string());
            return Ok(request)
        } else {
            println!("Error al deserializar el mensaje: trailing characters");
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Error parsing request"))
        };
    }


}