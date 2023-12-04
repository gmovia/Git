use crate::{
    constants::constant::ERR_NO_SUCH_OR_DIRECTORY,
    vcs::version_control_system::VersionControlSystem,
};

/// Controlador del comando cat-file. Recibe la instruccion junto con un hash y ejecuta el comando.
pub fn handler_cat_file(input: String) -> String {
    let args: Vec<&str> = input.split_whitespace().collect();
    if let Ok(result) = VersionControlSystem::cat_file(args[2]) {
        return result;
    }
    ERR_NO_SUCH_OR_DIRECTORY.to_string()
}
