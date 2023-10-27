use std::{fs::OpenOptions, self, io::{Write, self}, collections::HashMap};
use crate::{vcs::version_control_system::VersionControlSystem, utils::random::random::Random};
use super::init::Init;

extern crate chrono;
use chrono::{DateTime, Local};

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
        let id = Random::random();
        let hash = vcs.repository.write_repository(repository)?;
        let current_time: DateTime<Local> = Local::now();
        let _ = current_time.to_rfc2822();

        Ok(Commit {
            id,
            hash,
            message: message.clone(),
            timestamp: current_time,
        })
    }

    /// leo la tupla del commit actual y la escribo en la tabla ubicada en commits_file
    pub fn write_commit(vcs: &VersionControlSystem, commit: &Commit) -> Result<(), std::io::Error> {
        let id = commit.id.to_string(); 
        let hash = &commit.hash;
        let message = &commit.message;
        let current_time = commit.timestamp;

        let mut commits_file = OpenOptions::new().write(true).append(true).open(Init::get_commits_path(&vcs.path)?)?;

        let commit_info = format!("{}-{}-{}-{}\n", id, hash, message, current_time.to_rfc2822());
        commits_file.write_all(commit_info.as_bytes())?;
        Ok(())
    }
}