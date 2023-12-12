
use std::net::TcpStream;

use serde::{Serialize, Deserialize};

use crate::{server_http::validation::{send_response, send_error}, pull_request::controllers::pull_request::PullRequest};


#[derive(Serialize, Deserialize)]
pub struct GetPullRequest{
    pub base_repo: String,
    pub id: String,
}


impl GetPullRequest {

    pub fn get_pull_request(stream: &mut TcpStream, pull_request: PullRequest, base_repo: String, id: String) -> Result<(), std::io::Error> {
        match pull_request.find_one(GetPullRequest { base_repo, id }) {
            Ok( response ) => { send_response(stream, response)},
            Err( error_code) => { send_error(stream, error_code.to_string())}
        };
        Ok(()) 
    }


}