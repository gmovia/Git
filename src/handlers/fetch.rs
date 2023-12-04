use crate::{
    constants::constant::RESPONSE_OK_FETCH, vcs::version_control_system::VersionControlSystem,
};

pub fn handler_fetch(input: String) -> String {
    let _ = VersionControlSystem::fetch(input);
    RESPONSE_OK_FETCH.to_string()
}
