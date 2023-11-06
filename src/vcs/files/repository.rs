use std::{collections::HashMap, path::Path, fs::{OpenOptions, self}, io::{self, BufRead, Write}};
use crate::vcs::{commands::{hash_object::{WriteOption, HashObject},  init::Init}, version_control_system::VersionControlSystem};

use super::commits_table::CommitsTable;

#[derive(Debug, Clone)]
pub struct Repository;

impl Repository{

    /// Leo el archivo commits donde esta la tabla y lo paso al HashMap del local_repository
    /// Nos quedamos con la ultima entrada de la tabla de commits del archivo commits_file
     pub fn read_repository() -> Result<HashMap<String,String>,std::io::Error>{
        let current_path = VersionControlSystem::read_current_repository()?;
        let mut local_repository:HashMap<String, String>  = HashMap::new();
        
        let commits_file = OpenOptions::new().read(true).open(Init::get_commits_path(&current_path)?)?;

        let reader = io::BufReader::new(commits_file);
        
        if let Some(last_commit) = reader.lines().filter_map(Result::ok).last(){
            let parts: Vec<&str> = last_commit.split("-").collect(); // parts[0] = id ; parts[1] = hash ; parts[2] = message ; parts[3] = date ; parts[4] = tree
            local_repository.extend(CommitsTable::read_repository_of_commit(current_path.clone(), &Init::get_current_branch(&current_path)?, parts[1])?);
        }
        
        Ok(local_repository)
    }

    /// leo del hashmap local repository y armo un archivo commit_file que es temporal del commit.
    /// Luego se lo mando al hash_object para que me genere su hash.
    /// Genero una tupla (id,commit_hash_message)
    pub fn write_repository(repository: &HashMap<String,String>) -> Result<String, std::io::Error>{
        let current_path = VersionControlSystem::read_current_repository()?;

        let path = Path::new(&current_path).join("temp");
        let mut commit_file = OpenOptions::new().write(true).create(true).append(true).open(&path)?; 
        for (key, value) in repository {
            let entry = format!("{}-{}\n", key, value);
            commit_file.write_all(entry.as_bytes())?;
        }
        let hash = HashObject::hash_object(&path, Init::get_object_path(&current_path, ".rust_git")?, WriteOption::Write)?;
        let _ = fs::remove_file(path);
        Ok(hash)
    }
}
