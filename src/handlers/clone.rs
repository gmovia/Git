use crate::{vcs::version_control_system::VersionControlSystem, constants::constant::RESPONSE_OK_CLONE};

pub fn handler_clone(input: String) -> String {    
    let _ =  VersionControlSystem::git_clone(input);
    RESPONSE_OK_CLONE.to_string()
}