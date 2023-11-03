use std::{fs::{OpenOptions, self}, self, io::{Write, self}, collections::HashMap, path::{PathBuf, Path}};
use crate::{vcs::version_control_system::VersionControlSystem, utils::random::random::Random};
use super::{init::Init, hash_object::{HashObject, WriteOption}};

extern crate chrono;
use chrono::{DateTime, Local};
use rand::Rng;

pub struct Commit {
    id: String,
    hash: String,
    message: String,
    timestamp: DateTime<Local>,
}

impl Commit{

    pub fn commit(vcs: &VersionControlSystem, message: String) -> Result<HashMap<String, String>, std::io::Error>{
        let mut repository = vcs.repository.read_repository()?;
        let staging_area = vcs.index.read_index()?;
        if staging_area.is_empty(){
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "The staging area is empty, you need to add before commit"));
        }
        for (key, value) in &staging_area{
            match value.clone().state.as_str(){
                "CREATED" => {repository.insert(key.to_string(), value.clone().content);},
                "MODIFIED" => {repository.insert(key.to_string(), value.clone().content);},
                "DELETED" => {repository.remove(key);},
                _ => {}
            }
        }
        let commit = Commit::create_commit(&message, &repository, vcs)?;
        Commit::write_commit(vcs, &commit)?;
        let _ = vcs.index.clear(); //limpio el index
        Ok(repository)
    }
    

    pub fn create_commit(message: &String, repository: &HashMap<String, String>, vcs: &VersionControlSystem) -> Result<Commit, std::io::Error> {
        let mut rng = rand::thread_rng();
        let id = rng.gen_range(1..9);
        let hash = vcs.repository.write_repository(repository)?;
        let current_time: DateTime<Local> = Local::now();
        let _ = current_time.to_rfc2822();

        Ok(Commit {
            id: id.to_string(),
            hash,
            message: message.clone(),
            timestamp: current_time,
        })
    }

    /// leo la tupla del commit actual y la escribo en la tabla ubicada en commits_file
    pub fn write_commit(vcs: &VersionControlSystem, commit: &Commit) -> Result<(), std::io::Error> {
        let id = commit.id.to_string(); 
        let repository_hash = &commit.hash;
        let message = &commit.message;
        let current_time = commit.timestamp;
        
        let mut commits_file = OpenOptions::new().write(true).append(true).open(Init::get_commits_path(&vcs.path)?)?;
        let tree_hash = Self::create_tree(&vcs.path, repository_hash)?;

        let commit_info = format!("{}-{}-{}-{}-{}\n", id, repository_hash, message, current_time.to_rfc2822(), tree_hash);
        commits_file.write_all(commit_info.as_bytes())?;
        Ok(())
    }

    pub fn create_tree(path: &PathBuf, hash: &String) -> Result<String, std::io::Error>{
        let temp_path = Path::new(&path).join("temp");
        let mut tree_file = OpenOptions::new().write(true).create(true).append(true).open(&temp_path)?; 
    
        let entry = format!("tree-{}\n", hash);
        tree_file.write_all(entry.as_bytes())?;

        let hash = HashObject::hash_object(&temp_path, Init::get_object_path(&path, ".rust_git")?, WriteOption::Write)?;
        let _ = fs::remove_file(temp_path);
        Ok(hash)
    }
}


