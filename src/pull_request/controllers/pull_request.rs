use std::path::{Path, PathBuf};

use crate::{pull_request::{schemas::schema::{PullRequestEntry, CommitsPullRequest}, validators::validator::Validator, db::queries::Query}, vcs::commands::merge::Merge, server_http::requests::{create_pull_request::CreatePullRequest, list_pull_request::ListPullRequests, update_pull_request::UpdatePullRequest, merge_pull_request::MergePullRequest}};
use crate::server_http::requests::get_pull_request::GetPullRequest;

pub struct PullRequest{
    server: PathBuf
}

impl PullRequest { 

    pub fn init(server: &Path) -> PullRequest{
        PullRequest{ server: server.to_path_buf() }
    }
    
    pub fn find_all(&self, query: ListPullRequests) -> Result<Vec<PullRequestEntry>, std::io::Error>{
        let prs = Validator::validate_find_pull_requests(&self.server, &query)?; 
        Query::find_pull_requests(&prs, &query)
    }
    
    pub fn find_one(&self, query: GetPullRequest) -> Result<PullRequestEntry, std::io::Error> {
        let id = Validator::validate_find_a_pull_request(&self.server, &query)?;
        Query::find_a_pull_request(&id)
    }
    
    pub fn find_commits(&self, query: GetPullRequest) -> Result<Vec<CommitsPullRequest>, std::io::Error> {
        let id = Validator::validate_find_a_pull_request(&self.server, &query)?;
        Query::get_commits_pull_request(&self.server, &id)
    }
    
    pub fn create(&self, pr: &mut CreatePullRequest) -> Result<String, std::io::Error>{
        Validator::validate_create_pull_request(&self.server, pr)?;

        let base_repo = self.server.join(&pr.base_repo);
        let head_repo = self.server.join(&pr.head_repo);
        
        if !Merge::are_conflicts(&pr.head, &pr.base, &head_repo.clone(), &base_repo.clone())?{
            pr.mergeable = true;
        }

        let id = Query::create_pull_request(&self.server, pr)?;
    
        Ok(id)
    }

    pub fn merge_pr(&self, mut query: MergePullRequest) -> Result<String, std::io::Error> {
        let id = Validator::validate_merge_pr(&self.server, &query)?;
        if query.merge_method.is_none() {
            query.merge_method = Some("merge".to_string());
        }
        Query::merge_pull_request(&self.server, &id, &mut query.merge_method)
    }

    pub fn update(&self, pr: &UpdatePullRequest) -> Result<String, std::io::Error>{
        let id = Validator::validate_update_pull_request(&self.server, pr)?;
        Query::update_pull_request(&id, pr)
    }
}