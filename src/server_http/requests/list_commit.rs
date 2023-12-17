use crate::{
    pull_request::controllers::pull_request::PullRequest,
    server_http::{
        requests::get_pull_request::GetPullRequest,
        sender::{send_error, send_response},
    },
};
use std::net::TcpStream;

pub struct ListCommitsPullRequest;

impl ListCommitsPullRequest {
    pub fn list_commits_pull_request(
        stream: &mut TcpStream,
        pull_request: PullRequest,
        base_repo: String,
        id: String,
    ) -> Result<(), std::io::Error> {
        let list_commits = GetPullRequest { base_repo, id };

        match pull_request.find_commits(list_commits) {
            Ok(response) => send_response(stream, response),
            Err(error_code) => send_error(stream, error_code.to_string()),
        }

        Ok(())
    }
}
