use std::net::TcpStream;

use serde::{Deserialize, Serialize};

use crate::{
    pull_request::controllers::pull_request::PullRequest,
    server_http::sender::{send_error, send_response},
};

#[derive(Serialize, Deserialize)]
pub struct GetPullRequest {
    pub base_repo: String,
    pub id: String,
}

impl GetPullRequest {
    pub fn get_pull_request(
        stream: &mut TcpStream,
        pull_request: PullRequest,
        base_repo: String,
        id: String,
    ) -> Result<(), std::io::Error> {
        match pull_request.find_one(GetPullRequest { base_repo, id }) {
            Ok(response) => send_response(stream, response),
            Err(error_code) => send_error(stream, error_code.to_string()),
        };
        Ok(())
    }
}
