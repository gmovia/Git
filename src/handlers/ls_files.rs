use crate::vcs::version_control_system::VersionControlSystem;
use crate::vcs::commands::ls_files::LsFilesOptions;

pub fn handler_ls_files(input: String) -> String{
    let args: Vec<&str> = input.split_whitespace().collect();
    match args.len() {
        2 => {if let Ok(files) = VersionControlSystem::ls_files(LsFilesOptions::Everything) {
                println!("VECTOR: {:?}",files);
                return "OK".to_string();
        }},
        3 => {match args[2] {
            "-m" => {},
            "-c" => {},
            "-d" => {},
            _ => {},
        }},
        _ => {},
    }
    
    "ERROR".to_string()
}