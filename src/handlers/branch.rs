use crate::{
    constants::constant::{
        ERR_GET_BRANCHES, ERR_INVALID_PARAMETERS, RESPONSE_OK_CREATE_BRANCH,
        RESPONSE_OK_DELETED_BRANCH,
    },
    vcs::{commands::branch::BranchOptions, version_control_system::VersionControlSystem},
};

/// Controlador del comando branch. Recibe la instruccion junto con una opcion y ejecuta el comando.
pub fn handler_branch(input: String) -> String {
    let args: Vec<&str> = input.split_whitespace().collect();
    match args.len() {
        4 => match args[2] {
            "-d" | "-D" => {
                let _ = VersionControlSystem::branch(BranchOptions::DeleteBranch(args[3]));
                RESPONSE_OK_DELETED_BRANCH.to_string()
            }
            _ => ERR_INVALID_PARAMETERS.to_string(),
        },
        3 => match args[2] {
            "-a" => {
                if let Ok(result) = VersionControlSystem::branch(BranchOptions::GetCurrentBranch) {
                    let mut content = String::new();
                    for r in &result {
                        content.push_str(&format!("{}\n", r));
                    }
                    content
                } else {
                    ERR_INVALID_PARAMETERS.to_string()
                }
            }
            _ => {
                let _ = VersionControlSystem::branch(BranchOptions::NewBranch(args[2]));
                RESPONSE_OK_CREATE_BRANCH.to_string()
            }
        },
        2 => {
            if let Ok(branches) = VersionControlSystem::branch(BranchOptions::GetBranches) {
                branches.join("\n").to_string()
            } else {
                ERR_GET_BRANCHES.to_string()
            }
        }
        _ => ERR_INVALID_PARAMETERS.to_string(),
    }
}
