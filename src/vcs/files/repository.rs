use std::{collections::HashMap, path::PathBuf, fs::OpenOptions, io::{self, BufRead}};
use crate::vcs::{commands::init::Init, entities::entity::convert_to_repository};
use crate::vcs::entities::{commit_entity::CommitEntity, tree_entity::TreeEntity};
use super::{commits_table::CommitsTable, current_repository::CurrentRepository, current_commit::CurrentCommit};

#[derive(Debug, Clone)]
pub struct Repository;

impl Repository{

     pub fn read_repository() -> Result<HashMap<String,String>,std::io::Error>{
        let current_path = CurrentRepository::read()?;
        let current_branch = &Init::get_current_branch(&current_path)?;
        
        let current_commit_hash = CurrentCommit::read()?;
        
        let mut local_repository: HashMap<String, String>  = HashMap::new();
        local_repository.extend(Repository::read_repository_of_commit(current_path.clone(), &current_branch, &current_commit_hash)?);
        Ok(local_repository)
    }

    pub fn read_repository_of_commit(repo_path: PathBuf, branch: &str, commit_hash: &str) -> Result<HashMap<String, String>,std::io::Error>{
        let commits_table = CommitsTable::read(repo_path.clone(), branch)?;

        for commit in commits_table {
            if commit.hash == commit_hash {
                let commit_entity = CommitEntity::read(&repo_path, commit_hash)?; 
                
                let entities  = TreeEntity::read(&repo_path, commit_entity.tree_hash)?;

                return Ok(convert_to_repository(&entities, CurrentRepository::read()?));
            }
        }
        Ok(HashMap::new())
    }
}

