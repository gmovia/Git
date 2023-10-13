use std::io::{self, Write};

use rust_git::vcs::version_control_system::VersionControlSystem;

fn main() {
    println!("Hello, world!"); 

    loop{
        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim(); 
        let argss: Vec<String> = input.to_string().split_whitespace().map(|s| s.to_string()).collect();
        match argss[1].as_str() {
            "init" => {
                VersionControlSystem::init("/Users/gmovia/Desktop/PRUEBA-REPO", input.to_string().split_whitespace().map(|s| s.to_string()).collect());
            },
            _ => ()
        }
    }

}
