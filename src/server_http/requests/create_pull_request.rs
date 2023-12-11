
use std::net::TcpStream;

use serde::{Serialize, Deserialize};

use crate::server_http::validation::{send_server_error_msg, send_response};
/* 
pub struct InputCreatePR{
    title: Option
    body: Option
    head_repo: Option
    head: String
    base: String
}
*/
/* POST /gmovia/algo1 (base_repo)
{
    head: 'gmovia:new_branch' 
    base: 'master'
}

CreatePullRequest{
    title: None
    description: None
    head_repo: gmovia/algo1
    base_repo: gmovia/algo1
    head: gmovia:new_branch
    username: gmovia
    base: master
    mergeable: false
}
*/

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

    pub fn parser(json: &str, arg_path: Vec<&str>){
        // Si tiene titulo lo agarro y sino seteo en None
        // Si tiene body lo agarro y sino seteo en None
        // Si tiene head_repo lo agarro y sino lo igualo al arg_path
        // Base repo es OBLIGATORIO, es igual a arg_path
        // Head es OBLIGATORIO
        // Base es OBLIGATORIO  => te va a llegar username:nombre_rama, tenes que dividirlo haciendo split de :
        // Username viene con base asi que es obligatorio
        // mergeable siempre false => no viene por parametro, es un atributo nuestro

        // POST /repos/{repo}/pulls
        //if json.contains(pat)
    }

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