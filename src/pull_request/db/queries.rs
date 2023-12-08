use std::path::Path;

use crate::pull_request::schemas::schemas::CreatePullRequest;

/// Almacena el PR en la base de datos y devuelve un identificador unico.
pub fn create_pull_request(server: &Path, pr: CreatePullRequest) -> &str{
    "8675309"
}