use std::net::TcpStream;

use serde::{Serialize, Deserialize};

use crate::{pull_request::controllers::pull_request::PullRequest, server_http::sender::{send_response, send_error}};

#[derive(Serialize, Deserialize)]
pub struct StructListPR{
    commit_tittle: Option<String>,
    commit_message: Option<String>,
    merge_method: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct MergePullRequest{
    pub base_repo: String,
    pub id: String,
    pub commit_tittle: Option<String>,
    pub commit_message: Option<String>,
    pub merge_method: Option<String>
}


impl MergePullRequest {

    pub fn merge_pull_request(input_body: &str, stream: &mut TcpStream, base_repo: String, id: String, pull_request: PullRequest, media_type: String) -> Result<(), std::io::Error> {

        if media_type == "application/json"{
            if let Ok(request) = serde_json::from_str::<StructListPR>(input_body) {      
                let merge_pr = MergePullRequest {
                    base_repo,
                    id,
                    commit_tittle: request.commit_tittle,
                    commit_message: request.commit_message,
                    merge_method: request.merge_method
                };

                match pull_request.merge_pr(merge_pr) {
                    Ok( response ) => send_response(stream, response),
                    Err( code_error ) => send_error(stream, code_error.to_string()),
                }
            }
        }else if media_type == "application/xml" { 
            if let Ok(request) = serde_xml_rs::from_str::<StructListPR>(input_body) { 
                let merge_pr = MergePullRequest {
                    base_repo,
                    id,
                    commit_tittle: request.commit_tittle,
                    commit_message: request.commit_message,
                    merge_method: request.merge_method
                };

                match pull_request.merge_pr(merge_pr) {
                    Ok( response ) => send_response(stream, response),
                    Err( code_error ) => send_error(stream, code_error.to_string()),
                }
            }
        }
        else {
            let merge_pr = MergePullRequest {
                base_repo,
                id,
                commit_tittle: None,
                commit_message: None,
                merge_method: None
            };
            match pull_request.merge_pr(merge_pr) {
                Ok( response ) => send_response(stream, response),
                Err( code_error ) => send_error(stream, code_error.to_string()),
            }
        }
        Ok(())   
    }

}