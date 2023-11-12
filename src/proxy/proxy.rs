use std::path::PathBuf;
use crate::vcs::entities::{blob_entity::BlobEntity, tree_entity::TreeEntity, commit_entity::CommitEntity, entity::Entity};

pub struct Proxy;

impl Proxy{

    pub fn write_commit(repo_path: PathBuf, commit: CommitEntity) -> Result<String, std::io::Error>{
        Ok(CommitEntity::write(&repo_path, &commit)?)
    }

    pub fn read_commit(repo_path: PathBuf, commit_hash: String) -> Result<CommitEntity, std::io::Error>{
        Ok(CommitEntity::read(&repo_path, &commit_hash)?)
    }

    pub fn write_tree(repo_path: PathBuf, entities: Vec<Entity>) -> Result<String, std::io::Error>{
        Ok(TreeEntity::write(&repo_path, &entities)?)
    } 

    pub fn read_tree(repo_path: PathBuf, tree_hash: String) -> Result<Vec<Entity>, std::io::Error>{
        Ok(TreeEntity::read(&repo_path, tree_hash)?)
    } 
    
    pub fn write_blob(repo_path: PathBuf, content: &String) -> Result<String, std::io::Error>{
        Ok(BlobEntity::write(repo_path, content)?)
    }

    pub fn read_blob(repo_path: PathBuf, blob_hash: String) -> Result<String, std::io::Error>{
        Ok(BlobEntity::read(repo_path, blob_hash)?)
    } 
}


