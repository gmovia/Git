use crate::{
    constants::constant::{RESPONSE_NOK_REMOTE, RESPONSE_OK_REMOTE},
    vcs::{commands::remote::RemoteOption, version_control_system::VersionControlSystem},
};
/// Controlador del comando remote. Recibe la instruccion junto con una opcion y/o una etiqueta y ejecuta el comando.
pub fn handler_remote(input: String) -> String {
    let args: Vec<&str> = input.split_whitespace().collect();

    match args.len() {
        4 => {
            let _ = VersionControlSystem::remote(RemoteOption::Add(args[3], args[4]));
            RESPONSE_OK_REMOTE.to_string()
        }
        5 => match args[2] {
            "get" => {
                let _ = VersionControlSystem::remote(RemoteOption::Get(args[3]));
                RESPONSE_OK_REMOTE.to_string()
            }
            "remove" => {
                let _ = VersionControlSystem::remote(RemoteOption::Remove(args[3]));
                RESPONSE_OK_REMOTE.to_string()
            }
            _ => RESPONSE_NOK_REMOTE.to_string(),
        },
        _ => RESPONSE_NOK_REMOTE.to_string(),
    }
}
