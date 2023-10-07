use std::env;

use rust_git::{repository::{self, Repository}, commands::init::Init, version_control_system::{self, VersionControlSystem}};

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Please complete the arguments");
    }

    match &args[1] as &str {
        "init" => {
            let version_control_system = VersionControlSystem::init("repository_name", args[2..].to_vec());
        }
        _ => {
            println!("Opción no reconocida.");
        }
    
    } 

}
