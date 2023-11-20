use std::{fs::{OpenOptions, self}, io::Write, path::Path};

use crate::vcs::commands::init::Init;

use super::current_repository::CurrentRepository;

pub struct CurrentCommit;

impl CurrentCommit{
    pub fn read() -> Result<String, std::io::Error>{
        let current_repository = CurrentRepository::read()?;
        let head_path = Init::get_current_head(&current_repository)?;
        let hash = fs::read_to_string(head_path)?;
        Ok(hash)
    }

    pub fn write(hash: String) -> Result<String, std::io::Error>{
        let head_path = Init::get_current_head(&CurrentRepository::read()?)?;
        let mut head = OpenOptions::new().read(true).write(true).open(head_path)?;
        head.set_len(0)?;
        head.write_all(hash.as_bytes())?;
        Ok(hash)
    }
    
    pub fn read_for_branch(repo_path: &Path, branch: &str) -> Result<String, std::io::Error>{
        let head_path = repo_path.join(".rust_git").join("refs").join("heads").join(branch);
        let hash = fs::read_to_string(head_path)?;
        Ok(hash)
    }

    pub fn write_for_branch(repo_path: &Path, branch: &str, hash: String) -> Result<String, std::io::Error>{
        let head_path = repo_path.join(".rust_git").join("refs").join("heads").join(branch);
        let mut head = OpenOptions::new().read(true).write(true).open(head_path)?;
        head.set_len(0)?;
        head.write_all(hash.as_bytes())?;
        Ok(hash)
    }
}