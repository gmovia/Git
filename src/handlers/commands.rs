use crate::handlers::status::handler_status; 
use crate::handlers::add::handler_add;
use crate::handlers::hash_object::handler_hash_object;
use crate::handlers::cat_file::handler_cat_file;
use crate::handlers::log::handler_log;
use crate::handlers::branch::handler_branch;
use crate::handlers::checkout::handler_checkout;
use crate::handlers::commit::handler_commit;
use crate::handlers::rm::handler_rm;
use crate::vcs::version_control_system::VersionControlSystem;

pub fn handler_command<'a>(input: &str) -> String{
    let input = input.trim(); 
    let _: Vec<String> = input.to_string().split_whitespace().map(|s| s.to_string()).collect();

    match input {
        "git status" => handler_status(),
        x if x.contains("git merge") => {
            if let Ok(_) = VersionControlSystem::merge("new_branch"){
                println!("hola");
                return "Ok".to_string();
            }
            return "Err".to_string();
        },
        x if x.contains("git hash-object") => handler_hash_object(x.to_string()),
        x if x.contains("git add") => handler_add(x.to_string()),
        x if x.contains("git cat-file") => handler_cat_file(x.to_string(), ".rust_git".to_string()),
        x if x.contains("git rm") => handler_rm(x.to_string()),
        x if x.contains("git log") => handler_log(),
        x if x.contains("git commit") => handler_commit(x.to_string()),
        x if x.contains("git branch") => handler_branch(x.to_string()),
        x if x.contains("git checkout") => handler_checkout(x.to_string()),
         _ => "Failed or Panicked.".to_string()
     }
}