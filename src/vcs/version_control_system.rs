
use crate::vcs::{
    files::vcs_file::VCSFile,
    repository::Repository, commands::{init::Init, hash_object::HashObject,cat_file::CatFile}

};
use std::{collections::HashMap, path::Path};
use super::commands::hash_object::WriteOption;
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

    /// Calcula el hash object de un archivo. En el caso de que sea una carpeta, debe devolver un error.
    /// Si se añade el comando -w lo que sucede es que se guardan los datos en .git/objects (investigar bien) FALTA HACER
    pub fn hash_object(path: &Path, option: WriteOption) -> Result<String, std::io::Error>{
        Ok(HashObject::hash_object(path, option)?)
    }

    /// Recibe un hash
    /// Obtiene el path del hash y devuelve el contenido que hay en el archivo del path
    pub fn cat_file(hash: &str) -> Result<String, std::io::Error>{
        Ok(CatFile::cat_file(hash)?)
    }
}