
use std::net::TcpStream;

use serde::{Serialize, Deserialize};

use crate::{pull_request::controllers::pull_request::PullRequest, server_http::sender::{send_response, send_error}};

#[derive(Serialize, Deserialize)]
pub struct MergePullRequest{
    pub base_repo: String,
    pub id: String,
    // commit_tittle: String,
    // commit_message: String,
    // merge_method: String
}


impl MergePullRequest {

    pub fn merge_pull_request(json_body: &str, stream: &mut TcpStream, base_repo: String, id: String, pull_request: PullRequest) -> Result<(), std::io::Error> {

        //if let Ok(request) = serde_json::from_str::<MergePullRequest>(json_body) {      
            let merge_pr = MergePullRequest {
                base_repo,
                id,
                // commit_tittle: request.commit_tittle,
                // commit_message: request.commit_message,
                // merge_method: request.merge_method
            };      
            match pull_request.merge_pr(merge_pr) {
                Ok( response ) => send_response(stream, response),
                Err( code_error ) => send_error(stream, code_error.to_string()),
            }
            return Ok(())
        //} //else {
          //  println!("Error al deserializar el mensaje: trailing characters");
          //  return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Error parsing request"))
        //};
    }

}