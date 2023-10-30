use crate::vcs::{version_control_system::VersionControlSystem, commands::rm::RemoveOption};

/// Recibe input del comando rm puede ser "rm path" o "rm -r path"
/// Se setea la option correspondiente de acuerdo al comando 
pub fn handler_rm(vcs: &mut VersionControlSystem, input: String) -> String{
    let args: Vec<&str> = input.split_whitespace().collect();
    let mut option = RemoveOption::NoDirectory;

    if args.iter().any(|arg| arg.contains("-r")) {
        option = RemoveOption::Directory;
        if let Ok(_) = vcs.rm(&vcs.path.join(args[3]), option.clone()){
            return "Removed successfully.".to_string();
        }
        
    }
    if let Ok(_) = vcs.rm(&vcs.path.join(args[2]), option){
        return "Removed successfully.".to_string();
    }
    "No such file or directory".to_string()
}