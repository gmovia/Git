use std::path::Path;

use crate::{vcs::version_control_system::VersionControlSystem, constants::constants::{FULL_ADD, RESPONSE_OK_ADD}};

pub fn handler_add(vcs: & VersionControlSystem, input: String) -> String{
    let mut paths: Vec<String> = Vec::new();
    let files: Vec<&str> = input.split_whitespace().collect();

    for path_string in files.iter().skip(2) {
        if path_string.to_string() == FULL_ADD{
            if let Ok((untracked, not_commited, _)) = vcs.status(){
                for (key, _) in untracked{
                    let _ = vcs.add(Path::new(&key));
                }
                for (key, _) in not_commited{
                    let _ = vcs.add(Path::new(&key));
                }
            }
            return RESPONSE_OK_ADD.to_string();
        }
        paths.push(path_string.to_string());
    }

    for path_string in &paths{
        let path = vcs.path.join(path_string);
        let _ = vcs.add(&path);
    }
    RESPONSE_OK_ADD.to_string()
}
