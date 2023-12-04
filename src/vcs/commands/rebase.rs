use super::branch::Branch;
use crate::vcs::{
    commands::checkout::Checkout,
    files::{
        commits_table::CommitsTable, current_commit::CurrentCommit,
        current_repository::CurrentRepository, repository::Repository,
    },
};
use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
    io::{self, Write},
};

pub struct Rebase;

impl Rebase {

    /// Comando rebase
    /// Esta funcion recibe un nombre de una branch para cambiar la base y reordena los commits
    pub fn rebase(branch: &str) -> Result<(), std::io::Error> {
        let current = CurrentRepository::read()?;
        let current_branch = Branch::get_current_branch(&current)?;

        let branches = Branch::get_branches(&current)?;
        if !branches.contains(&branch.to_string()) {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Can't find the branch",
            ));
        }

        let old_current_commits_table = CommitsTable::read(current.clone(), &current_branch)?;
        let branch_commits_table = CommitsTable::read(current.clone(), branch)?;

        let mut current_file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(current.join(".rust_git").join("logs").join(&current_branch))?;

        let content = fs::read_to_string(current.join(".rust_git").join("logs").join(branch))?;
        current_file.set_len(0)?;
        current_file.write_all(content.as_bytes())?;
        CurrentCommit::write(CurrentCommit::read_for_branch(&current, branch)?)?;

        let mut commits_rebase = Vec::new();
        for commit in old_current_commits_table {
            if !CommitsTable::contains(&branch_commits_table, &commit) {
                commits_rebase.push(commit);
            }
        }

        for commit in commits_rebase {
            let mut repository_last_commit: HashMap<String, String> =
                Repository::read_repository_of_branch(current.clone(), branch)?;
            let repository_commit: HashMap<String, String> =
                Repository::get_repository(current.clone(), &commit.hash)?;
            repository_last_commit.extend(repository_commit);
            CommitsTable::write(&commit.message, &repository_last_commit)?;
        }

        Checkout::update_cd(&current)?;

        Ok(())
    }
}
