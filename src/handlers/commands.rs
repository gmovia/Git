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

pub fn handler_command(vcs: &mut VersionControlSystem, input: &str) -> Result<(), std::io::Error>{
    let input = input.trim(); 
    let _: Vec<String> = input.to_string().split_whitespace().map(|s| s.to_string()).collect();

    match input {
        "git status" => handler_status(vcs),
        x if x.contains("git hash-object") => {println!("{:?}",handler_hash_object(vcs, x.to_string())?);} ,
        x if x.contains("git add") => {handler_add(vcs, x.to_string())?;},
        x if x.contains("git cat-file") => {println!("{:?}",handler_cat_file(vcs, x.to_string())?);},
        x if x.contains("git rm") => {handler_rm(vcs, x.to_string())?;},
        x if x.contains("git log") => {let _ = handler_log(vcs);},
        x if x.contains("git commit") => {handler_commit(vcs, x.to_string())?;},
        x if x.contains("git branch") => {handler_branch(vcs, x.to_string())?;},
        x if x.contains("git checkout") => {handler_checkout(vcs, x.to_string())?;},
         _ => {}
     }
     Ok(())
}