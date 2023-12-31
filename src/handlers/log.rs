use crate::constants::constant::ERR_COMMIT_IS_NOT_EXIST;
use crate::vcs::commands::init::Init;
use crate::vcs::files::current_repository::CurrentRepository;
use crate::vcs::version_control_system::VersionControlSystem;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Controlador del comando log. Recibe la instruccion y ejecuta el comando.
pub fn handler_log() -> String {
    if let Ok(current) = CurrentRepository::read() {
        if let Ok(path) = Init::get_current_log(&current) {
            if let Ok(commits_file) = File::open(path) {
                let reader = BufReader::new(commits_file);
                let has_commits = reader.lines().count() > 0;
                if has_commits {
                    if let Ok(result) = VersionControlSystem::log() {
                        return result;
                    }
                }
            }
        }
    }
    ERR_COMMIT_IS_NOT_EXIST.to_string()
}
