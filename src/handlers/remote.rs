use std::path::PathBuf;

use crate::{constants::constant::{RESPONSE_OK_REMOTE, RESPONSE_NOK_REMOTE}, vcs::{version_control_system::VersionControlSystem, commands::remote::RemoteOption}};


pub fn handler_remote(input: String) -> String {
    let args: Vec<&str> = input.split_whitespace().collect();
    let mut option = RemoteOption::Add;
    let mut path_to_process = PathBuf::new();
    let mut repo_name_to_process = "";

    if args.iter().any(|arg| arg.contains("add")) {
        option = RemoteOption::Add;
        if args.len() > 4 {
            repo_name_to_process = args[3];
            path_to_process = PathBuf::from(args[4]);
        }
    } else if args.iter().any(|arg| arg.contains("remove")) {
        option = RemoteOption::Remove;
        println!("ARGS ES {}", args[3]);
        if args.len() > 3 {
            repo_name_to_process = args[3];
        }
    } else if args.iter().any(|arg| arg.contains("get")) {
        option = RemoteOption::Get;
        if args.len() > 3 {
            repo_name_to_process = args[3];
        }
    }
    match VersionControlSystem::remote(repo_name_to_process.to_string(), &path_to_process, option) {
        Ok(_) => RESPONSE_OK_REMOTE.to_string(),
        Err(_) => RESPONSE_NOK_REMOTE.to_string(),
    }
}

//git remote add origin repo1  LEN = 4
//git remote get origin ---> repo1  LEN = 4
//git remote remove origin  --> elimina [remote "origin"] path ....  LEN = 4

