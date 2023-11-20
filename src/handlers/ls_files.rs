use crate::vcs::version_control_system::VersionControlSystem;
use crate::vcs::commands::ls_files::LsFilesOptions;

pub fn handler_ls_files(input: String) -> String{
    let args: Vec<&str> = input.split_whitespace().collect();
    match args.len() {
        2 => {if let Ok(files) = VersionControlSystem::ls_files(LsFilesOptions::EverythingInVCS) {
            if let Some(entry) = files.into_iter().next() {
                return entry;
            }
                
        }},
        3 => {match args[2] {
            "-m" => {if VersionControlSystem::ls_files(LsFilesOptions::OnlyModified).is_ok() {
                return "OK".to_string();
            }},
            "-c" => {if VersionControlSystem::ls_files(LsFilesOptions::OnlyStaging).is_ok() {
                return "OK".to_string();
            }},
            "-d" => {if VersionControlSystem::ls_files(LsFilesOptions::OnlyDeleted).is_ok() {
                return "OK".to_string();
            }},
            "-o" => {if VersionControlSystem::ls_files(LsFilesOptions::OnlyUntracked).is_ok() {
                return "OK".to_string();
            }}
            _ => {},
        }},
        _ => {},
    }
    
    "ERROR".to_string()
}