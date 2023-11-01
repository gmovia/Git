use crate::{vcs::version_control_system::VersionControlSystem, constants::constants::ERR_NO_SUCH_OR_DIRECTORY};

/// Recibe input del comando, por ejemplo, "git cat-file d6ab2.."
/// Devuelve el contenido del path que contiene el hash.
pub fn handler_cat_file(vcs: &VersionControlSystem, input: String) -> String{
    let args: Vec<&str> = input.split_whitespace().collect();
    if let Ok(result) = vcs.cat_file(args[2]){
        return result;
    }
    ERR_NO_SUCH_OR_DIRECTORY.to_string()
    
}

