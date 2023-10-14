use std::io::{self, Write};

use rust_git::vcs::version_control_system::VersionControlSystem;
use rust_git::handlers::{status::handler_status, add::handler_add, hash_object::handler_hash_object, cat_file::handler_cat_file};


fn main() -> Result<(), std::io::Error>{
    //let mut vcs = VersionControlSystem::init("/Users/gmovia/Desktop/PRUEBA", Vec::new());
    //let mut vcs = VersionControlSystem::init(r"C:\Users\Administrator\Desktop\PRUEBA\", Vec::new());
    let mut vcs = VersionControlSystem::init(r"C:\Users\laura\OneDrive\Escritorio\FIUBA\Taller de programacion I\Trabajo practico grupal\PRUEBA", Vec::new());
    loop{
        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim(); 
        let _: Vec<String> = input.to_string().split_whitespace().map(|s| s.to_string()).collect();

        match input {
            "git status" => handler_status(&vcs),
            x if x.contains("git hash-object") => {println!("{:?}",handler_hash_object(&vcs, x.to_string())?);} ,
            x if x.contains("git add") => {handler_add(&mut vcs, x.to_string())?;},
            x if x.contains("git cat-file") => {println!("{:?}",handler_cat_file(&vcs, x.to_string())?);}
            _ => {}
        }
    }
}