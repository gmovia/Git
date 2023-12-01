use std::{fs::{OpenOptions, self}, io::{Read, Write, self, BufRead}, collections::HashMap};
use crate::{vcs::{files::{current_repository::CurrentRepository, commits_table::CommitsTable, repository::Repository, current_commit::CurrentCommit}, entities::commit_table_entry::CommitTableEntry, commands::checkout::Checkout}, constants::constant::{STATE_CREATED, STATE_MODIFIED}, utils::randoms::random::Random};
use super::{branch::Branch, diff::Diff};

pub struct Rebase;

impl Rebase{
    pub fn rebase(branch: &str) -> Result<(), std::io::Error>{
        let current = CurrentRepository::read()?;
        let current_branch = Branch::get_current_branch(&current)?;
        let mut names = Vec::new();
        let refs_path = current.join(".rust_git").join("refs").join("heads");

        if let Ok(entries) = fs::read_dir(refs_path) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                names.push(name);
            }
        }

        if !names.contains(&branch.to_string()){
            return Err(io::Error::new(io::ErrorKind::NotFound, "Can't find the branch"));
        }

        let old_current_commits_table = CommitsTable::read(current.clone(), &current_branch)?;
        let branch_commits_table = CommitsTable::read(current.clone(), branch)?;
        
        let mut current_file = OpenOptions::new().write(true).create(true).append(true).open(current.join(".rust_git").join("logs").join(&current_branch))?;

        let content = fs::read_to_string(current.join(".rust_git").join("logs").join(branch))?;
        current_file.set_len(0)?;
        current_file.write_all(content.as_bytes())?;
        CurrentCommit::write(CurrentCommit::read_for_branch(&current, branch)?)?;


        let mut commits_rebase = Vec::new();
        for commit in old_current_commits_table{
            if !CommitsTable::contains(&branch_commits_table, &commit){
                commits_rebase.push(commit);                  
            }
        }

        for commit in commits_rebase{
            let mut repository_last_commit: HashMap<String, String> =  Repository::read_repository_of_branch(current.clone(), branch)?;
            let repository_commit: HashMap<String, String> = Repository::get_repository(current.clone(),&commit.hash)?;
            repository_last_commit.extend(repository_commit);
            CommitsTable::write(&commit.message, &repository_last_commit)?; 
        }

        Checkout::update_cd(&current)?;

        Ok(())
    }
}

