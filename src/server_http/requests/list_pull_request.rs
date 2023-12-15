
use std::net::TcpStream;

use serde::{Serialize, Deserialize};

use crate::{server_http::sender::{send_response, send_error}, pull_request::controllers::pull_request::PullRequest};

#[derive(Serialize, Deserialize)]
pub struct JsonListPR{
    pub status: Option<String>,
    pub head: Option<String>,
    pub base: Option<String>,
    pub per_page: Option<i32>,
    pub username: Option<String> 
}

#[derive(Serialize, Deserialize)]
pub struct ListPullRequests{
    pub base_repo: String,
    pub status: Option<String>,
    pub head: Option<String>,
    pub base: Option<String>,
    pub username: Option<String>,
    pub per_page: Option<i32> 
}


impl ListPullRequests {

    pub fn response_list_pull_request_object(json_body: &str, stream: &mut TcpStream,  base_repo: String, pull_request: PullRequest) -> Result<(), std::io::Error> {
        if let Ok(request) = serde_json::from_str::<JsonListPR>(json_body) {            
            let list = ListPullRequests {
                base_repo: base_repo,
                status: request.status,
                head: request.head,
                base: request.base,
                username: request.username,
                per_page: request.per_page,
            };
            match pull_request.find_all(list) {
                Ok( response ) => send_response(stream, response),
                Err( code_error ) => send_error(stream, code_error.to_string()),
            }
            return Ok(())
        } else {
            let list = ListPullRequests {
                base_repo: base_repo,
                status: None,
                head: None,
                base: None,
                username: None,
                per_page: None,
            };
            match pull_request.find_all(list) {
                Ok( response ) => send_response(stream, response),
                Err( code_error ) => send_error(stream, code_error.to_string()),
            }
            return Ok(())
        };
    }


}