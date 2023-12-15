
use std::net::TcpStream;

use serde::{Serialize, Deserialize};

use crate::{server_http::sender::{send_response, send_error, send_server_error_msg}, pull_request::controllers::pull_request::PullRequest};

#[derive(Serialize, Deserialize)]
pub struct JsonUpdatePR{
    pub title: Option<String>,
    pub body: Option<String>,
    pub status: Option<String>,
    pub base: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdatePullRequest{
    pub id: String,
    pub base_repo: String,
    pub title: Option<String>,
    pub body: Option<String>,
    pub status: Option<String>,
    pub base: Option<String>,
}


impl UpdatePullRequest {

    pub fn update_pull_request(json_body: &str, stream: &mut TcpStream,  base_repo: String, id: String, pull_request: PullRequest) -> Result<(), std::io::Error> {
        if let Ok(request) = serde_json::from_str::<JsonUpdatePR>(json_body) {            
            let update_pr = UpdatePullRequest {
                id,
                base_repo,
                title: request.title,
                body: request.body,
                status: request.status,
                base: request.base,
            };
            match pull_request.update(&update_pr) {
                Ok( response ) => send_response(stream, response),
                Err( code_error ) => send_error(stream, code_error.to_string()),
            }
            return Ok(())
        } else {
            send_server_error_msg(stream);
            return Ok(())
        };
    }


}