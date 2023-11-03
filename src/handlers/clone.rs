use crate::vcs::version_control_system::VersionControlSystem;

/// Recibe input del comando, por ejemplo, git commit "primer commit"
/// Llama a commit() para agregarlo en la tabla de commits
pub fn handler_clone(vcs: &mut VersionControlSystem, input: String) -> Result<(), std::io::Error>{
    let mut args: Vec<&str> = input.split(" ").collect();
    args.remove(1);
    args.remove(0);
    let puerto = args[0];
    let host = args[1];
    let server_repo = args[2];

    vcs.clone(puerto.to_string(), host.to_string(), server_repo.to_string())?;
    
    Ok(())
}