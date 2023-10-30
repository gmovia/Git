use crate::vcs::version_control_system::VersionControlSystem;

/// Recibe input del comando, por ejemplo, "git cat-file d6ab2.."
/// Devuelve el contenido del path que contiene el hash.
pub fn handler_cat_file(vcs: &VersionControlSystem, input: String) -> String{
    let args: Vec<&str> = input.split_whitespace().collect();
    if let Ok(result) = vcs.cat_file(args[2]){
        return result;
    }
    "No such file or directory".to_string()
    
}

