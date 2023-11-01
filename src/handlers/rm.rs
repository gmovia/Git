use crate::{vcs::{version_control_system::VersionControlSystem, commands::rm::RemoveOption}, constants::constants::{RESPONSE_OK_RM, ERR_NO_SUCH_OR_DIRECTORY}};

/// Recibe input del comando rm puede ser "rm path" o "rm -r path"
/// Se setea la option correspondiente de acuerdo al comando 
pub fn handler_rm(vcs: &VersionControlSystem, input: String) -> String{
    let args: Vec<&str> = input.split_whitespace().collect();
    let mut option = RemoveOption::NoDirectory;

    if args.iter().any(|arg| arg.contains("-r")) {
        option = RemoveOption::Directory;
        println!("{:?}",args);
        if let Ok(_) = vcs.rm(&vcs.path.join(args[3]), option.clone()){
            return RESPONSE_OK_RM.to_string();
        }
        
    }
    if let Ok(_) = vcs.rm(&vcs.path.join(args[2]), option){
        return RESPONSE_OK_RM.to_string();
    }
    ERR_NO_SUCH_OR_DIRECTORY.to_string()
}