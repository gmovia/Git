use std::io;

use crate::vcs::{version_control_system::VersionControlSystem, commands::branch::BranchOptions};



pub fn handler_branch(vcs: &VersionControlSystem, input: String) -> Result<(), std::io::Error>{
    let args: Vec<&str> = input.split_whitespace().collect();
    match args.len(){
        4 => {match args[2]{
            "-d" | "-D" => {vcs.branch(BranchOptions::DeleteBranch(args[3]))?;},
            _ => {return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid parameters"));},
        }},
        3 => {vcs.branch(BranchOptions::NewBranch(args[2]))?;},
        2 => {vcs.branch(BranchOptions::GetBranchs)?;},
        _ => {return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid parameters"));},
    }
    Ok(())
}