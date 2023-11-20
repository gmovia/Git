use crate::{vcs::version_control_system::VersionControlSystem, constants::constant::{RESPONSE_OK_MERGE, ERR_MERGE}};

pub fn handler_merge(input: String) -> String{
    let args: Vec<&str> = input.split_whitespace().collect();
    if let Ok(_) = VersionControlSystem::merge(args[2]){
        return RESPONSE_OK_MERGE.to_string();
    }
    return ERR_MERGE.to_string();
}
