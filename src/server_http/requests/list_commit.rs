use std::net::TcpStream;
use crate::{pull_request::controllers::pull_request::PullRequest, server_http::{requests::get_pull_request::GetPullRequest, sender::send_response}};

pub struct ListCommitsPullRequest;

impl ListCommitsPullRequest {

    pub fn list_commits_pull_request(stream: &mut TcpStream, pull_request: PullRequest, base_repo: String, id: String) -> Result<(), std::io::Error> {              
        let list_commits = GetPullRequest {
            base_repo,
            id,
        };
        
        let reponse = pull_request.find_commits(list_commits)?;
        send_response(stream, reponse);
        Ok(())
        
    }


}