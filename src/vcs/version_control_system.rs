use crate::vcs::{
    files::vcs_file::VCSFile,
    repository::Repository, commands::{init::Init, hash_object::HashObject}
};
use std::{collections::HashMap, path::Path};

pub struct VersionControlSystem {
    pub path: String,
    pub local_repository: Repository,
    pub staging_area: HashMap<String, VCSFile>,
}

impl VersionControlSystem {
    /// Inicializacion del versionControlSystem --> posee el repositorio local y la ruta de la carpeta a informar.
    pub fn init(path: String, args: Vec<String>) -> VersionControlSystem {
        let repository = Init::git_init("repository_name", args);
        
        VersionControlSystem {
            path: path.to_string(),
            local_repository: repository,
            staging_area: HashMap::new(),
        }
    }

    /// Calcula el hash object de un archivo. En el caso de que sea una carpeta, debe devolver un error.
    /// Si se añade el comando -w lo que sucede es que se guardan los datos en .git/objects (investigar bien) FALTA HACER
    pub fn hash_object(path: &Path) -> Result<String, std::io::Error>{
        Ok(HashObject::hash_object(path)?)
    }
}