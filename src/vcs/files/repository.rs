use std::{collections::HashMap, path::PathBuf, fs::OpenOptions, io::{self, BufRead}};
use crate::vcs::commands::init::Init;
use crate::vcs::entities::{commit_entity::CommitEntity, tree_entity::TreeEntity, blob_entity::BlobEntity};
use crate::constants::constants::BLOB_CODE;
use super::{commits_table::CommitsTable, current_repository::CurrentRepository};

#[derive(Debug, Clone)]
pub struct Repository;

impl Repository{

     pub fn read_repository() -> Result<HashMap<String,String>,std::io::Error>{
        let current_path = CurrentRepository::read()?;
        let mut local_repository: HashMap<String, String>  = HashMap::new();
        
        let commits_file = OpenOptions::new().read(true).open(Init::get_commits_path(&current_path)?)?;
        let reader = io::BufReader::new(commits_file);
        
        if let Some(last_commit) = reader.lines().filter_map(Result::ok).last(){
            let parts: Vec<&str> = last_commit.split("-").collect(); // parts[0] = id ; parts[1] = hash ; parts[2] = message ; parts[3] = date ; parts[4] = tree
            local_repository.extend(Repository::read_repository_of_commit(current_path.clone(), &Init::get_current_branch(&current_path)?, parts[1])?);
        }
        
        Ok(local_repository)
    }

    pub fn read_repository_of_commit(repo_path: PathBuf, branch: &str, commit_hash: &str) -> Result<HashMap<String, String>,std::io::Error>{
        let commits_table = CommitsTable::read(repo_path.clone(), branch)?;

        for commit in commits_table {
            if commit.hash == commit_hash {
                let commit_entity = CommitEntity::read(&repo_path, commit_hash)?; 
                
                let blobs  = TreeEntity::read(&repo_path, commit_entity.tree_hash)?;
                
                return Ok(Self::convert_to_repository(&blobs));
            }
        }
        Ok(HashMap::new())
    }
    
    pub fn convert_to_repository(blobs: &Vec<BlobEntity>) -> HashMap<String, String>{
        let mut local_repository: HashMap<String, String> = HashMap::new();
        for blob in blobs{
            local_repository.insert(blob.path.clone(), blob.blob_hash.clone());
        }
        local_repository
    }

    pub fn convert_to_blobs(repository: &HashMap<String,String>) -> Vec<BlobEntity>{
        let mut blobs: Vec<BlobEntity> = Vec::new();
        for (key, value) in repository{
            let blob = BlobEntity{content_type: BLOB_CODE.to_string(), path: key.to_string(), blob_hash: value.to_string()};
            blobs.push(blob)
        }
        blobs       
    }
}

