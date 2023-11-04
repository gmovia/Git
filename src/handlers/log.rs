use crate::constants::constants::ERR_COMMIT_IS_NOT_EXIST;
use crate::vcs::commands::init::Init;
use crate::vcs::version_control_system::VersionControlSystem;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn handler_log(vcs: &VersionControlSystem) -> String {
    if let Ok(path) = Init::get_commits_path(&vcs.path) {
        if let Ok(commits_file) = File::open(path){
            let reader = BufReader::new(commits_file);
            let has_commits = reader.lines().count() > 0;
            if has_commits {
                if let Ok(result) = vcs.log(){
                    return result;
                }
            }
        }
    }
    ERR_COMMIT_IS_NOT_EXIST.to_string()
}
