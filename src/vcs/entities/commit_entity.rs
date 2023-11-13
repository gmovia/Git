use std::{path::{PathBuf, Path}, fs::{OpenOptions, self}, io::Write};
use crate::{vcs::commands::{hash_object::{HashObject, WriteOption}, init::Init, cat_file::CatFile}, utils::random::random::Random, constants::constants::COMMIT_CODE};

pub struct CommitEntity{
    pub content_type: String,
    pub tree_hash: String,
    pub message: String,
}

impl CommitEntity{

    pub fn write(repo_path: &PathBuf, commit: &CommitEntity) -> Result<String, std::io::Error>{
        let commit_path = Path::new(&repo_path).join(Random::random());
        let mut commit_file = OpenOptions::new().write(true).create(true).append(true).open(&commit_path)?; 

        let author_user_info = "author ldiazcto <ldiazc@fi.uba.ar> 1699704762 -0300";
        let commiter_user_info = "committer ldiazcto <ldiazc@fi.uba.ar> 1699704762 -0300";

        let entry = format!("tree {}\n{}\n{}\n\n{}\n", commit.tree_hash, author_user_info, commiter_user_info, commit.message);
        commit_file.write_all(entry.as_bytes())?;
        
        let commit_hash = HashObject::hash_object(&commit_path, Init::get_object_path(&repo_path)?, WriteOption::Write, COMMIT_CODE)?;
        
        let _ = fs::remove_file(commit_path);
        Ok(commit_hash)
    }
    
    pub fn read(repo_path: &PathBuf, commit_hash: &str) -> Result<CommitEntity, std::io::Error>{
        let commit = CatFile::cat_file(commit_hash, Init::get_object_path(repo_path)?)?;
        let commit_lines: Vec<&str> = commit.split_whitespace().collect();
        Ok(CommitEntity{content_type: commit_lines[0].to_string(), tree_hash: commit_lines[1].to_string(), message: commit_lines[5].to_string()})
    }
}