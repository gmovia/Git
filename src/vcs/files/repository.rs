use std::{collections::HashMap, path::{Path, PathBuf}, fs::{OpenOptions, self}, io::{self, BufRead, Write}};
use crate::vcs::commands::{hash_object::{WriteOption, HashObject}, cat_file::CatFile, init::Init};

pub struct Repository{
    path: PathBuf,
}

impl Repository{


    pub fn init(path: PathBuf) -> Repository{
        Repository{path}
    }

    /// Leo el archivo commits donde esta la tabla y lo paso al HashMap del local_repository
     pub fn read_repository(&self) -> Result<HashMap<String,String>,std::io::Error>{
        let mut local_repository:HashMap<String, String>  = HashMap::new();
        let commits_file = OpenOptions::new().read(true).open(Init::get_commits_path(&self.path)?)?;

        let reader = io::BufReader::new(commits_file);
        
        if let Some(last_commit) = reader.lines().filter_map(Result::ok).last(){
            let parts: Vec<&str> = last_commit.split("-").collect(); // parts[0] = id ; parts[1] = hash ; parts[2] = message

            let content = CatFile::cat_file(parts[1], Init::get_object_path(&self.path)?)?;
            let content_lines: Vec<&str> = content.split("\n").collect();

            for line in content_lines{
                if line != ""{
                    let line_parts: Vec<&str> = line.split("-").collect(); // line_parts[0] = path ; line_parts[1] = content
                    local_repository.insert(line_parts[0].to_string(), line_parts[1].to_string());
                }
            }
        }
        
        Ok(local_repository)
    }

    /// leo del hashmap local repository y armo un archivo commit_file que es temporal del commit.
    /// Luego se lo mando al hash_object para que me genere su hash.
    /// Genero una tupla (id,commit_hash_message)
    pub fn write_repository(&self, repository: &HashMap<String,String>) -> Result<String, std::io::Error>{
        let path = Path::new(&self.path).join("temp");
        let mut commit_file = OpenOptions::new().write(true).create(true).append(true).open(&path)?; 
        for (key, value) in repository {
            let entry = format!("{}-{}\n", key, value);
            commit_file.write_all(entry.as_bytes())?;
        }
        let hash = HashObject::hash_object(&path, Init::get_object_path(&self.path)?, WriteOption::Write)?;
        let _ = fs::remove_file(path);
        Ok(hash)
    }
}

