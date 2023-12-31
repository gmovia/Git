use crate::vcs::version_control_system::VersionControlSystem;

/// Controlador del comando ls-tree. Recibe la instruccion junto con una opcion y ejecuta el comando.
pub fn handler_ls_tree(input: String) -> String {
    let args: Vec<&str> = input.split_whitespace().collect();
    match args.len() {
        3 => {
            let _ = VersionControlSystem::ls_tree(args[2]);
            "Ok".to_string()
        }
        _ => "Error".to_string(),
    }
}
