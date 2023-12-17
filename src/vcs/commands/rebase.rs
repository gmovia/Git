use super::branch::Branch;
use crate::vcs::{
    commands::checkout::Checkout,
    files::{
        commits_table::CommitsTable, config::Config, current_commit::CurrentCommit,
        current_repository::CurrentRepository, repository::Repository,
    },
};
use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
    io::{self, Write},
    path::Path,
};

pub struct Rebase;

impl Rebase {
    /// Comando rebase
    /// Esta funcion recibe un nombre de una branch para cambiar la base y reordena los commits
    pub fn rebase(branch: &str) -> Result<(), std::io::Error> {
        let config = Config::read_config()?;
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
        CurrentCommit::write_for_branch(
            &current,
            &current_branch,
            CurrentCommit::read_for_branch(&current, branch)?,
        )?;

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
            CommitsTable::write(
                &current,
                &current_branch,
                &commit.message,
                config.clone(),
                &repository_last_commit,
            )?;
        }

        Checkout::update_cd(&current)?;

        Ok(())
    }

    pub fn rebase_pr(
        username: &str,
        head: &str,
        base: &str,
        head_path: &Path,
        base_path: &Path,
    ) -> Result<(), std::io::Error> {
        let base_commits_table = CommitsTable::read(base_path.to_path_buf(), base)?;
        let head_commits_table = CommitsTable::read(head_path.to_path_buf(), head)?;

        let mut base_file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(base_path.join(".rust_git").join("logs").join(base))?;

        let content = fs::read_to_string(head_path.join(".rust_git").join("logs").join(head))?;
        base_file.set_len(0)?;
        base_file.write_all(content.as_bytes())?;
        CurrentCommit::write_for_branch(
            base_path,
            base,
            CurrentCommit::read_for_branch(head_path, head)?,
        )?;

        let mut commits_rebase = Vec::new();
        for commit in base_commits_table {
            if !CommitsTable::contains(&head_commits_table, &commit) {
                commits_rebase.push(commit);
            }
        }

        for commit in commits_rebase {
            let mut repository_last_commit: HashMap<String, String> =
                Repository::read_repository_of_branch(head_path.to_path_buf(), head)?;
            let repository_commit: HashMap<String, String> =
                Repository::get_repository(base_path.to_path_buf(), &commit.hash)?;
            repository_last_commit.extend(repository_commit);
            CommitsTable::write(
                base_path,
                base,
                &commit.message,
                (username.to_string(), username.to_string()),
                &repository_last_commit,
            )?;
        }

        Checkout::update_cd(base_path)?;
        Ok(())
    }
}
