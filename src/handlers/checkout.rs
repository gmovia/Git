use std::{ path::Path, fs};

use crate::{vcs::{version_control_system::VersionControlSystem, commands::checkout::CheckoutOptions}, constants::constants::ERR_INVALID_PARAMETERS};

pub fn handler_checkout(vcs: &VersionControlSystem, input: String) -> String{
    let args: Vec<&str> = input.split_whitespace().collect();
    let p = Path::new(&vcs.path);
    let branchs_dir_path = p.join(".rust_git").join("refs").join("heads");
    match args.len(){
        4 => {match args[2]{
            "-b" => {let _ = vcs.checkout(CheckoutOptions::CreateAndChangeBranch(args[3]));
                    return format!("Create and Change at {}",args[3]).to_string();
                },
            _ => ERR_INVALID_PARAMETERS.to_string(),
        }},
        3 => {if let Ok(entries) = fs::read_dir(branchs_dir_path) {
            let mut _branch_matched = false;
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(file_name) = entry.path().file_name() {
                        if file_name.to_string_lossy().to_string() == args[2].to_string() {
                            let _ = vcs.checkout(CheckoutOptions::ChangeBranch(&args[2].to_string()));
                            _branch_matched = true;
                            break;
                        }
                    }
                }
            }
        }
        format!("Changed successfully at {}",args[2]).to_string()
        },
        _ => ERR_INVALID_PARAMETERS.to_string(),
    }
}