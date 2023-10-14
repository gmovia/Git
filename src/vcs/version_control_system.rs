use crate::{
    vcs::files::vcs_file::VCSFile,
    utils::files::files::read,
    types::types::{ChangesNotStagedForCommit, ChangesToBeCommited, UntrackedFiles},
    vcs::commands::{status::Status, add::Add, init::Init, hash_object::HashObject,cat_file::CatFile},
};
use super::commands::{hash_object::WriteOption, rm::Rm};
use std::{collections::HashMap, path::{Path, self}};
use super::files::index::Index;

pub struct VersionControlSystem {
    pub path: String,
    pub local_repository: HashMap<String, String>,
    pub index: Index
}

impl VersionControlSystem {
    /// Inicializacion del versionControlSystem --> posee el repositorio local y la ruta de la carpeta a informar.
    pub fn init(path: &str, args: Vec<String>) -> VersionControlSystem {
        let _ = Init::git_init(path, args);
        VersionControlSystem {
            path: path.to_string(),
            local_repository: HashMap::new(),
            index: Index::init(path)
        }
    }

    /// Devuelve la informacion de los archivos creados, modificados y eliminados recientemente, junto con el area de staging.
    pub fn status(&self) -> Result<(UntrackedFiles, ChangesNotStagedForCommit, ChangesToBeCommited), std::io::Error> {
        let files = read(Path::new(&self.path.clone()))?;
        let staging_area = self.index.read_index_write_staging()?;
        Ok(Status::status(&files, &staging_area, &self.local_repository))
    }

    /// Recibe un path
    /// Agrega los archivos que se encuentran dentro del path al area de staging
    /// Devuelve el area de staging
    pub fn add(&mut self, path: &Path) -> Result<HashMap<String, VCSFile>, std::io::Error> {
        Add::add(self, path)        
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


    // pub fn rm(&mut self, path: &Path) -> Result<String, std::io::Error> {
    //     Rm::rm(self, path)?
    // }
    
}