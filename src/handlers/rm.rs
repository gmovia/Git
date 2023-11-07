use crate::{vcs::{version_control_system::VersionControlSystem, commands::rm::RemoveOption}, constants::constants::{RESPONSE_OK_RM, ERR_NO_SUCH_OR_DIRECTORY}};

pub fn handler_rm(input: String) -> String{
    let args: Vec<&str> = input.split_whitespace().collect();
    let mut option = RemoveOption::NoDirectory;

    if args.iter().any(|arg| arg.contains("-r")) {
        option = RemoveOption::Directory;
        if let Ok(current) = VersionControlSystem::read_current_repository() {
            if let Ok(_) = VersionControlSystem::rm(&current.join(args[3]), option.clone()){
                return RESPONSE_OK_RM.to_string();
            }
        }
    }
    if let Ok(current) = VersionControlSystem::read_current_repository() {
        if let Ok(_) = VersionControlSystem::rm(&current.join(args[2]), option){
            return RESPONSE_OK_RM.to_string();
        }
    }
    
    ERR_NO_SUCH_OR_DIRECTORY.to_string()
}