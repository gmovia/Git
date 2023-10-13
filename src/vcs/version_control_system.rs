use crate::vcs::{
    files::vcs_file::VCSFile,
    repository::Repository, commands::init::Init,
};
use std::collections::HashMap;

pub struct VersionControlSystem {
    pub path: String,
    pub local_repository: Repository,
    pub staging_area: HashMap<String, VCSFile>,
}

impl VersionControlSystem {
    /// Inicializacion del versionControlSystem --> posee el repositorio local y la ruta de la carpeta a informar.
    pub fn init(path: &str, args: Vec<String>) -> VersionControlSystem {
        let repository = Init::git_init(path, "repository_name", args);
        
        VersionControlSystem {
            path: path.to_string(),
            local_repository: repository,
            staging_area: HashMap::new(),
        }
    }


}