use crate::{vcs::version_control_system::VersionControlSystem, constants::constant::ERR_NO_SUCH_OR_DIRECTORY};

/// Controlador del comando check-ignore. Recibe la instruccion junto con un path y ejecuta el comando.
pub fn handler_check_ignore(input: String) -> String{
    let args: Vec<&str> = input.split_whitespace().collect();
    let path = Path::new(CurrentRepository::read().join(args[2]));
    VersionControlSystem::check_ignore(path)
}
