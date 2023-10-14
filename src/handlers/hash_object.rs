use std::path::Path;

use crate::vcs::{version_control_system::VersionControlSystem, commands::hash_object::WriteOption};
/// Recibe un input del tipo "git hash-object -w path" o "git hash-object path"
/// Devuelve un hash
pub fn handler_hash_object(vcs: &VersionControlSystem, input: String) -> Result<String, std::io::Error>{
    let args: Vec<&str> = input.split_whitespace().collect();
    let mut vcs_path = String::from(vcs.path.clone());
    vcs_path.push_str(r"\"); // windows barrita al reves
    if args.len() == 4{ // -w
        vcs_path.push_str(args[3]);
        return Ok(vcs.hash_object(Path::new(&vcs_path), WriteOption::Write)?);   
    }
    vcs_path.push_str(args[2]);
    Ok(vcs.hash_object(Path::new(&vcs_path), WriteOption::NoWrite)?)
}