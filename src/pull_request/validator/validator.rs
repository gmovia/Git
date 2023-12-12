use std::{path::{Path, PathBuf}, io, fs};

use crate::{pull_request::utils::path::{get_pr_path, get_prs_path}, vcs::commands::branch::Branch, server_http::requests::{create_pull_request::CreatePullRequest, list_pull_request::ListPullRequests}};
use crate::server_http::requests::get_pull_request::GetPullRequest;
pub struct Validator;

impl Validator{

    pub fn validate_create_pull_request(server: &Path, pr: &CreatePullRequest) -> Result<(), std::io::Error>{
        let base_repo = server.join(&pr.base_repo);
        let head_repo = server.join(&pr.head_repo);

        Self::validate_creation_pr(&server, &pr)?;
        Self::validate_repo(&base_repo)?;
        Self::validate_repo(&head_repo)?;
        Self::validate_branch(&base_repo, &pr.base)?;
        Self::validate_branch(&head_repo, &pr.head)?;
        Ok(())
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
