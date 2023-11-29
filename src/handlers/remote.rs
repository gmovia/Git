use std::path::PathBuf;

use crate::{constants::constant::{RESPONSE_OK_REMOTE, RESPONSE_NOK_REMOTE}, vcs::version_control_system::VersionControlSystem};

pub fn handler_remote(input: String) -> String {    
    let message: Vec<&str> = input.split_whitespace().collect();
    let new_repo_name = message[3];
    let server_repo = message[4];
    let path_buf_remote: PathBuf = server_repo.into();
    if VersionControlSystem::remote(new_repo_name.to_string(), &path_buf_remote).is_ok() {
        return RESPONSE_OK_REMOTE.to_string();  
    }
       
    RESPONSE_NOK_REMOTE.to_string()
}