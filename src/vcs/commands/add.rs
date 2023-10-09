use crate::{
    vcs::{files::vcs_file::VCSFile, version_control_system::VersionControlSystem},
    utils::files::files::read,
};

use std::{collections::HashMap, path::Path};
pub struct Add;

impl Add{
    /// Recibe el sistema de control de versiones y un path
    /// Inserta los archivos correspondientes al area de staging, junto con sus respectivos estados.
    /// Devuelve el area de staging.
    pub fn add(vcs: &mut VersionControlSystem, path: &Path) -> Result<HashMap<String, VCSFile>, std::io::Error> {
        let (untracked_files, changes_not_staged_for_commit, _) = vcs.status()?;
    
        if let Ok(files) = read(path) {
            for (key, value) in &files {
                let state = match (untracked_files.get(key), changes_not_staged_for_commit.get(key)) {
                    (Some(state), _) => state.to_string(),
                    (_, Some(_)) if !vcs.local_repository.contains_key(key) => "CREATED".to_string(),
                    (_, Some(state)) => state.to_string(),
                    _ => continue,
                };
                let file = VCSFile::new(key.clone(), value.clone(), state);
                vcs.staging_area.insert(key.clone(), file);
            }
        }
    
        if vcs.local_repository.contains_key(&path.display().to_string()) && !read(path)?.contains_key(&path.display().to_string()){
            let file = VCSFile::new(path.display().to_string(), "".to_string(), "DELETED".to_string());
            vcs.staging_area.insert(path.display().to_string(), file);
        }
    
        Ok(vcs.staging_area.clone())
    }
}