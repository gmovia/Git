use super::{
    commits_table::CommitsTable, current_commit::CurrentCommit,
    current_repository::CurrentRepository,
};
use crate::vcs::entities::{commit_entity::CommitEntity, tree_entity::TreeEntity};
use crate::vcs::{commands::init::Init, entities::entity::convert_to_repository};
use std::path::Path;
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Clone)]
pub struct Repository;

impl Repository {

    /// Devuelve el hashmap del local repository llamando a read()
    pub fn read_repository() -> Result<HashMap<String, String>, std::io::Error> {
        Self::read(&CurrentRepository::read()?)
    }


    /// Devuelve el hashmap del local repository
    pub fn read(repo_path: &Path) -> Result<HashMap<String, String>, std::io::Error> {
        let current_branch = &Init::get_current_branch(repo_path)?;

        let current_commit_hash = CurrentCommit::read()?;

        let mut local_repository: HashMap<String, String> = HashMap::new();
        local_repository.extend(Repository::read_repository_of_commit(
            repo_path.to_path_buf(),
            current_branch,
            &current_commit_hash,
        )?);
        Ok(local_repository)
    }

    /// Devuelve el repositorio asociado a un commit y a una rama
    pub fn read_repository_of_commit(
        repo_path: PathBuf,
        branch: &str,
        commit_hash: &str,
    ) -> Result<HashMap<String, String>, std::io::Error> {
        let commits_table = CommitsTable::read(repo_path.clone(), branch)?;

        for commit in commits_table {
            if commit.hash == commit_hash {
                return Self::get_repository(repo_path, commit_hash);
            }
        }
        Ok(HashMap::new())
    }

    /// Obtiene las entidades asociadas a un commit y las transforma en un repositorio
    pub fn get_repository(
        repo_path: PathBuf,
        commit_hash: &str,
    ) -> Result<HashMap<String, String>, std::io::Error> {
        let commit_entity = CommitEntity::read(&repo_path, commit_hash)?;
        let entities = TreeEntity::read(&repo_path, commit_entity.tree_hash)?;
        Ok(convert_to_repository(&entities, CurrentRepository::read()?))
    }

    /// Te devuelve el repositorio actual de una branch especifica
    pub fn read_repository_of_branch(
        repo_path: PathBuf,
        branch: &str,
    ) -> Result<HashMap<String, String>, std::io::Error> {
        let commits_table = CommitsTable::read(repo_path.clone(), branch)?;
        let current_commit_hash = CurrentCommit::read_for_branch(&repo_path, branch)?;

        for commit in commits_table {
            if commit.hash == current_commit_hash {
                let commit_entity = CommitEntity::read(&repo_path, &current_commit_hash)?;

                let entities = TreeEntity::read(&repo_path, commit_entity.tree_hash)?;

                return Ok(convert_to_repository(&entities, CurrentRepository::read()?));
            }
        }
        Ok(HashMap::new())
    }
}
