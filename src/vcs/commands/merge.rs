use super::checkout::Checkout;
use super::diff::Diff;
use crate::constants::constant::MERGE;
use crate::vcs::commands::branch::Branch;
use crate::vcs::entities::change::add_changes;
use crate::vcs::entities::conflict::{conflicts_search, resolve_conflicts, Conflict};
use crate::vcs::files::commits_table::CommitsTable;
use crate::vcs::files::current_commit::CurrentCommit;
use crate::vcs::files::current_repository::CurrentRepository;
use crate::vcs::files::repository::Repository;
use std::collections::HashMap;
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
                CommitsTable::write(&MERGE.to_string(), &repository)?;
                Checkout::update_cd(&current)?;
            }
        }
        Ok(conflicts)
    }
}
