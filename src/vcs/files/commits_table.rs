use std::{path::{PathBuf, Path}, io::{self, BufRead, Write}, fs::OpenOptions, collections::HashMap};

use crate::vcs::commands::{cat_file::CatFile, init::Init};

#[derive(Debug, Clone)]
pub struct CommitsTable;

pub struct CommitEntry{
    pub id: String,
    pub hash: String,
    pub message: String,
    pub date: String,
}

impl CommitsTable{

    pub fn read_repository_of_commit(repo_path: PathBuf, branch: &str, commit_hash: &str) -> Result<HashMap<String, String>,std::io::Error>{
        let mut repository: HashMap<String, String> = HashMap::new();
        let commits_table = CommitsTable::read(repo_path.clone(), branch)?;

        for commit in commits_table {
            if commit.hash == commit_hash {
                let content = CatFile::cat_file(commit_hash, Init::get_object_path(&repo_path)?)?;
                let content_lines: Vec<&str> = content.split("\n").collect();
                for line in content_lines{
                    if line != ""{
                        let line_parts: Vec<&str> = line.split("-").collect(); // line_parts[0] = path ; line_parts[1] = content
                        repository.insert(line_parts[0].to_string(), line_parts[1].to_string());
                    }
                }
            }
        }
        Ok(repository)
    }

    pub fn read(repo_path: PathBuf, branch: &str) -> Result<Vec<CommitEntry>, std::io::Error>{
        let mut commits: Vec<CommitEntry> = Vec::new();
        let path = repo_path.join(".rust_git").join("logs").join(Path::new(branch));
        let commits_file = OpenOptions::new().read(true).open(path)?;

        let reader = io::BufReader::new(commits_file);
        for line in reader.lines().filter_map(Result::ok) { //leo linea a linea la tabla
            let parts: Vec<&str> = line.split("-").collect();
            let commit = CommitEntry{id: parts[0].to_string(), hash: parts[1].to_string(), message: parts[2].to_string(), date: parts[3].to_string()};
            commits.push(commit); //me quedo con el hash, lo agrego al vec
        }
        Ok(commits)
    } 

    pub fn write(repo_path: PathBuf, commits: &Vec<CommitEntry>, branch: &str) -> Result<(), std::io::Error>{
        let path = repo_path.join(".rust_git").join("logs").join(Path::new(branch));
        let mut commits_file = OpenOptions::new().read(true).open(path)?;

        for commit in commits{
            let entry = format!("{}-{}-{}-{}\n", commit.id, commit.hash, commit.message, commit.date);
            commits_file.write_all(entry.as_bytes())?;
        }
        Ok(())
    } 
}