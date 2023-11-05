use std::{fs::{OpenOptions, self}, self, io::{Write, self}, collections::HashMap, path::{PathBuf, Path}};
use crate::{vcs::{version_control_system::VersionControlSystem, files::{repository::Repository, index::Index}}, utils::random::random::Random, constants::constants::{STATE_CREATED, STATE_MODIFIED, STATE_DELETED}};
use super::{init::Init, hash_object::{HashObject, WriteOption}};

extern crate chrono;
use chrono::{DateTime, Local};

pub struct Commit;

impl Commit{

    pub fn commit(message: String) -> Result<HashMap<String, String>, std::io::Error>{
        let mut repository = Repository::read_repository()?;
        let staging_area = Index::read_index()?;
        if staging_area.is_empty(){
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "The staging area is empty, you need to add before commit"));
        }
        for (key, value) in &staging_area{
            match value.clone().state.as_str(){
                STATE_CREATED => {repository.insert(key.to_string(), value.clone().content);},
                STATE_MODIFIED => {repository.insert(key.to_string(), value.clone().content);},
                STATE_DELETED => {repository.remove(key);},
                _ => {}
            }
        }

        let _ = Commit::write_commit(&message, &repository)?;
        let _ = Index::clear(); //limpio el index
        Ok(repository)
    }
    

    /// leo la tupla del commit actual y la escribo en la tabla ubicada en commits_file
    pub fn write_commit(message: &String, repository: &HashMap<String, String>) -> Result<(),std::io::Error>{
        let id = Random::random();
        let hash = Repository::write_repository(&repository)?;
        let current = VersionControlSystem::read_current_repository()?;
        let mut commits_file = OpenOptions::new().write(true).append(true).open(Init::get_commits_path(&current)?)?; //abro la tabla de commits para escribir - si no existe, la creo

        let current_time: DateTime<Local> = Local::now();
        let _ = current_time.to_rfc2822();

        let tree_hash = Self::create_tree(&current, &hash)?;


        let commit = format!("{}-{}-{}-{}-{}\n", id, hash, message, current_time, tree_hash); 
        commits_file.write_all(commit.as_bytes())?;
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