
use crate::{vcs::version_control_system::VersionControlSystem, constants::constants::RESPONSE_OK_PUSH};

pub fn handler_push(input: String) -> String{
    let _ =  VersionControlSystem::push(input);
    RESPONSE_OK_PUSH.to_string()
}