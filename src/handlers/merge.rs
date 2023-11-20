use crate::{vcs::version_control_system::VersionControlSystem, constants::constants::{RESPONSE_OK_MERGE, ERR_MERGE}};

pub fn handler_merge(input: String) -> String{
    let args: Vec<&str> = input.split_whitespace().collect();
    if VersionControlSystem::merge(args[2]).is_ok(){
        return RESPONSE_OK_MERGE.to_string();
    }
    ERR_MERGE.to_string()
}
