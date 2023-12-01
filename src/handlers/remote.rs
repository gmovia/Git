use std::path::PathBuf;

use crate::{constants::constant::{RESPONSE_OK_REMOTE, RESPONSE_NOK_REMOTE}, vcs::{version_control_system::VersionControlSystem, commands::remote::RemoteOption}};

pub fn handler_remote(input: String) -> String {    
    let args: Vec<&str> = input.split_whitespace().collect();
    let mut option = RemoteOption::Add;

    if args.iter().any(|arg| arg.contains("add")) {
        let new_repo_name = args[3];
        let server_repo = args[4];
        let path_buf_remote: PathBuf = server_repo.into();
    }
    if args.iter().any(|arg| arg.contains("remove")) {
            option = RemoteOption::Remove;
            let repo_to_remove = args[3];
    }

    if args.iter().any(|arg| arg.contains("get")) {
        option = RemoteOption::Get;
        let repo_to_get = args[3];
    }
    if VersionControlSystem::remote(new_repo_name.to_string(), &path_buf_remote, option).is_ok() {
        return RESPONSE_OK_REMOTE.to_string();  
    }
    RESPONSE_NOK_REMOTE.to_string()
}

//git remote add origin repo1  LEN = 4
//git remote get origin ---> repo1  LEN = 4
//git remote remove origin  --> elimina [remote "origin"] path ....  LEN = 4

