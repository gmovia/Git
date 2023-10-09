use rust_git::vcs::version_control_system::VersionControlSystem;
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
        path_file.push_str("/");
        path_file.push_str(&path_string);

        let path = Path::new(&path_file);
        let _ = vcs.add(path);
    }
    Ok(())
}

fn main(){
    let mut vcs = VersionControlSystem::init("/Users/gmovia/Desktop/T1-RustGit/23C2-4Rust".to_string());
    
    loop{
        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input{
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