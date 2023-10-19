use crate::vcs::{version_control_system::VersionControlSystem, commands::hash_object::WriteOption};
/// Recibe un input del tipo "git hash-object -w path" o "git hash-object path"
/// Devuelve un hash
pub fn handler_hash_object(vcs: &VersionControlSystem, input: String) -> Result<String, std::io::Error>{
    let args: Vec<&str> = input.split_whitespace().collect();
    if args.len() == 4{ // -w
        let input_path = vcs.path.join(args[3]);
        return Ok(vcs.hash_object(&input_path, WriteOption::Write)?);   
    }
    let input_path = vcs.path.join(args[2]);
    Ok(vcs.hash_object(&input_path, WriteOption::NoWrite)?)
}