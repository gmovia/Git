use rust_git::vcs::version_control_system::VersionControlSystem;
use rust_git::vcs::commands::hash_object::WriteOption;

use std::{io::{self, Write}, path::Path};

fn handle_status(vcs: &VersionControlSystem){
    if let Ok((untracked, not_commited, commited)) = vcs.status(){
        println!("UNTRACKED");
        for (key, value) in untracked{
            println!("{} {}", key, value);
        }
        println!("NOT COMMITED");
        for (key, value) in not_commited{
            println!("{} {}", key, value);
        }
        println!("AREA");
        for (key, value) in commited{
            println!("{} {}", key, value);
        }
    }
}

fn handle_add(vcs: &mut VersionControlSystem, input: String) -> Result<(), std::io::Error>{
    let mut paths: Vec<String> = Vec::new();
    let files: Vec<&str> = input.split_whitespace().collect();

    for path_string in files.iter().skip(2) {
        if path_string.to_string() == "."{
            if let Ok((untracked, not_commited, _)) = vcs.status(){
                for (key, _) in untracked{
                    let _ = vcs.add(Path::new(&key));
                }
                for (key, _) in not_commited{
                    let _ = vcs.add(Path::new(&key));
                }
            }
            return Ok(());
        }
        paths.push(path_string.to_string());
    }

    for path_string in &paths{
        let mut path_file = String::new();
        
        let vcs_path = vcs.path.clone();
        
        path_file.push_str(&vcs_path);
        path_file.push_str(r"\"); //es para windows
        //path_file.push_str("/"); //es para linux
        path_file.push_str(&path_string);

        let path = Path::new(&path_file);
        let _ = vcs.add(path);
    }
    Ok(())
}


fn main() -> Result<(), std::io::Error>{
    //let mut vcs = VersionControlSystem::init("/Users/gmovia/Desktop/PRUEBA", Vec::new());
    //let mut vcs = VersionControlSystem::init(r"C:\Users\Administrator\Desktop\PRUEBA\", Vec::new());
    //let mut vcs = VersionControlSystem::init(r"C:\Users\laura\OneDrive\Escritorio\FIUBA\Taller de programacion I\Trabajo practico grupal\PRUEBA", Vec::new());
    loop{
        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim(); 
        let _: Vec<String> = input.to_string().split_whitespace().map(|s| s.to_string()).collect();
        let vcs = VersionControlSystem::init("/Users/gmovia/Desktop/PRUEBA-REPO", input.to_string().split_whitespace().map(|s| s.to_string()).collect());

        match input {
            "git hash-object -w README.md" => {
                let path = std::path::Path::new("/Users/gmovia/Desktop/T1-RustGit/23C2-4Rust/README.md");
                let result = VersionControlSystem::hash_object(&path, WriteOption::Write)?;
                println!("{:?}", result);
            },
            "git hash-object README.md" => {
                let path = std::path::Path::new("/Users/gmovia/Desktop/T1-RustGit/23C2-4Rust/README.md");
                let result = VersionControlSystem::hash_object(&path, WriteOption::NoWrite)?;
                println!("{:?}", result);
            },
            "git cat-file" => {
                let path = std::path::Path::new("/Users/gmovia/Desktop/T1-RustGit/23C2-4Rust/README.md");
                let hash = VersionControlSystem::hash_object(&path, WriteOption::NoWrite)?;
                let result = VersionControlSystem::cat_file(&hash)?;
                println!("{:?}", result);
            },
            "git status" => handle_status(&vcs),
              x if x.contains("git add") => {
                  if let Err(err) = handle_add(&mut vcs, x.to_string()) {
                      eprintln!("Error al ejecutar 'git add': {}", err);
                  }
              },
            _ => ()
        }
    }
}

