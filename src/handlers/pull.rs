use crate::{vcs::version_control_system::VersionControlSystem, constants::constants::RESPONSE_OK_PULL};

pub fn handler_pull() -> String {    
    let _ =  VersionControlSystem::git_pull();
    RESPONSE_OK_PULL.to_string()
}