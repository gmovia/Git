use std::path::Path;

use crate::pull_request::{schemas::schemas::{CreatePullRequest, PullRequestEntry, FindPullRequests, FindPullRequest}, validator::validator::Validator, db::queries::Query};

pub struct PullRequest;

impl PullRequest { 

    /// POST
    pub fn create(server: &Path, pr: CreatePullRequest) -> Result<String, String>{
        if let Err(err) = Validator::validate_create_pull_request(server, &pr){
            return Err(err.to_string());
        }

        if let Ok(id) = Query::create_pull_request(server, &pr){
            return Ok(id);
        }

        Err(String::from("500: Internal Server Error"))
    }

    /// GET ALL
    pub fn find_all(server: &Path, query: FindPullRequests) -> Result<Vec<PullRequestEntry>, String>{
        if let Err(err) = Validator::validate_find_pull_requests(server, &query){
            return Err(err.to_string());
        }

        if let Ok(prs) = Query::find_pull_requests(server, &query){
            return Ok(prs);
        }

        Err(String::from("500: Internal Server Error"))
    }

    /// GET
    pub fn find_one(server: &Path, query: FindPullRequest) -> Result<PullRequestEntry, String> {
        if let Err(err) = Validator::validate_find_a_pull_request(server, &query){
            return Err(err.to_string());
        }

        if let Ok(prs) = Query::find_a_pull_request(server, &query){
            return Ok(prs);
        }

        Err(String::from("500: Internal Server Error"))
    }
}