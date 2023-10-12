
use crate::vcs::{
    files::vcs_file::VCSFile,
    repository::Repository, commands::{init::Init, hash_object::HashObject}

};
use std::{collections::HashMap, path::Path, fs::{self, File}, io::Read};
use super::commands::hash_object::WriteOption;
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
    pub fn hash_object(path: &Path, option: WriteOption) -> Result<String, std::io::Error>{
        Ok(HashObject::hash_object(path, option)?)
    }

    pub fn cat_file(hash: &str) -> Result<String, std::io::Error>{
        let folder_name = hash.chars().take(2).collect::<String>();

        let object_path = Path::new(".rust_git/objects/");

        let file_path = object_path.join(format!("{}/{}",folder_name,&hash[2..]).as_str());
        let path = Path::new(&file_path);

        if !path.exists(){
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "No such file or directory"));
        }
        
        let data = fs::read_to_string(&path)?;

        Ok(data)
    }
}