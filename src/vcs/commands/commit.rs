use std::{fs::{OpenOptions, File}, self, io::Write, collections::HashMap};

use crate::vcs::version_control_system::VersionControlSystem;

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

        let commit = vcs.repository.write_repository(vcs, message, &repository)?;
        let _ = Commit::write_commit(vcs,commit)?;
        let _ = vcs.index.clear(); //limpio el index
        Ok(repository)
    }
    
    

    /// leo la tupla del commit actual y la escribo en la tabla ubicada en commits_file
    pub fn write_commit(vcs: &VersionControlSystem,commit: (String, String, String)) -> Result<(),std::io::Error>{
        let mut commits_file = OpenOptions::new().write(true).append(true).open(Init::get_commits_path(&vcs.path)?)?; //abro la tabla de commits para escribir - si no existe, la creo
        let _ = Commit::add(&mut commits_file, commit)?;
        Ok(())
    }

    /// Escribo la tabla de commits --> Formato: id-hash object del commit-mensaje
    pub fn add(commits: &mut File, commit: (String, String, String))-> Result<(),std::io::Error> {
        commits.write_all(&commit.0.as_bytes())?;
        commits.write_all("-".as_bytes())?;
        commits.write_all(commit.1.as_bytes())?;
        commits.write_all("-".as_bytes())?;
        commits.write_all(commit.2.as_bytes())?;
        commits.write_all("\n".as_bytes())?;
        Ok(())
    }

}