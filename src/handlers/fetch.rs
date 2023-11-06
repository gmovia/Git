use crate::vcs::version_control_system::VersionControlSystem;


/// Recibe input del comando, por ejemplo, git commit "primer commit"
/// Llama a commit() para agregarlo en la tabla de commits
pub fn handler_fetch(input: String) -> String {
    if let Err(e) = VersionControlSystem::fetch(input) {
        println!("Error al realizar el fetch ({})", e);
    }
    "OK".to_string()
}