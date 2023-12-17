use super::checkout::Checkout;
use super::diff::Diff;
use crate::constants::constant::MERGE;
use crate::vcs::commands::branch::Branch;
use crate::vcs::entities::change::add_changes;
use crate::vcs::entities::conflict::{conflicts_search, resolve_conflicts, Conflict};
use crate::vcs::files::commits_table::CommitsTable;
use crate::vcs::files::config::Config;
use crate::vcs::files::current_commit::CurrentCommit;
use crate::vcs::files::current_repository::CurrentRepository;
use crate::vcs::files::repository::Repository;
use std::collections::HashMap;
use std::path::Path;
#[derive(Debug, Clone)]
pub struct Merge;

impl Merge {
    /// Comando merge.
    /// Utiliza la tabla de commits actual y la de una branch especifica y compara la ultima entrada (commit actual)
    pub fn merge(
        branch: &str,
        potential_conflicts: HashMap<String, Conflict>,
    ) -> Result<HashMap<String, Conflict>, std::io::Error> {
        let mut repository = Repository::read_repository()?;
        let config = Config::read_config()?;

        let current = CurrentRepository::read()?;
        let current_branch = Branch::get_current_branch(&current)?;

        let current_commits_table = CommitsTable::read(current.clone(), &current_branch)?;
        let branch_commits_table = CommitsTable::read(current.clone(), branch)?;
        let mut conflicts: HashMap<String, Conflict> = HashMap::new();

        let current_commit_of_current_commits_table =
            CurrentCommit::read_for_branch(&current, &current_branch)?;
        let current_commit_of_branch_commits_table =
            CurrentCommit::read_for_branch(&current, branch)?;

        if let Some(parent_commit) =
            CommitsTable::get_parent_commit(&current_commits_table, &branch_commits_table)
        {
            let current_repository = Repository::read_repository_of_commit(
                current.clone(),
                &current_branch,
                &current_commit_of_current_commits_table,
            )?;
            let branch_repository = Repository::read_repository_of_commit(
                current.clone(),
                branch,
                &current_commit_of_branch_commits_table,
            )?;
            let parent_repository = Repository::read_repository_of_commit(
                current.clone(),
                &current_branch,
                &parent_commit.hash,
            )?;

            let mut changes_current_repository =
                Diff::diff(&parent_repository, &current_repository);
            let mut changes_branch_repository = Diff::diff(&parent_repository, &branch_repository);

            resolve_conflicts(
                &potential_conflicts,
                &mut changes_current_repository,
                &mut changes_branch_repository,
            );

            conflicts = conflicts_search(&changes_current_repository, &changes_branch_repository);

            if conflicts.is_empty() {
                add_changes(&mut repository, &changes_current_repository);
                add_changes(&mut repository, &changes_branch_repository);
                CommitsTable::write(
                    &current,
                    &current_branch,
                    &MERGE.to_string(),
                    config,
                    &repository,
                )?;
                Checkout::update_cd(&current)?;
            }
        }
        Ok(conflicts)
    }

    pub fn merge_pr(
        username: &str,
        head: &str,
        base: &str,
        head_path: &Path,
        base_path: &Path,
        potential_conflicts: HashMap<String, Conflict>,
    ) -> Result<HashMap<String, Conflict>, std::io::Error> {
        let base_commits_table = CommitsTable::read(base_path.to_path_buf(), base)?;
        let head_commits_table = CommitsTable::read(head_path.to_path_buf(), head)?;

        let mut conflicts: HashMap<String, Conflict> = HashMap::new();

        let current_commit_of_base_commits_table = CurrentCommit::read_for_branch(base_path, base)?;
        let current_commit_of_head_commits_table = CurrentCommit::read_for_branch(head_path, head)?;

        if let Some(parent_commit) =
            CommitsTable::get_parent_commit(&base_commits_table, &head_commits_table)
        {
            let base_repository = Repository::read_repository_of_commit(
                base_path.to_path_buf(),
                base,
                &current_commit_of_base_commits_table,
            )?;

            let head_repository = Repository::read_repository_of_commit(
                head_path.to_path_buf(),
                head,
                &current_commit_of_head_commits_table,
            )?;

            let mut parent_repository = Repository::read_repository_of_commit(
                base_path.to_path_buf(),
                base,
                &parent_commit.hash,
            )?;

            let mut changes_base_repository = Diff::diff(&parent_repository, &base_repository);
            let mut changes_head_repository = Diff::diff(&parent_repository, &head_repository);

            resolve_conflicts(
                &potential_conflicts,
                &mut changes_base_repository,
                &mut changes_head_repository,
            );

            conflicts = conflicts_search(&changes_base_repository, &changes_head_repository);

            if conflicts.is_empty() {
                add_changes(&mut parent_repository, &changes_base_repository);
                add_changes(&mut parent_repository, &changes_head_repository);
                println!("LA BASE ES {}", base);
                CommitsTable::write(
                    base_path,
                    base,
                    &MERGE.to_string(),
                    (username.to_string(), username.to_string()),
                    &parent_repository,
                )?;
                Checkout::update_cd(base_path)?;
            }
        }

        Ok(conflicts)
    }

    pub fn are_conflicts(
        head: &str,
        base: &str,
        head_path: &Path,
        base_path: &Path,
    ) -> Result<bool, std::io::Error> {
        let base_commits_table = CommitsTable::read(base_path.to_path_buf(), base)?;
        let head_commits_table = CommitsTable::read(head_path.to_path_buf(), head)?;

        let current_commit_of_base_commits_table = CurrentCommit::read_for_branch(base_path, base)?;
        let current_commit_of_head_commits_table = CurrentCommit::read_for_branch(head_path, head)?;

        if let Some(parent_commit) =
            CommitsTable::get_parent_commit(&base_commits_table, &head_commits_table)
        {
            let base_repository = Repository::read_repository_of_commit(
                base_path.to_path_buf(),
                base,
                &current_commit_of_base_commits_table,
            )?;

            let head_repository = Repository::read_repository_of_commit(
                head_path.to_path_buf(),
                head,
                &current_commit_of_head_commits_table,
            )?;

            let parent_repository = Repository::read_repository_of_commit(
                base_path.to_path_buf(),
                base,
                &parent_commit.hash,
            )?;

            let changes_base_repository = Diff::diff(&parent_repository, &base_repository);
            let changes_head_repository = Diff::diff(&parent_repository, &head_repository);

            let conflicts: HashMap<String, Conflict> =
                conflicts_search(&changes_base_repository, &changes_head_repository);

            return Ok(!conflicts.is_empty());
        }

        Ok(true)
    }
}
