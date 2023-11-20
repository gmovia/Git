use std::{path::{PathBuf, Path}, fs::{OpenOptions, self}, io::Write};
use crate::{vcs::commands::{hash_object::{HashObject, WriteOption}, init::Init, cat_file::CatFile}, utils::randoms::random::Random, constants::constant::{COMMIT_CODE, COMMIT_INIT_HASH}};

pub struct CommitEntity{
    pub content_type: String,
    pub tree_hash: String,
    pub parent_hash: String,
    pub author: String,
    pub committer: String,
    pub message: String,
}

impl CommitEntity{

    pub fn write(repo_path: &PathBuf, commit: &CommitEntity) -> Result<String, std::io::Error>{
        let commit_path = Path::new(&repo_path).join(Random::random());
        let mut commit_file = OpenOptions::new().write(true).create(true).append(true).open(&commit_path)?; 

        if commit.parent_hash == COMMIT_INIT_HASH{
            let entry = format!("tree {}\n{}\n{}\n\n{}\n", commit.tree_hash, commit.author, commit.committer, commit.message);
            commit_file.write_all(entry.as_bytes())?;
        }
        else{
            let entry = format!("tree {}\nparent {}\n{}\n{}\n\n{}\n", commit.tree_hash, commit.parent_hash, commit.author, commit.committer, commit.message);
            commit_file.write_all(entry.as_bytes())?;
        }
        
        let commit_hash = HashObject::hash_object(&commit_path, Init::get_object_path(repo_path)?, WriteOption::Write, COMMIT_CODE)?;
        
        let _ = fs::remove_file(commit_path);
        Ok(commit_hash)
    }
    
    pub fn read(repo_path: &PathBuf, commit_hash: &str) -> Result<CommitEntity, std::io::Error>{
        let commit = CatFile::cat_file(commit_hash, Init::get_object_path(repo_path)?)?;
        let commit_lines: Vec<&str> = commit.split('\n').collect();
        let tree_info: Vec<&str> = commit_lines[0].split_whitespace().collect();

        let second_line: Vec<&str> = commit_lines[1].split_whitespace().collect();
        if second_line[0] == "parent"{
            return Ok(CommitEntity{content_type: tree_info[0].to_string(), tree_hash: tree_info[1].to_string(), parent_hash: "".to_string(), author: commit_lines[2].to_string(), committer: commit_lines[3].to_string(),  message: commit_lines[5].to_string() })
        }
        Ok(CommitEntity{content_type: tree_info[0].to_string(), tree_hash: tree_info[1].to_string(), parent_hash: second_line[1].to_string(), author: commit_lines[2].to_string(), committer: commit_lines[3].to_string(),  message: commit_lines[5].to_string() })
    }
}