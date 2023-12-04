use std::path::PathBuf;

use crate::{
    constants::constant::{RESPONSE_NOK_CLONE, RESPONSE_OK_CLONE},
    vcs::version_control_system::VersionControlSystem,
};

pub fn handler_clone(input: String) -> String {
    let message: Vec<&str> = input.split_whitespace().collect();
    let string_path = message[2];
    let path_buf_clone: PathBuf = string_path.into();
    if VersionControlSystem::clone(input, &path_buf_clone).is_ok() {
        RESPONSE_OK_CLONE.to_string()
    } else {
        RESPONSE_NOK_CLONE.to_string()
    }
}
