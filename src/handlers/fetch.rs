use crate::vcs::version_control_system::VersionControlSystem;


/// Recibe input del comando, por ejemplo, git commit "primer commit"
/// Llama a commit() para agregarlo en la tabla de commits
pub fn handler_fetch(input: String) -> Result<(), std::io::Error>{
    let args: Vec<&str> = input.split(" ").collect();
    let server_repo = args[0];
    if let Err(e) = VersionControlSystem::fetch(server_repo.to_owned()) {
        println!("Error al realizar el fetch ({})", e);
    }
    Ok(())
}