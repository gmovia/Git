use std::path::PathBuf;

use crate::{constants::constant::{RESPONSE_OK_REMOTE, RESPONSE_NOK_REMOTE}, vcs::{version_control_system::VersionControlSystem, commands::remote::RemoteOption}};


pub fn handler_remote(input: String) -> String {
    let args: Vec<&str> = input.split_whitespace().collect();

    match args.len() {
        4 => {let _ = VersionControlSystem::remote(RemoteOption::Add(args[3], args[4]));
            RESPONSE_OK_REMOTE.to_string()},
        5 => {match args[2] {
            "get" => {let _ = VersionControlSystem::remote(RemoteOption::Get(args[3]));
                RESPONSE_OK_REMOTE.to_string()},
            "remove" => {let _ = VersionControlSystem::remote(RemoteOption::Remove(args[3]));
                RESPONSE_OK_REMOTE.to_string()},
            _ => {RESPONSE_NOK_REMOTE.to_string()},
        }},
        _ => {RESPONSE_NOK_REMOTE.to_string()},
    }
}

//git remote add origin repo1  LEN = 4
//git remote get origin ---> repo1  LEN = 4
//git remote remove origin  --> elimina [remote "origin"] path ....  LEN = 4

