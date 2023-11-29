use crate::{vcs::{version_control_system::VersionControlSystem, files::current_repository::CurrentRepository}, constants::constant::RESPONSE_OK_FETCH};

pub fn handler_fetch(input: String) -> String {
       
    let _ = VersionControlSystem::fetch(input);
    RESPONSE_OK_FETCH.to_string()

}
