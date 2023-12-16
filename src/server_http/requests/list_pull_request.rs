
use std::net::TcpStream;

use serde::{Serialize, Deserialize};

use crate::{server_http::sender::{send_response, send_error}, pull_request::controllers::pull_request::PullRequest};

#[derive(Serialize, Deserialize)]
pub struct StructListPR{
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

    pub fn response_list_pull_request_object(input_body: &str, stream: &mut TcpStream,  base_repo: String, pull_request: PullRequest, media_type: &str) -> Result<(), std::io::Error> {
        
        if media_type == "application/json"{
            if let Ok(request) = serde_json::from_str::<StructListPR>(input_body) {
                let list = ListPullRequests {
                    base_repo,
                    status: request.status,
                    head: request.head,
                    base: request.base,
                    username: request.username,
                    per_page: request.per_page,
                };

                match pull_request.find_all(list){
                    Ok(response) => send_response(stream, response),
                    Err(error_code) => send_error(stream, error_code.to_string())
                }

            };
            Ok(())
        }
        else if media_type == "application/xml" { 
            if let Ok(request) = serde_xml_rs::from_str::<StructListPR>(input_body) {
                let list = ListPullRequests {
                    base_repo,
                    status: request.status,
                    head: request.head,
                    base: request.base,
                    username: request.username,
                    per_page: request.per_page,
                };

                match pull_request.find_all(list){
                    Ok(response) => send_response(stream, response),
                    Err(error_code) => send_error(stream, error_code.to_string())
                }
                
            };
            Ok(())
        } else {
            let list = ListPullRequests {
                base_repo,
                status: None,
                head: None,
                base: None,
                username: None,
                per_page: None,
            };

            match pull_request.find_all(list){
                Ok(response) => send_response(stream, response),
                Err(error_code) => send_error(stream, error_code.to_string())
            }

            Ok(())
        }
    }
}