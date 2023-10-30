use crate::vcs::{version_control_system::VersionControlSystem, commands::hash_object::WriteOption};
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
    "The path is an directory or no such file or directory.".to_string()
}