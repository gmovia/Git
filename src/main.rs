use std::io::{self, Write};
use std::path::Path;

use rust_git::handlers::branch::handler_branch;
use rust_git::handlers::checkout::handler_checkout;
use rust_git::handlers::commit::handler_commit;
use rust_git::handlers::rm::handler_rm;
use rust_git::vcs::version_control_system::VersionControlSystem;
use rust_git::handlers::{status::handler_status, add::handler_add, hash_object::handler_hash_object, cat_file::handler_cat_file, log::handler_log};


fn main() -> Result<(), std::io::Error>{
    let mut vcs = VersionControlSystem::init(Path::new("test_folder"), Vec::new());
    
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
            x if x.contains("git cat-file") => {println!("{:?}",handler_cat_file(&vcs, x.to_string())?);},
            x if x.contains("git rm") => {handler_rm(&mut vcs, x.to_string())?;},
            x if x.contains("git log") => {let _ = handler_log(&vcs);},
            x if x.contains("git commit") => {handler_commit(&mut vcs, x.to_string())?;},
            x if x.contains("git branch") => {handler_branch(&vcs, x.to_string())?;},
            x if x.contains("git checkout") => {handler_checkout(&vcs, x.to_string())?;},
            _ => {}
        }
    }
}