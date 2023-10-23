use std::{io, path::Path, fs};

use crate::vcs::{version_control_system::VersionControlSystem, commands::checkout::CheckoutOptions};



pub fn handler_checkout(vcs: &VersionControlSystem, input: String) -> Result<(), std::io::Error>{
    let args: Vec<&str> = input.split_whitespace().collect();
    let p = Path::new(&vcs.path);
    let branchs_dir_path = p.join(".rust_git").join("refs").join("heads");
    match args.len(){
        4 => {match args[2]{
            "-b" => {vcs.checkout(CheckoutOptions::CreateAndChangeBranch(args[3]))?;},
            _ => {return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid parameters"));},
        }},
        3 => {if let Ok(entries) = fs::read_dir(branchs_dir_path) {
            let mut _branch_matched = false;
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(file_name) = entry.path().file_name() {
                        if file_name.to_string_lossy().to_string() == args[2].to_string() {
                            vcs.checkout(CheckoutOptions::ChangeBranch(&args[2].to_string()))?;
                            _branch_matched = true;
                            break;
                        }
                    }
                }
            }
            // if !branch_matched {
            //     vcs.checkout(CheckoutOptions::ChangeCommit(args[2]))?;
            // }
        }
        },
        _ => {return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid parameters"));},
    }
    Ok(())
}