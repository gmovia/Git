use std::{fs::OpenOptions, self, io::Write, collections::HashMap};
use crate::{vcs::version_control_system::VersionControlSystem, utils::random::random::Random};
use super::init::Init;

pub struct Commit;

impl Commit{

    pub fn commit(vcs: & VersionControlSystem, message: String) -> Result<HashMap<String, String>, std::io::Error>{
        let mut repository = vcs.repository.read_repository()?;
        let staging_area = vcs.index.read_index()?;
        
        for (key, value) in &staging_area{
            match value.clone().state.as_str(){
                "CREATED" => {repository.insert(key.to_string(), value.clone().content);},
                "MODIFIED" => {repository.insert(key.to_string(), value.clone().content);},
                "DELETED" => {repository.remove(key);},
                _ => {}
            }
        }

        let _ = Commit::write_commit(vcs, &message, &repository)?;
        let _ = vcs.index.clear(); //limpio el index
        Ok(repository)
    }
    

    /// leo la tupla del commit actual y la escribo en la tabla ubicada en commits_file
    pub fn write_commit(vcs: &VersionControlSystem, message: &String, repository: &HashMap<String, String>) -> Result<(),std::io::Error>{
        let id = Random::random();
        let hash = vcs.repository.write_repository(&repository)?;
        let mut commits_file = OpenOptions::new().write(true).append(true).open(Init::get_commits_path(&vcs.path)?)?; //abro la tabla de commits para escribir - si no existe, la creo
        let commit = format!("{}-{}-{}\n", id, hash, message);
        commits_file.write_all(commit.as_bytes())?;
        Ok(())
    }
}