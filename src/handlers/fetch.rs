use crate::vcs::version_control_system::VersionControlSystem;


/// Recibe input del comando, por ejemplo, git commit "primer commit"
/// Llama a commit() para agregarlo en la tabla de commits
pub fn handler_fetch(vcs: &mut VersionControlSystem, input: String) -> Result<(), std::io::Error>{
    if let Err(e) = vcs.fetch(input) {
        println!("Error al realizar el fetch ({})", e);
    }
    Ok(())
}