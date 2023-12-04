use std::path::Path;

use crate::{
    constants::constant::{FULL_ADD, RESPONSE_OK_ADD},
    vcs::{
        files::current_repository::CurrentRepository, version_control_system::VersionControlSystem,
    },
};

pub fn handler_add(input: String) -> String {
    let mut paths: Vec<String> = Vec::new();
    let files: Vec<&str> = input.split_whitespace().collect();

    for path_string in files.iter().skip(2) {
        if *path_string == FULL_ADD {
            if let Ok((untracked, not_commited, _)) = VersionControlSystem::status() {
                for key in untracked.keys() {
                    let _ = VersionControlSystem::add(Path::new(&key));
                }
                for key in not_commited.keys() {
                    let _ = VersionControlSystem::add(Path::new(&key));
                }
            }
            return RESPONSE_OK_ADD.to_string();
        }
        paths.push(path_string.to_string());
    }

    for path_string in &paths {
        if let Ok(current) = CurrentRepository::read() {
            let path = current.join(path_string);
            let _ = VersionControlSystem::add(&path);
        }
    }
    RESPONSE_OK_ADD.to_string()
}
