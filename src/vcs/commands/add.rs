use crate::{
    vcs::{files::{vcs_file::VCSFile, index::write}, version_control_system::VersionControlSystem},
    utils::files::files::read,
};

use std::{collections::HashMap, path::Path};
pub struct Add;

impl Add{ // OBSERVACION PONER ALGUN OPTION PARA ESCRIBIR EN EL TEST PORQUE SINO SE LLENA EL INDEX DE BASURA! REVISAR ESTO
    /// Recibe el sistema de control de versiones y un path
    /// Inserta los archivos correspondientes al area de staging, junto con sus respectivos estados.
    /// Devuelve el area de staging.
    pub fn add(vcs: &mut VersionControlSystem, path: &Path) -> Result<HashMap<String, VCSFile>, std::io::Error> {
        let (untracked_files, changes_not_staged_for_commit, _) = vcs.status()?;

        if let Ok(files) = read(path) {
            for (key, value) in &files {
                let state = match (untracked_files.get(key), changes_not_staged_for_commit.get(key)) {
                    (Some(state), _) => state.to_string(),
                    (_, Some(state)) => state.to_string(),
                    _ => continue,
                };
                let file = VCSFile::new(key.clone(), value.clone(), state.clone());
                vcs.staging_area.insert(key.clone(), file);
            }
        }

        if let Some(element) = changes_not_staged_for_commit.get(&path.display().to_string()){
            if element == "DELETED"{
                let file = VCSFile::new(path.display().to_string(), "".to_string(), "DELETED".to_string());
                vcs.staging_area.insert(path.display().to_string(), file);
            }
        }
    
        let _ = write(&vcs.staging_area);
        Ok(vcs.staging_area.clone())
    }

}
