use std::{path::{PathBuf, Path}, fs::{OpenOptions, self}, io::Write};
use crate::{vcs::commands::{hash_object::{HashObject, WriteOption}, init::Init, cat_file::CatFile}, utils::random::random::Random};

pub struct CommitEntity{
    pub content_type: String,
    pub tree_hash: String
}

impl CommitEntity{

    pub fn write(repo_path: &PathBuf, tree_hash: &String) -> Result<String, std::io::Error>{
        let commit_path = Path::new(&repo_path).join(Random::random());
        let mut commit_file = OpenOptions::new().write(true).create(true).append(true).open(&commit_path)?; 

        let entry = format!("tree {}\n", tree_hash);
        commit_file.write_all(entry.as_bytes())?;
        
        let commit_hash = HashObject::hash_object(&commit_path, Init::get_object_path(&repo_path)?, WriteOption::Write)?;
        
        let _ = fs::remove_file(commit_path);
        Ok(commit_hash)
    }
    
    pub fn read(repo_path: &PathBuf, commit_hash: &str) -> Result<CommitEntity, std::io::Error>{
        let commit = CatFile::cat_file(commit_hash, Init::get_object_path(repo_path)?)?;
        let commit_lines: Vec<&str> = commit.split_whitespace().collect();
        Ok(CommitEntity{content_type: commit_lines[0].to_string(), tree_hash: commit_lines[1].to_string()})
    }
}