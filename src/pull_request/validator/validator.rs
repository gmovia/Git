use std::{path::Path, io};

use crate::{pull_request::schemas::schemas::CreatePullRequest, vcs::commands::branch::Branch};

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

    /// valida si la branch existe
    pub fn validate_branch(repo: &Path, branch: &str) -> Result<(), std::io::Error>{
        let branches = Branch::get_branches(&repo)?;
        if !branches.contains(&branch.to_string()){
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Can't find the branch",
            ));
        }
        Ok(())
    }

    /// valida si el repositorio existe
    pub fn validate_repo(repo: &Path) -> Result<(), std::io::Error>{ 
        if !repo.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Can't find the repository",
            ));
        }
        Ok(())
    }
}