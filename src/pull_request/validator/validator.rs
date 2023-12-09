use std::{path::Path, io};

use crate::{pull_request::schemas::schemas::{CreatePullRequest, FindPullRequests, FindPullRequest}, vcs::commands::branch::Branch};

pub struct Validator;

impl Validator{

    /// Hace las validaciones pertinentes
    pub fn validate_create_pull_request(server: &Path, pr: &CreatePullRequest) -> Result<(), std::io::Error>{
        let base_repo = server.join(&pr.base_repo);
        let head_repo = server.join(&pr.head_repo);

        Self::validate_repo(&base_repo)?; 
        Self::validate_repo(&head_repo)?;
        Self::validate_branch(&base_repo, &pr.base)?;
        Self::validate_branch(&head_repo, &pr.head)?;
        Ok(())
    }

    pub fn validate_find_pull_requests(server: &Path, query: &FindPullRequests) -> Result<(), std::io::Error>{
        let base_repo = server.join(&query.base_repo);
        Self::validate_repo(&base_repo)
    }

    /// valida si la branch existe
    pub fn validate_branch(repo: &Path, branch: &str) -> Result<(), std::io::Error>{
        let branches = Branch::get_branches(&repo)?;
        if !branches.contains(&branch.to_string()){
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "422: Can't find the branch",
            ));
        }
        Ok(())
    }

    /// valida si el repositorio existe
    pub fn validate_repo(repo: &Path) -> Result<(), std::io::Error>{ 
        if !repo.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "422: Can't find the repository",
            ));
        }
        Ok(())
    }

    pub fn validate_id(id: &Path) -> Result<(), std::io::Error> {
        if !id.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "404: Id not found",
            ));
        }
        Ok(())
        }

    pub fn validate_find_a_pull_request(server: &Path, query: &FindPullRequest) -> Result<(), std::io::Error> {
        let base_repo = server.join(&query.base_repo);
        let id = server.join("pull_requests").join(&query.base_repo).join(&query.id);
        Self::validate_id(&id)?;
        Self::validate_repo(&base_repo)?;
        Ok(())
    }
}
