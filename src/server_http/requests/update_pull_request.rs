use serde::{Deserialize, Serialize};
use std::net::TcpStream;

// Agrega la nueva dependencia
use serde_xml_rs;

use crate::{
    pull_request::controllers::pull_request::PullRequest,
    server_http::sender::{send_error, send_response, send_server_error_msg},
};

#[derive(Serialize, Deserialize)]
pub struct StructUpdatePR {
    pub title: Option<String>,
    pub body: Option<String>,
    pub status: Option<String>,
    pub base: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdatePullRequest {
    pub id: String,
    pub base_repo: String,
    pub title: Option<String>,
    pub body: Option<String>,
    pub status: Option<String>,
    pub base: Option<String>,
}

impl UpdatePullRequest {
    pub fn update_pull_request(
        body: &str,
        stream: &mut TcpStream,
        base_repo: String,
        id: String,
        pull_request: PullRequest,
        media_type: &str,
    ) -> Result<(), std::io::Error> {
        if media_type == "application/json" {
            if let Ok(request) = serde_json::from_str::<StructUpdatePR>(body) {
                let update_pr = UpdatePullRequest {
                    id,
                    base_repo,
                    title: request.title,
                    body: request.body,
                    status: request.status,
                    base: request.base,
                };
                match pull_request.update(&update_pr) {
                    Ok(response) => send_response(stream, response),
                    Err(code_error) => send_error(stream, code_error.to_string()),
                }
                return Ok(());
            }
        } else if media_type == "application/xml" {
            if let Ok(request) = serde_xml_rs::from_str::<StructUpdatePR>(body) {
                let update_pr = UpdatePullRequest {
                    id,
                    base_repo,
                    title: request.title,
                    body: request.body,
                    status: request.status,
                    base: request.base,
                };
                match pull_request.update(&update_pr) {
                    Ok(response) => send_response(stream, response),
                    Err(code_error) => send_error(stream, code_error.to_string()),
                }
                return Ok(());
            }
        }
        send_server_error_msg(stream);
        Ok(())
    }
}
