use crate::constants::constant::{STATE_CREATED, STATE_DELETED, STATE_MODIFIED};
use crate::types::set_type::{ChangesNotStagedForCommit, ChangesToBeCommited, UntrackedFiles};
use crate::utils::sets::set::{difference, idem_set_different_content};
use crate::vcs::files::vcs_file::VCSFile;
use std::collections::HashMap;

/// Transforma un HashMap del tipo String, VCSFile en un HashMap del tipo String, String.
pub fn transform_to_string_hashmap(staging_area: &HashMap<String, VCSFile>) -> HashMap<String, String> {
    let mut area: HashMap<String, String> = HashMap::new();
    for (key, value) in staging_area {
        area.insert(key.to_string(), value.content.to_string());
    }
    area
}

/// Recibe los archivos actuales, los que se encuentran en el area de staging y los que se encuentran en el repositorio local.
/// Devuelve aquellos archivos que fueron creados y que no se encuentran en el repositorio local.
pub fn get_untracked_files(files: &HashMap<String, String>, staging_area: &HashMap<String, String>, repository: &HashMap<String, String>) -> UntrackedFiles {
    let mut untracked_files = HashMap::new();
    for key in difference(&difference(files, repository), staging_area).keys() {
        untracked_files.insert(key.to_string(), STATE_CREATED.to_string());
    }
    untracked_files
}

/// Recibe los archivos que se encuentran en el area de staging.
/// Devuelve los mismos archivos pero en el formato String, String donde la clave es el estado del archivo.
pub fn get_changes_to_be_commited(staging_area: &HashMap<String, VCSFile>) -> ChangesToBeCommited {
    let mut changes_to_be_commited = HashMap::new();
    for (key, value) in staging_area {
        changes_to_be_commited.insert(key.to_string(), value.state.to_string());
    }
    changes_to_be_commited
}

/// Recibe los archivos actuales, los que se encuentran en el area de staging y los que se encuentran en el repositorio local.
/// Devuelve aquellos archivos que fueron modificados o eliminados respecto del area de staging o del repositorio local.
pub fn get_changes_not_staged_for_commit(files: &HashMap<String, String>, staging_area_vcs: &HashMap<String, VCSFile>, repository: &HashMap<String, String>) -> ChangesNotStagedForCommit {
    let staging_area = &transform_to_string_hashmap(staging_area_vcs);
    
    let mut changes_not_staged_for_commit = HashMap::new();

    for key in difference(&difference(repository, files), staging_area).keys() {
        changes_not_staged_for_commit.insert(key.to_string(), STATE_DELETED.to_string());
    }

    for key in difference(staging_area, files).keys(){
        if let Some(value) = staging_area_vcs.get(key){
            if value.state != STATE_DELETED{
                changes_not_staged_for_commit.insert(key.to_string(), STATE_DELETED.to_string());
            }
        }
    }

    for key in difference(&idem_set_different_content(files, repository), staging_area).keys() {
        changes_not_staged_for_commit.insert(key.to_string(), STATE_MODIFIED.to_string());
    }

    for key in idem_set_different_content(files, staging_area).keys() {
        changes_not_staged_for_commit.insert(key.to_string(), STATE_MODIFIED.to_string());
    }

    changes_not_staged_for_commit
}