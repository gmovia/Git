
use std::net::TcpStream;

use serde::{Serialize, Deserialize};

use crate::server_http::validation::send_response;

#[derive(Serialize, Deserialize)]
pub struct ListPullRequests{
    pub base_repo: String,
    pub state: Option<String>,
    pub head: Option<String>,
    pub base: Option<String>,
    pub username: Option<String>,
    pub per_page: Option<i32> 
}


impl ListPullRequests {

    pub fn response_list_pull_request_object(json_body: &str, stream: &mut TcpStream) -> Result<ListPullRequests, std::io::Error> {
        println!("JSON Body: {}", json_body);
        if let Ok(request) = serde_json::from_str::<ListPullRequests>(json_body) {            
            send_response(stream, "RESPUESTA FUNCIONALIDAD".to_string());
            return Ok(request)
        } else {
            println!("Error al deserializar el mensaje: trailing characters");
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Error parsing request"))
        };
    }


}