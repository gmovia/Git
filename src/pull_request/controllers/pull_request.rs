use std::{path::Path, collections::HashMap};

use crate::{pull_request::{schemas::schemas::{CreatePullRequest, PullRequestEntry, FindPullRequests, FindPullRequest}, validator::validator::Validator, db::queries::Query}, vcs::{commands::{branch::Branch, merge::Merge}, files::current_commit::CurrentCommit}};

pub struct PullRequest;

impl PullRequest { 

    /// POST
    pub fn create(server: &Path, mut pr: CreatePullRequest) -> Result<String, std::io::Error>{
        Validator::validate_create_pull_request(server, &pr)?;

        let base_repo = server.join(&pr.base_repo);
        let head_repo = server.join(&pr.head_repo);

        if !Merge::are_conflicts(&pr.head, &pr.base, &head_repo, &base_repo)?{
            pr.mergeable = true;
        }

        Query::create_pull_request(server, &pr)
    }

    /// GET ALL
    pub fn find_all(server: &Path, query: FindPullRequests) -> Result<Vec<PullRequestEntry>, std::io::Error>{
        Validator::validate_find_pull_requests(server, &query)?;
        Query::find_pull_requests(server, &query)
    }

    /// GET
    pub fn find_one(server: &Path, query: FindPullRequest) -> Result<PullRequestEntry, std::io::Error> {
        Validator::validate_find_a_pull_request(server, &query)?;
        Query::find_a_pull_request(server, &query)
    }
}