use crate::vcs::version_control_system::VersionControlSystem;
use crate::vcs::commands::ls_files::LsFilesOptions;

pub fn handler_ls_files(input: String) -> String{
    let args: Vec<&str> = input.split_whitespace().collect();
    match args.len() {
        2 => {if let Ok(files) = VersionControlSystem::ls_files(LsFilesOptions::EverythingInVCS) {
            for entry in files {
                return entry;
            }
                
        }},
        3 => {match args[2] {
            "-m" => {if let Ok(_) = VersionControlSystem::ls_files(LsFilesOptions::OnlyModified) {
                return "OK".to_string();
            }},
            "-c" => {if let Ok(_) = VersionControlSystem::ls_files(LsFilesOptions::OnlyStaging) {
                return "OK".to_string();
            }},
            "-d" => {if let Ok(_) = VersionControlSystem::ls_files(LsFilesOptions::OnlyDeleted) {
                return "OK".to_string();
            }},
            "-o" => {if let Ok(_) = VersionControlSystem::ls_files(LsFilesOptions::OnlyUntracked) {
                return "OK".to_string();
            }}
            _ => {},
        }},
        _ => {},
    }
    
    "ERROR".to_string()
}