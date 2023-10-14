use std::path::Path;

use crate::vcs::version_control_system::VersionControlSystem;

pub fn handler_add(vcs: &mut VersionControlSystem, input: String) -> Result<(), std::io::Error>{
    let mut paths: Vec<String> = Vec::new();
    let files: Vec<&str> = input.split_whitespace().collect();

    for path_string in files.iter().skip(2) {
        if path_string.to_string() == "."{
            if let Ok((untracked, not_commited, _)) = vcs.status(){
                for (key, _) in untracked{
                    let _ = vcs.add(Path::new(&key));
                }
                for (key, _) in not_commited{
                    let _ = vcs.add(Path::new(&key));
                }
            }
            return Ok(());
        }
        paths.push(path_string.to_string());
    }

    for path_string in &paths{
        let mut path_file = String::new();
        
        let vcs_path = vcs.path.clone();
        
        path_file.push_str(&vcs_path);
        path_file.push_str(r"\"); //es para windows
        //path_file.push_str("/"); //es para linux
        path_file.push_str(&path_string);

        let path = Path::new(&path_file);
        let _ = vcs.add(path);
    }
    Ok(())
}