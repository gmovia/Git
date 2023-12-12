
use std::net::TcpStream;

use serde::{Serialize, Deserialize};

use crate::{server_http::{validation::{send_server_error_msg, send_response, send_error}}, pull_request::{controllers::pull_request::PullRequest}};

#[derive(Serialize, Deserialize)]
pub struct JsonCreatePR{
    title: Option<String>,
    body: Option<String>,
    head_repo: Option<String>,
    head: String,
    base: String,
}

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

#[derive(Serialize, Deserialize)]
pub struct ResponseOkCreatePullRequest {
    id: String
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

    pub fn response_create_pull_request_object(json_body: &str, path: String, stream: &mut TcpStream, pull_request: PullRequest) -> Result<CreatePullRequest, std::io::Error> {
        println!("JSON Body: {}", json_body);
        if let Ok(request) = serde_json::from_str::<JsonCreatePR>(json_body) {            
            let head_vec: Vec<&str> = request.head.split(":").collect();

            let mut create = CreatePullRequest {
                title: request.title,
                body: request.body,
                head_repo: "amoralejo/algo1".to_string(),
                base_repo: path,
                head: head_vec[1].to_owned(),
                base: request.base,
                username: head_vec[0].to_owned(),
                mergeable: false,
            };

            if request.head_repo.is_none() {
                create.head_repo = create.base_repo.clone()
            }

            match pull_request.create(&mut create) {
                Ok( response ) => { send_response(stream, ResponseOkCreatePullRequest { id: response})},
                Err( error_code ) => { send_error(stream, error_code.to_string()) }
            }; 
            
            return Ok(create)
        } else {
            println!("Error al deserializar el mensaje: trailing characters");
            send_server_error_msg(stream);
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Error parsing request"))
        };
    }

}