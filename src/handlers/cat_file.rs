use crate::{vcs::version_control_system::VersionControlSystem, constants::constants::ERR_NO_SUCH_OR_DIRECTORY};

pub fn handler_cat_file(input: String) -> String{
    let args: Vec<&str> = input.split_whitespace().collect();
    if let Ok(result) = VersionControlSystem::cat_file(args[2]) {
        return result;
    }
    ERR_NO_SUCH_OR_DIRECTORY.to_string()   
}
