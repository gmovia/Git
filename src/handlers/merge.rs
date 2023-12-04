use crate::{
    constants::constant::{ERR_MERGE, RESPONSE_OK_MERGE},
    vcs::version_control_system::VersionControlSystem,
};

pub fn handler_merge(input: String) -> String {
    let args: Vec<&str> = input.split_whitespace().collect();
    if VersionControlSystem::merge(args[2]).is_ok() {
        return RESPONSE_OK_MERGE.to_string();
    }
    ERR_MERGE.to_string()
}
