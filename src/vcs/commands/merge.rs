use std::collections::HashMap;
use crate::constants::constants::MERGE;
use crate::vcs::entities::change::add_changes;
use crate::vcs::entities::conflict::{Conflict, resolve_conflicts, conflicts_search};
use crate::vcs::files::commits_table::CommitsTable;
use crate::vcs::commands::branch::Branch;
use crate::vcs::version_control_system::VersionControlSystem;
use super::checkout::Checkout;
use super::commit::Commit;
use super::diff::Diff;
#[derive(Debug, Clone)]
pub struct Merge;

impl Merge {
    pub fn merge(vcs: &VersionControlSystem, branch: &str, potential_conflicts: HashMap<String, Conflict>) -> Result<HashMap<String, Conflict>,std::io::Error> {
        let mut repository = vcs.repository.read_repository()?;

        let current_branch = Branch::get_current_branch(&vcs.path)?;

        let current_commits_table = CommitsTable::read(vcs.path.clone().to_path_buf(), &current_branch)?;
        let branch_commits_table = CommitsTable::read(vcs.path.clone().to_path_buf(), branch)?;

        let mut conflicts: HashMap<String, Conflict> = HashMap::new();

        if let (Some(last_commit_of_current_commits_table), Some(last_commit_of_branch_commits_table), Some(parent_commit)) = (current_commits_table.last(), branch_commits_table.last(), CommitsTable::get_parent_commit(&current_commits_table, &branch_commits_table)){
            let current_repository = CommitsTable::read_repository_of_commit(vcs.path.clone(), &current_branch, &last_commit_of_current_commits_table.hash)?;
            let branch_repository = CommitsTable::read_repository_of_commit(vcs.path.clone(), branch, &last_commit_of_branch_commits_table.hash)?;
            let parent_repository = CommitsTable::read_repository_of_commit(vcs.path.clone(), &current_branch, &parent_commit.hash)?;

            let mut changes_current_repository = Diff::diff(&parent_repository, &current_repository);
            let mut changes_branch_repository = Diff::diff(&parent_repository, &branch_repository);
            
            // si busco mergear => pasar una lista de nulos potencilaes conflictos
            // si busco resolver conflictos => paso una lista de potenciales conflictos

            resolve_conflicts(&potential_conflicts, &mut changes_current_repository, &mut changes_branch_repository);

            // recorro los potenciales conflictos y trato de arreglarlos
            // si no tengo potenciales conflictos entonces sigo de largo => me fijo si puedo hacer una f.a
            // si tengo los arreglo => me fijo si puedo hacer una f.a

            conflicts = conflicts_search(&changes_current_repository, &changes_branch_repository);

            if conflicts.len() == 0 { // FUSION AUTOMATICA
                add_changes(&mut repository, &changes_current_repository);
                add_changes(&mut repository, &changes_branch_repository);
                Commit::write_commit(vcs, &MERGE.to_string(), &repository)?;
                Checkout::update_cd(&vcs.path)?;
            }
        }
        Ok(conflicts)
    }
}