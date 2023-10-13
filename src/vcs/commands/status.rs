use std::collections::HashMap;
use crate::types::types::{ChangesNotStagedForCommit, ChangesToBeCommited, UntrackedFiles};
use crate::vcs::files::vcs_file::VCSFile;
use crate::vcs::sets::sets::{get_changes_not_staged_for_commit, get_untracked_files, get_changes_to_be_commited, transform_to_string_hashmap};

pub struct Status;

impl Status{
    /// Recibe los archivos actuales, los que se encuentran en el area de staging y los que se encuentran en el repositorio.
    /// Calcula los diversos conjuntos con los que trabaja el comando git status.
    /// Untracked files => Archivos que no fueron rastreados en el repositorio local.
    /// Changes not staged for commit => Archivos modificados o eliminados que fueron rastreados en el repositorio local.
    /// Changes to be commited => Archivos que se encuentran en el area de staging.
    pub fn status(files: &HashMap<String, String>, staging_area: &HashMap<String, VCSFile>, repository: &HashMap<String, String>) -> (UntrackedFiles, ChangesNotStagedForCommit, ChangesToBeCommited){
        let untracked_files = get_untracked_files(files, &transform_to_string_hashmap(staging_area), repository);
        let changes_to_be_commited = get_changes_to_be_commited(staging_area);
        let changes_not_staged_for_commit = get_changes_not_staged_for_commit(files, staging_area, repository);
        (untracked_files, changes_not_staged_for_commit, changes_to_be_commited)
    }
}