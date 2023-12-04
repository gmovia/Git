use std::path::Path;

use crate::{
    constants::constant::{STATE_DELETED, STATE_MODIFIED},
    vcs::{files::repository::Repository, version_control_system::VersionControlSystem},
};

pub struct LsFiles;

pub enum LsFilesOptions {
    EverythingInVCS,
    OnlyUntracked,
    OnlyModified,
    OnlyStaging,
    OnlyDeleted,
}

impl LsFiles {

    /// Comando ls_files.
    /// Recibe el current path y una option para realizar distintas operaciones
    pub fn ls_files(option: LsFilesOptions, path: &Path) -> Result<Vec<String>, std::io::Error> {
        let mut files: Vec<String> = Vec::new();
        match option {
            LsFilesOptions::EverythingInVCS => Ok(Self::get_everything(path, &mut files)?),
            LsFilesOptions::OnlyModified => Ok(Self::get_modified(path, &mut files)?),
            LsFilesOptions::OnlyStaging => Ok(Self::get_staging(path, &mut files)?),
            LsFilesOptions::OnlyDeleted => Ok(Self::get_deleted(path, &mut files)?),
            LsFilesOptions::OnlyUntracked => Ok(Self::get_untracked(path, &mut files)?),
        }
    }

    /// Devuelve todos los files del repositorio
    pub fn get_everything(
        _path: &Path,
        files: &mut Vec<String>,
    ) -> Result<Vec<String>, std::io::Error> {
        let local_repository = Repository::read_repository()?;
        let (_, changes_not_staged_for_commit, changes_to_be_commited) =
            VersionControlSystem::status()?;
        for (key, _) in changes_to_be_commited {
            if !files.contains(&key) {
                files.push(key);
            }
        }
        for (key, _) in changes_not_staged_for_commit {
            if !files.contains(&key) {
                files.push(key);
            }
        }
        for (key, _) in local_repository {
            if !files.contains(&key) {
                files.push(key);
            }
        }
        Ok(files.to_vec())
    }

    /// Devuelve los files que fueron modificados luego de committearlos, pero no se volvieron a committear
    pub fn get_modified(
        _path: &Path,
        files: &mut Vec<String>,
    ) -> Result<Vec<String>, std::io::Error> {
        let (_, changes_not_staged_for_commit, _) = VersionControlSystem::status()?;
        for (key, value) in changes_not_staged_for_commit {
            if value == STATE_MODIFIED || value == STATE_DELETED {
                files.push(key);
            }
        }
        Ok(files.to_vec())
    }


    /// Devuelve los files que fueron agregados al staging_area con el comando add, pero todavia no se committearon
    pub fn get_staging(
        _path: &Path,
        files: &mut Vec<String>,
    ) -> Result<Vec<String>, std::io::Error> {
        let (_, _, changes_to_be_commited) = VersionControlSystem::status()?;
        for (key, _) in changes_to_be_commited {
            files.push(key);
        }
        Ok(files.to_vec())
    }


    /// Devuelve los files que fueron eliminados luego de committearlos, pero no se volvieron a committear
    pub fn get_deleted(
        _path: &Path,
        files: &mut Vec<String>,
    ) -> Result<Vec<String>, std::io::Error> {
        let (_, changes_not_staged_for_commit, _) = VersionControlSystem::status()?;
        for (key, value) in changes_not_staged_for_commit {
            if value == STATE_DELETED {
                files.push(key);
            }
        }
        Ok(files.to_vec())
    }


    /// Devuelve los files creados que todavia no fueron committeados
    pub fn get_untracked(
        _path: &Path,
        files: &mut Vec<String>,
    ) -> Result<Vec<String>, std::io::Error> {
        let (untracked_files, _, _) = VersionControlSystem::status()?;
        for (key, _) in untracked_files {
            files.push(key);
        }
        Ok(files.to_vec())
    }
}
