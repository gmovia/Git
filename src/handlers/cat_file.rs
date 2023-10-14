use crate::vcs::version_control_system::VersionControlSystem;

/// Recibe input del comando, por ejemplo, "git cat-file d6ab2.."
/// Devuelve el contenido del path que contiene el hash.
pub fn handler_cat_file(vcs: &VersionControlSystem, input: String) -> Result<String, std::io::Error>{
    let args: Vec<&str> = input.split_whitespace().collect();
    Ok(vcs.cat_file(args[2])?)
}