use std::{path::Path, collections::HashMap};

use crate::{pull_request::{schemas::schemas::{CreatePullRequest, PullRequestEntry, FindPullRequests, FindPullRequest}, validator::validator::Validator, db::queries::Query}, vcs::{commands::{branch::Branch, merge::Merge}, files::current_commit::CurrentCommit}};

pub struct PullRequest;

impl PullRequest { 

    /// POST
    pub fn create(server: &Path, mut pr: CreatePullRequest) -> Result<String, std::io::Error>{
        Validator::validate_create_pull_request(server, &pr)?;

        let hash = CurrentCommit::read_for_branch(&server.join(&pr.base_repo), &pr.base)?;

        let base_repo = server.join(&pr.base_repo);
        let head_repo = server.join(&pr.head_repo);
        let _ = Branch::create_new_branch_with_hash(&base_repo, "temp_branch", &hash);

        let conflicts = Merge::merge_pr(&pr.username, &pr.head, "temp_branch", &head_repo,&base_repo, HashMap::new())?;
        if conflicts.is_empty(){
            pr.mergeable = true;
            pr.merge_commit_sha = Some(CurrentCommit::read_for_branch(&server.join(&pr.base_repo), "temp_branch")?);
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