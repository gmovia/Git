use crate::vcs::version_control_system::VersionControlSystem;

/// Recibe input del comando, por ejemplo, git commit "primer commit"
/// Llama a commit() para agregarlo en la tabla de commits
pub fn handler_clone(input: String) -> String {
    println!("INPUT: {}", input);
    if let Ok(_) = VersionControlSystem::git_clone(input) {
        return "ok".to_string();
    }
    "ERROR".to_string()
}