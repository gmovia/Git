use std::collections::HashMap;
use crate::vcs::files::commits_table::{CommitsTable, CommitEntry};
use crate::vcs::commands::branch::Branch;
use crate::vcs::version_control_system::VersionControlSystem;

use super::checkout::Checkout;
use super::commit::Commit;
#[derive(Debug, Clone)]
pub struct Merge;

#[derive(Debug, Clone)]
pub struct Change{
    pub file: String,
    pub hash: String,
    pub state: String,
}

#[derive(Debug, Clone)]
pub struct Conflict{
    pub file: String,
    pub change_current: Change,
    pub change_branch: Change
}

impl Merge {
    pub fn merge(vcs: &VersionControlSystem, branch: &str) -> Result<HashMap<String, Conflict>,std::io::Error> {
        let mut repository = vcs.repository.read_repository()?;

        let current_branch = Branch::get_current_branch(&vcs.path)?;

        // Obtenemos las tablas de commits
        let current_commits_table = CommitsTable::read(vcs.path.clone().to_path_buf(), &current_branch)?;
        let branch_commits_table = CommitsTable::read(vcs.path.clone().to_path_buf(), branch)?;

        let mut conflicts: HashMap<String, Conflict> = HashMap::new();

        if let (Some(last_commit_of_current_commits_table), Some(last_commit_of_branch_commits_table), Some(parent_commit)) = (current_commits_table.last(), branch_commits_table.last(), Self::get_parent_commit(&current_commits_table, &branch_commits_table)){
            let current_repository = CommitsTable::read_repository_of_commit(vcs.path.clone(), &current_branch, &last_commit_of_current_commits_table.hash)?;
            let branch_repository = CommitsTable::read_repository_of_commit(vcs.path.clone(), branch, &last_commit_of_branch_commits_table.hash)?;
            let parent_repository = CommitsTable::read_repository_of_commit(vcs.path.clone(), &current_branch, &parent_commit.hash)?;

            // Juntamos las diferencias y vemos si no hay conflicto (por ahora, no analizar los conflictos)
            let changes_current_repository = Self::diff(&parent_repository, &current_repository);
            let changes_branch_repository = Self::diff(&parent_repository, &branch_repository);

            // HAY CONFLICTO CUANDO LA RAMA ACTUAL Y LA RAMA A MERGEAR REALIZAN CAMBIOS SOBRE UN MISMO ARCHIVO.
            // TMB PUEDE HABER CONFLICTO POR CAMBIOS DE ESTADO EN UN MISMO ARCHIVO ENTRE ESAS DOS RAMAS.
            
            for change_current in &changes_current_repository {
                for change_branch in &changes_branch_repository {
                    if change_branch.file == change_current.file && (change_branch.hash != change_current.hash || change_branch.state != change_current.state) {
                        let conflict = Conflict{file: change_branch.file.clone(), change_current: change_current.clone(), change_branch: change_branch.clone()};
                        conflicts.insert(change_branch.file.clone(), conflict);
                    }
                }
            }

            if conflicts.len() == 0 { // FUSION AUTOMATICA
                
                for change in changes_current_repository{
                    match change.state.as_str() {
                        "CREATED" | "MODIFIED" => {repository.insert(change.file, change.hash);},
                        "DELETED" => {repository.remove(&change.file);},
                        _ => {},
                    } 
                }

                for change in changes_branch_repository{
                    match change.state.as_str() {
                        "CREATED" | "MODIFIED" => {repository.insert(change.file, change.hash);},
                        "DELETED" => {repository.remove(&change.file);},
                        _ => {},
                    } 
                }
                // RESOLVER CONFLICTOS => VEMO DEPUE QUE HACEMO
                Commit::write_commit(vcs, &"merge".to_string(), &repository)?;
            }
        }
        Checkout::update_cd(&vcs.path)?;
        Ok(conflicts)
    }
    
    pub fn get_parent_commit(current_commits: &Vec<CommitEntry>, branch_commits: &Vec<CommitEntry>) ->  Option<CommitEntry>{ // PRESTAR ATENCION
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

    pub fn diff(parent: &HashMap<String, String>, current: &HashMap<String, String>) -> Vec<Change>{
        let mut diff: Vec<Change> = Vec::new();
        for (path, hash) in current{
            if !parent.contains_key(path){
                let change = Change {file: path.to_string(), state: "CREATED".to_string(), hash: hash.to_string()};
                diff.push(change);
            }
            else{
                if let Some(hash_parent) = parent.get(path){
                    if hash != hash_parent{
                        let change = Change {file: path.to_string(), state: "MODIFIED".to_string(), hash: hash.to_string()};
                        diff.push(change);
                    }
                }
            }
        }

        for (path, hash) in parent{
            if !current.contains_key(path){
                let change = Change {file: path.to_string(), state: "DELETED".to_string(), hash: hash.to_string()};
                diff.push(change);
            }
        }

        diff        
    }
}