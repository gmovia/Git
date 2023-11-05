use crate::{
    vcs::{files::{vcs_file::VCSFile, repository::Repository, index::Index}, version_control_system::VersionControlSystem},
    utils::files::files::read, constants::constants::{STATE_CREATED, STATE_DELETED, NULL},
};

use std::{collections::HashMap, path::Path};

use super::hash_object::WriteOption;
pub struct Add;

impl Add{
    /// Recibe el sistema de control de versiones y un path
    /// Inserta los archivos correspondientes al area de staging, junto con sus respectivos estados.
    /// Devuelve el area de staging.
    pub fn add(path: &Path) -> Result<HashMap<String, VCSFile>, std::io::Error> {
        let (untracked_files, changes_not_staged_for_commit, _) = VersionControlSystem::status()?;
        let mut staging_area = Index::read_index()?;

        if let Ok(files) = read(path) {
            for key in files.keys() {
                let state = match (untracked_files.get(key), changes_not_staged_for_commit.get(key)) {
                    (Some(state), _) => state.to_string(),
                    (_, Some(_)) if !Repository::read_repository()?.contains_key(key) => STATE_CREATED.to_string(),
                    (_, Some(state)) => state.to_string(),
                    _ => continue,
                };
                let hash = VersionControlSystem::hash_object(Path::new(&key.clone()), WriteOption::Write)?;
                let file = VCSFile::new(key.clone(), hash, state.clone());
                staging_area.insert(key.clone(), file);
            }
        }

        if let Some(element) = changes_not_staged_for_commit.get(&path.display().to_string()){
            if element == STATE_DELETED{
                let file = VCSFile::new(path.display().to_string(), NULL.to_string(), STATE_DELETED.to_string());
                staging_area.insert(path.display().to_string(), file);
            }
        }
        let _ = Index::write_index(&staging_area);
        Ok(staging_area.clone())
    }

}