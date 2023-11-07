use crate::{vcs::{version_control_system::VersionControlSystem, commands::hash_object::WriteOption}, constants::constants::ERR_PATH_IS_NOT_DIRECTORY_OR_NO_SUCH_OR_DIRECTORY};

pub fn handler_hash_object(input: String) -> String{
    let args: Vec<&str> = input.split_whitespace().collect();
    if args.len() == 4{ // -w
        if let Ok(current) = VersionControlSystem::read_current_repository() {
            let input_path = current.join(args[3]);
            if let Ok(hash) = VersionControlSystem::hash_object(&input_path, WriteOption::Write){
                return hash;
            }
        }
        
    }
    if let Ok(current) = VersionControlSystem::read_current_repository() {
        let input_path = current.join(args[2]);
        if let Ok(hash) = VersionControlSystem::hash_object(&input_path, WriteOption::NoWrite){
            return hash;
        }
    }
    
    ERR_PATH_IS_NOT_DIRECTORY_OR_NO_SUCH_OR_DIRECTORY.to_string()
}