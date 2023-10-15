use std::path::Path;

use crate::vcs::{version_control_system::VersionControlSystem, commands::rm::RemoveOption};

/// Recibe input del comando rm puede ser "rm path" o "rm -r path"
/// Se setea la option correspondiente de acuerdo al comando 
pub fn handler_rm(vcs: &mut VersionControlSystem, input: String) -> Result<(), std::io::Error>{
    let args: Vec<&str> = input.split_whitespace().collect();
    let vcs_path = String::from(vcs.path.clone());
    let mut option = RemoveOption::NoDirectory;

    if args.iter().any(|arg| arg.contains("-r")) {
        option = RemoveOption::Directory;
    }

    vcs.rm(Path::new(&vcs_path), option)?;
    Ok(())
}