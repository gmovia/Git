use crate::vcs::version_control_system::VersionControlSystem;

/// Recibe input del comando, por ejemplo, git commit "primer commit"
/// Llama a commit() para agregarlo en la tabla de commits
pub fn handler_clone(input: String) -> Result<(), std::io::Error>{
    VersionControlSystem::git_clone(input)?;    
    Ok(())
}