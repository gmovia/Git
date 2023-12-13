use std::{path::{Path, PathBuf}, io, fs};

use crate::{pull_request::utils::path::{get_pr_path, get_prs_path}, vcs::{commands::branch::Branch, files::commits_table::CommitsTable}, server_http::requests::{create_pull_request::CreatePullRequest, list_pull_request::ListPullRequests, update_pull_request::UpdatePullRequest}};
use crate::server_http::requests::get_pull_request::GetPullRequest;
pub struct Validator;

impl Validator{

    pub fn validate_create_pull_request(server: &Path, pr: &CreatePullRequest) -> Result<(), std::io::Error>{
        let base_repo = server.join(&pr.base_repo);
        let head_repo = server.join(&pr.head_repo);
        let base_commits_table = CommitsTable::read(base_repo.clone(), &pr.base)?;
        let head_commits_table = CommitsTable::read(head_repo.clone(), &pr.head)?;
        let current_commit_base = base_commits_table.last();
        let current_commit_head = head_commits_table.last();
        if let ( Some(commit_base), Some(commit_head) ) = (current_commit_base, current_commit_head) {
            if commit_base.hash == commit_head.hash {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "422 Head and Base only shared equal commits",
                ));
            }                
        }    
        Self::validate_creation_pr(&server, &pr)?;
        Self::validate_repo(&base_repo)?;
        Self::validate_repo(&head_repo)?;
        Self::validate_branch(&base_repo, &pr.base)?;
        Self::validate_branch(&head_repo, &pr.head)?;
        Ok(())
    }
    
    pub fn validate_update_pull_request(server: &Path, pr: &UpdatePullRequest) -> Result<PathBuf, std::io::Error>{
        let base_repo = server.join(&pr.base_repo);
        let id = get_pr_path(server, &pr.base_repo, &pr.id);
        Self::validate_id(&id)?;
        Self::validate_repo(&base_repo)?;
        if let Some(base) = &pr.base{
            Self::validate_branch(&base_repo, &base)?;
        }
        Ok(id)
    }

    pub fn validate_find_pull_requests(server: &Path, query: &ListPullRequests) -> Result<PathBuf, std::io::Error>{
        let base_repo = server.join(&query.base_repo);
        Self::validate_repo(&base_repo)?;
        Ok(get_prs_path(server, &query.base_repo)) // Si pasa la validacion, devuelve la ruta que tiene todos los PRs
    }

    pub fn validate_find_a_pull_request(server: &Path, query: &GetPullRequest) -> Result<PathBuf, std::io::Error> {
        let base_repo = server.join(&query.base_repo);
        let id = get_pr_path(server, &query.base_repo, &query.id);
        Self::validate_id(&id)?;
        Self::validate_repo(&base_repo)?;
        Ok(id) // Si pasa la validacion, devuelve la ruta del PR
    }

    pub fn validate_creation_pr(server: &Path, pr: &CreatePullRequest) -> Result<(), std::io::Error> {
        let prs_path = server.join("pull_requests").join(&pr.base_repo);
        if let Ok(entries) = fs::read_dir(prs_path) {
            for entry in entries.flatten() {
                let content = fs::read_to_string(entry.path())?;
                let parts: Vec<&str> = content.split('\n').collect();
                if parts[0] == "PR" {
                    if parts[8] == "open" && 
                    parts[3] == pr.head_repo.as_str() && 
                    parts[4] == pr.base_repo.as_str() && 
                    parts[5] == pr.head.as_str() && 
                    parts[6] == pr.base.as_str(){
                        return Err(io::Error::new(
                            io::ErrorKind::Other,
                            "403 The requested pr has already been created",
                        ));
                }
                }
                
            }
        }
        Ok(())
    }

    pub fn validate_repo(repo: &Path) -> Result<(), std::io::Error>{ 
        if !repo.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "422 Can't find the repository",
            ));
        }
        Ok(())
    }

    pub fn validate_id(id: &Path) -> Result<(), std::io::Error> {
        if !id.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "404 Id not found",
            ));
        }
        Ok(())
        }

    pub fn validate_branch(repo: &Path, branch: &str) -> Result<(), std::io::Error>{
        let branches = Branch::get_branches(&repo)?;
        if !branches.contains(&branch.to_string()){
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "422 Can't find the branch",
            ));
        }
        Ok(())
    }
}
