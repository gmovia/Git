use std::path::PathBuf;

use crate::{vcs::version_control_system::VersionControlSystem, constants::constant::{RESPONSE_OK_CLONE, RESPONSE_NOK_CLONE}};

pub fn handler_clone(input: String) -> String {    
    let message: Vec<&str> = input.split_whitespace().collect();
    let string_path = message[2];
    let path_buf_clone: PathBuf = string_path.into();
    if VersionControlSystem::git_clone(input, &path_buf_clone).is_ok(){
        RESPONSE_OK_CLONE.to_string()
    }else{
        RESPONSE_NOK_CLONE.to_string()
    }
    
}