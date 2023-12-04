use crate::{
    constants::constant::RESPONSE_OK_PULL, vcs::version_control_system::VersionControlSystem,
};

/// Controlador del comando pull. Recibe la instruccion junto con una etiqueta y ejecuta el comando.
pub fn handler_pull(input: String) -> String {
    let _ = VersionControlSystem::pull(input);

    RESPONSE_OK_PULL.to_string()
}
