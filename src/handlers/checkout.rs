use std::{ path::Path, fs};

use crate::{vcs::{version_control_system::VersionControlSystem, commands::checkout::CheckoutOptions, files::current_repository::CurrentRepository}, constants::constant::ERR_INVALID_PARAMETERS};

pub fn handler_checkout(input: String) -> String{
    let args: Vec<&str> = input.split_whitespace().collect();
    if let Ok(current) = CurrentRepository::read() {
        let p = Path::new(&current);
        let branchs_dir_path = p.join(".rust_git").join("refs").join("heads");
        match args.len(){
            4 => {match args[2]{
                "-b" => {let _ = VersionControlSystem::checkout(CheckoutOptions::CreateAndChangeBranch(args[3]));
                        return format!("Create and Change at {}",args[3]).to_string();
                    },
                _ => {return ERR_INVALID_PARAMETERS.to_string();},
            }},
            3 => {if let Ok(entries) = fs::read_dir(branchs_dir_path) {
                let mut _branch_matched = false;
                for entry in entries.flatten() {
                    if let Some(file_name) = entry.path().file_name() {
                        if file_name.to_string_lossy() == args[2] {
                            let _ = VersionControlSystem::checkout(CheckoutOptions::ChangeBranch(args[2]));
                            _branch_matched = true;
                            break;
                        }
                    }
                }
            }
                return format!("Changed successfully at {}",args[2]).to_string();
            },
            _ => {return ERR_INVALID_PARAMETERS.to_string();},
        }
    }
    ERR_INVALID_PARAMETERS.to_string()
}