
use crate::{vcs::{version_control_system::VersionControlSystem, commands::branch::BranchOptions}, constants::constants::{RESPONSE_OK_DELETED_BRANCH, ERR_INVALID_PARAMETERS, RESPONSE_OK_CREATE_BRANCH, ERR_GET_BRANCHES}};

pub fn handler_branch(vcs: &VersionControlSystem, input: String) -> String{
    let args: Vec<&str> = input.split_whitespace().collect();
    match args.len(){
        4 => {match args[2]{
            "-d" | "-D" => {
                let _ = vcs.branch(BranchOptions::DeleteBranch(args[3]));
                RESPONSE_OK_DELETED_BRANCH.to_string()
             },
            _ => {ERR_INVALID_PARAMETERS.to_string()},
        }},
        3 => {
            match args[2]{
            "-a" => {
                if let Ok(result) = vcs.branch(BranchOptions::GetCurrentBranch){
                    let mut content = String::new();
                    for r in &result{
                        content.push_str(&format!("{}\n",r));
                        println!("{:?}",r);
                    }
                    println!("{:?}",content);
                    return content;
                }else{
                    ERR_INVALID_PARAMETERS.to_string()
                }
            }
            _ => {let _ = vcs.branch(BranchOptions::NewBranch(args[2]));
                RESPONSE_OK_CREATE_BRANCH.to_string()}
        }},
        2 => {
            if let Ok(branches) = vcs.get_branches(){
                branches.join("\n").to_string()
            }else{
                ERR_GET_BRANCHES.to_string()
            }
        },
        _ => ERR_INVALID_PARAMETERS.to_string(),
    }

}