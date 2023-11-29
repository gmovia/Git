use crate::{vcs::{version_control_system::VersionControlSystem, files::current_repository::CurrentRepository}, constants::constant::RESPONSE_OK_FETCH};

pub fn handler_fetch(input: String) -> String {
    match CurrentRepository::read() {
        Ok(current) => {
            let _ = VersionControlSystem::fetch(input, current);
            RESPONSE_OK_FETCH.to_string()
        }
        Err(error) => {
            eprintln!("Error reading current repository: {}", error);
            "Error fetching data".to_string()
        }
    }
}
