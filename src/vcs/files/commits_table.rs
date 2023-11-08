use std::{path::{PathBuf, Path}, fs::{OpenOptions, self}, collections::HashMap, io::{Write, self, BufRead}};

use chrono::{Local, DateTime};

use crate::{vcs::{entities::commit_entry::CommitEntry, files::current_repository::CurrentRepository, commands::{init::Init, hash_object::{WriteOption, HashObject}}}, utils::random::random::Random};

use super::repository::Repository;

#[derive(Debug, Clone)]
pub struct CommitsTable;

impl CommitsTable{

    pub fn read(repo_path: PathBuf, branch: &str) -> Result<Vec<CommitEntry>, std::io::Error>{
        let mut commits: Vec<CommitEntry> = Vec::new();
        let path = repo_path.join(".rust_git").join("logs").join(Path::new(branch));
        let commits_file = OpenOptions::new().read(true).open(path)?;

        let reader = io::BufReader::new(commits_file);
        for line in reader.lines().filter_map(Result::ok) {
            let parts: Vec<&str> = line.split("-").collect();
            let commit = CommitEntry{id: parts[0].to_string(), hash: parts[1].to_string(), message: parts[2].to_string(), date: parts[3].to_string()};
            commits.push(commit);
        }
        Ok(commits)
    } 

    pub fn write(message: &String, repository: &HashMap<String, String>) -> Result<(),std::io::Error>{
        let id = Random::random();
        let current_time: DateTime<Local> = Local::now();
        let _ = current_time.to_rfc2822();
        
        let current = CurrentRepository::read()?;

        let mut commits_file = OpenOptions::new().write(true).append(true).open(Init::get_commits_path(&current)?)?; //abro la tabla de commits para escribir - si no existe, la creo
        
        let commit_hash = Self::create_tree(&current, &repository)?;

        let commit = format!("{}-{}-{}-{}\n", id, commit_hash, message, current_time); 
        commits_file.write_all(commit.as_bytes())?;
        Ok(())
    }

    pub fn get_parent_commit(current_commits: &Vec<CommitEntry>, branch_commits: &Vec<CommitEntry>) ->  Option<CommitEntry>{
        let size = if current_commits.len() >= branch_commits.len() { branch_commits.len() } else { current_commits.len() };
        for index in 0..size{
            if current_commits[index].id == branch_commits[index].id{
                if index == size - 1{
                    return Some(current_commits[index].clone())
                }
                continue;
            }
            return Some(current_commits[index-1].clone());
        }
        None
    }

    pub fn create_tree(path: &PathBuf, repository: &HashMap<String, String>) -> Result<String, std::io::Error>{
        let tree_path = Path::new(&path).join("tree");
        let mut tree_file = OpenOptions::new().write(true).create(true).append(true).open(&tree_path)?; 

        let repository_hash = Repository::write_repository(&repository)?;
        
        let entry = format!("tree {}\n", repository_hash);
        tree_file.write_all(entry.as_bytes())?;
        
        let hash = HashObject::hash_object(&tree_path, Init::get_object_path(&path)?, WriteOption::Write)?;
        
        let _ = fs::remove_file(tree_path);
        Ok(hash)
    }
}