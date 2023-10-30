
use crate::vcs::{version_control_system::VersionControlSystem, commands::branch::BranchOptions};

pub fn handler_branch(vcs: &VersionControlSystem, input: String) -> String{
    let args: Vec<&str> = input.split_whitespace().collect();
    match args.len(){
        4 => {match args[2]{
            "-d" | "-D" => {
                let _ = vcs.branch(BranchOptions::DeleteBranch(args[3]));
                "Deleted successfully.".to_string()
             },
            _ => {"Invalid parameters.".to_string()},
        }},
        3 => {
            let _ = vcs.branch(BranchOptions::NewBranch(args[2]));
            "Created successfully.".to_string()
        },
        2 => {
            if let Ok(branches) = vcs.get_branches(){
                branches.join("\n").to_string()
            }else{
                "Error getting the branches.".to_string()
            }
        },
        _ => "Invalid parameters".to_string(),
    }

}