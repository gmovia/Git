use crate::{
    constants::constant::{ERR_NO_SUCH_OR_DIRECTORY, RESPONSE_OK_RM},
    vcs::{
        commands::rm::RemoveOption, files::current_repository::CurrentRepository,
        version_control_system::VersionControlSystem,
    },
};

/// Controlador del comando rm. Recibe la instruccion junto con una opcion y un path y ejecuta el comando.
pub fn handler_rm(input: String) -> String {
    let args: Vec<&str> = input.split_whitespace().collect();
    let mut option = RemoveOption::NoDirectory;

    if args.iter().any(|arg| arg.contains("-r")) {
        option = RemoveOption::Directory;
        if let Ok(current) = CurrentRepository::read() {
            if VersionControlSystem::rm(&current.join(args[3]), option.clone()).is_ok() {
                return RESPONSE_OK_RM.to_string();
            } else {
                return ERR_NO_SUCH_OR_DIRECTORY.to_string();
            }
        }
    }
    if let Ok(current) = CurrentRepository::read() {
        if VersionControlSystem::rm(&current.join(args[2]), option).is_ok() {
            return RESPONSE_OK_RM.to_string();
        } else {
            return ERR_NO_SUCH_OR_DIRECTORY.to_string();
        }
    }

    ERR_NO_SUCH_OR_DIRECTORY.to_string()
}
