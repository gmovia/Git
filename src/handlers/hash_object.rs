use crate::{vcs::{version_control_system::VersionControlSystem, commands::hash_object::WriteOption}, constants::constants::ERR_PATH_IS_NOT_DIRECTORY_OR_NO_SUCH_OR_DIRECTORY};
/// Recibe un input del tipo "git hash-object -w path" o "git hash-object path"
/// Devuelve un hash
pub fn handler_hash_object(vcs: &VersionControlSystem, input: String) -> String{
    let args: Vec<&str> = input.split_whitespace().collect();
    if args.len() == 4{ // -w
        let input_path = vcs.path.join(args[3]);
        if let Ok(hash) = vcs.hash_object(&input_path, WriteOption::Write){
            return hash;
        }
    }
    let input_path = vcs.path.join(args[2]);
    if let Ok(hash) = vcs.hash_object(&input_path, WriteOption::NoWrite){
        return hash;
    }
    ERR_PATH_IS_NOT_DIRECTORY_OR_NO_SUCH_OR_DIRECTORY.to_string()
}