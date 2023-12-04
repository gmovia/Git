use crate::{
    constants::constant::RESPONSE_OK_PUSH, vcs::version_control_system::VersionControlSystem,
};

/// Controlador del comando push. Recibe la instruccion junto con una etiqueta y ejecuta el comando.
pub fn handler_push(input: String) -> String {
    let _ = VersionControlSystem::push(input);
    RESPONSE_OK_PUSH.to_string()
}
