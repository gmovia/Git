use std::path::PathBuf;

use crate::{vcs::version_control_system::VersionControlSystem, constants::constants::{RESPONSE_OK_CLONE, RESPONSE_NOK_CLONE}};

pub fn handler_clone(input: String) -> String {    
    let message: Vec<&str> = input.split_whitespace().collect();
    let string_path = message[2];
    println!("El path que quiero clonar -----> {}", string_path);
    let path_to_clone = format!("{}", string_path);
    let path_buf_clone: PathBuf = path_to_clone.into();
    if VersionControlSystem::git_clone(input, &path_buf_clone.to_path_buf()).is_ok(){
        return RESPONSE_OK_CLONE.to_string();
    }else{
        return RESPONSE_NOK_CLONE.to_string();
    }
    
}