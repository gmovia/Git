use crate::{
    constants::constant::RESPONSE_OK_PUSH, vcs::version_control_system::VersionControlSystem,
};

pub fn handler_push(input: String) -> String {
    let _ = VersionControlSystem::push(input);
    RESPONSE_OK_PUSH.to_string()
}
