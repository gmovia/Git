use std::{path::Path, fs::{OpenOptions, self}, io::{Write, self, BufRead}};
use crate::constants::constant::{BDD_PATH, CURRENT_REPOSITORY_PATH};

pub struct Repositories;

impl Repositories{
    
    pub fn write(path: &Path) -> Result<(), std::io::Error>{
        let repositories = Self::read()?;
        if !repositories.contains(&path.to_string_lossy().to_string()){
            let bdd_path = Path::new(BDD_PATH);
            let mut bdd = OpenOptions::new().write(true).append(true).open(bdd_path)?; 
            bdd.write_all(format!("{}\n",path.to_string_lossy()).as_bytes())?;
        }
        let current_repository_path = Path::new(CURRENT_REPOSITORY_PATH); // DEFINIR LA CONSTANTE = "current_repository.txt"
        let mut current_repository = OpenOptions::new().write(true).append(true).open(current_repository_path)?; 
        current_repository.set_len(0)?;
        current_repository.write_all(format!("{}",path.to_string_lossy()).as_bytes())?;

        Ok(())
    }

    pub fn read() -> Result<Vec<String>, std::io::Error>{
        let mut repositories = Vec::new();
        let bdd_path = Path::new(BDD_PATH);
        let repo_file = OpenOptions::new().read(true).open(bdd_path)?;
        let reader = io::BufReader::new(repo_file);
        for line in reader.lines().map_while(Result::ok) {
            repositories.push(line);
        }
        Ok(repositories)
    }

    pub fn remove(path: &Path) -> Result<(), std::io::Error>{
        let mut repositories = Self::read()?;
        if !repositories.contains(&path.display().to_string()) {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Can't find the repository"));
        }
        let _ = fs::remove_dir_all(path);
        
        if let Some(index) = repositories.iter().position(|item| item == &path.display().to_string()) {
            repositories.remove(index);
        }
        let bdd_path = Path::new(BDD_PATH);
        let mut bdd = OpenOptions::new().write(true).append(true).open(bdd_path)?; 
        bdd.set_len(0)?;
        for repo in repositories {
            bdd.write_all(format!("{}\n",repo).as_bytes())?;
        }
        Ok(())
    }
}