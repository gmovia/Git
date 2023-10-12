use std::{io::{self, Write}};
use rust_git::vcs::commands::hash_object::WriteOption;
use rust_git::vcs::version_control_system::VersionControlSystem;

fn main() -> Result<(), std::io::Error>{
    loop{
        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim(); 
        let argss: Vec<String> = input.to_string().split_whitespace().map(|s| s.to_string()).collect();
        match input {
            "git init" => {
                VersionControlSystem::init("nombre_repo".to_string(), input.to_string().split_whitespace().map(|s| s.to_string()).collect());
            },
            "git hash-object -w README.md" => {
                let path = std::path::Path::new("/Users/gmovia/Desktop/T1-RustGit/23C2-4Rust/README.md");
                let result = VersionControlSystem::hash_object(&path, WriteOption::Write)?;
                println!("{:?}", result);
            },
            "git hash-object README.md" => {
                let path = std::path::Path::new("/Users/gmovia/Desktop/T1-RustGit/23C2-4Rust/README.md");
                let result = VersionControlSystem::hash_object(&path, WriteOption::NoWrite)?;
                println!("{:?}", result);
            }
            _ => ()
        }
    }
}
