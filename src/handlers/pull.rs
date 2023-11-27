use crate::{vcs::version_control_system::VersionControlSystem, constants::constant::RESPONSE_OK_PULL};

pub fn handler_pull(input: String) -> String {    
    let _ =  VersionControlSystem::git_pull(input);
    RESPONSE_OK_PULL.to_string()
}