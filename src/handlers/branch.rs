use crate::vcs::version_control_system::VersionControlSystem;



pub fn handler_branch(vcs: &VersionControlSystem, input: String) -> Result<(), std::io::Error>{
    let args: Vec<&str> = input.split_whitespace().collect();
    vcs.branch(args[2])?;
    Ok(())
}