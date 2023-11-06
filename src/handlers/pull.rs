use crate::vcs::version_control_system::VersionControlSystem;


/// Recibe input del comando, por ejemplo, git commit "primer commit"
/// Llama a commit() para agregarlo en la tabla de commits
pub fn handler_pull() -> String {
    if let Err(e) = VersionControlSystem::pull() {
        println!("Error al realizar el fetch ({})", e);
    }
    "OK".to_string()
}