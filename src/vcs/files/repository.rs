use std::{collections::HashMap, path::{Path, PathBuf}, fs::{OpenOptions, self}, io::{self, BufRead, Write}};

use rand::Rng;

use crate::vcs::{version_control_system::VersionControlSystem, commands::{cat_file::CatFile, hash_object::WriteOption}};


pub struct Repository{
    commits_path: PathBuf,
    object_path: PathBuf,
}

impl Repository{

    pub fn init(path: &str) -> Repository{
        let commits_path = Path::new(path).join(".rust_git").join("logs").join("commits");//?.display().to_string();  // VER EL PATH, NO SE SI ESTA BIEN ESTO
        let object_path = Path::new(path).join(".rust_git").join("objects");//?.display().toi_string();
        Repository{commits_path, object_path}
    }

    /// Leo el archivo commits donde esta la tabla y lo paso al HashMap del local_repository
     pub fn read_repository(&self) -> Result<HashMap<String,String>,std::io::Error>{
        let mut local_repository:HashMap<String, String>  = HashMap::new();
        let commits_file = OpenOptions::new().read(true).open(&self.commits_path)?;

        let reader = io::BufReader::new(commits_file);
        
        if let Some(last_commit) = reader.lines().filter_map(Result::ok).last(){
            let parts: Vec<&str> = last_commit.split("-").collect(); // parts[0] = id ; parts[1] = hash ; parts[2] = message

            let content = CatFile::cat_file(parts[1], self.object_path.clone().into())?;
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
    pub fn write_repository(&self,vcs: &VersionControlSystem, message: String, repository: &HashMap<String,String>) -> Result<(String, String, String), std::io::Error>{
        let mut rng = rand::thread_rng();
        let id = rng.gen_range(1..=100).to_string(); // Genera un número entre 1 y 100 (ambos inclusive)
        let path = Path::new(&vcs.path).join("temp");
        let mut commit_file = OpenOptions::new().write(true).create(true).append(true).open(&path)?; 
        for (key,value) in repository{
            commit_file.write_all(key.as_bytes())?;
            commit_file.write_all("-".as_bytes())?;
            commit_file.write_all(value.as_bytes())?;
            commit_file.write_all("\n".as_bytes())?;
        }
        let commit_hash = vcs.hash_object(&path, WriteOption::Write)?; 
        let _ = fs::remove_file(path);
        Ok((id,commit_hash,message))
    }


}

