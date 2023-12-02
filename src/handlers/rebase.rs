use crate::vcs::version_control_system::VersionControlSystem;

pub fn handler_rebase(input: String) -> String {

    let args: Vec<&str> = input.split_whitespace().collect();

    if VersionControlSystem::rebase(args[2]).is_ok() {
        return "Ok".to_string();
    }

    "Err".to_string()
}