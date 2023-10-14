use crate::vcs::version_control_system::VersionControlSystem;

pub fn handler_status(vcs: &VersionControlSystem){
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