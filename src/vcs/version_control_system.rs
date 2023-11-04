use crate::{
    vcs::files::vcs_file::VCSFile,
    utils::files::files::read,
    types::types::{ChangesNotStagedForCommit, ChangesToBeCommited, UntrackedFiles},
    vcs::commands::{status::Status, add::Add, init::Init, hash_object::HashObject,cat_file::CatFile, clone::Clone}, client::client::Client, server::server::Server,
};
use super::{commands::{hash_object::WriteOption, rm::{Rm, RemoveOption}, commit::Commit, log::Log, branch::{Branch, BranchOptions}}, files::repository::Repository};
use std::{collections::HashMap, path::{Path, PathBuf}};
use super::files::index::Index;

pub struct VersionControlSystem {
    pub path: PathBuf,
    pub repository: Repository,
    pub index: Index
}

impl VersionControlSystem {
    /// Inicializacion del versionControlSystem --> posee el repositorio local y la ruta de la carpeta a informar.
    pub fn init(path: &Path, args: Vec<String>) -> VersionControlSystem {
        let _ = Init::git_init(&path.to_path_buf(), args);
        VersionControlSystem {
            path: path.to_path_buf(),
            repository: Repository::init(path.to_path_buf()),
            index: Index::init(&path.to_path_buf())
        }
    }

    /// Devuelve la informacion de los archivos creados, modificados y eliminados recientemente, junto con el area de staging.
    pub fn status(&self) -> Result<(UntrackedFiles, ChangesNotStagedForCommit, ChangesToBeCommited), std::io::Error> {
        let files = read(Path::new(&self.path.clone()))?;
        let staging_area = self.index.read_index()?;
        let repository = self.repository.read_repository()?;
        Ok(Status::status(&files, &staging_area, &repository))
    }

    /// Recibe un path
    /// Agrega los archivos que se encuentran dentro del path al area de staging
    /// Devuelve el area de staging
    pub fn add(&mut self, path: &Path) -> Result<HashMap<String, VCSFile>, std::io::Error> {
        Add::add(self, path)        
    }

    /// Calcula el hash object de un archivo. En el caso de que sea una carpeta, debe devolver un error.
    /// Si se añade el comando -w lo que sucede es que se guardan los datos en .git/objects (investigar bien) FALTA HACER
    pub fn hash_object(&self, path: &Path, option: WriteOption) -> Result<String, std::io::Error>{
        let object_path = Init::get_object_path(&self.path, ".rust_git")?;
        HashObject::hash_object(path, object_path, option)
    }
 
    /// Recibe un hash
    /// Obtiene el path del hash y devuelve el contenido que hay en el archivo del path
    pub fn cat_file(&self, hash: &str, path: &str) -> Result<String, std::io::Error>{
        let object_path = Init::get_object_path(&self.path, path)?;
        CatFile::cat_file(hash, object_path)
    }

    /// Recibe un hash
    /// Obtiene el path del hash y devuelve el contenido que hay en el archivo del path en bytes
    pub fn cat_file_bytes(&self, hash: &str, path: &str) -> Result<Vec<u8>, std::io::Error>{
        let object_path = Init::get_object_path(&self.path, path)?;
        CatFile::cat_file_bytes(hash, object_path)
    }

    /// Recibe un path
    /// Elimina los archivos del workspace y repositorio local dado el path
    /// Si el comando tiene un -r se eliminan los archivos de un directorio entero
    pub fn rm(&mut self, path: &Path, option: RemoveOption) -> Result<HashMap<String, VCSFile>, std::io::Error> {
        Rm::rm(self, path, option)
    }

    /// Recibe un mensaje
    /// Crea una entrada en la tabla de commits con su correspondiente id, hash del repositorio y mensaje.
    pub fn commit(&mut self, message: String) -> Result<HashMap<String, String>, std::io::Error>{
        Commit::commit(self, message)
    }

    ///Muestra el historial de commits
    pub fn log(&self) -> Result<(), std::io::Error> {
        Log::log(self)
    }


    pub fn clone(&self, server_repo: String) -> Result<(), std::io::Error> {
        let _ = Client::client_(self,"git clone".to_string(),server_repo);
        Ok(())
    }

    pub fn fetch(&self, server_repo: String) -> Result<(), std::io::Error> {
        let _ = Client::client_(self, "git fetch".to_string(),server_repo);
        Ok(())
    }
    
    /// Recibe una opcion de branch (crear, borrar, listar)
    /// Segun la opcion, el branch permite crear una rama, borrar una ya existente o listar todas las ramas
    pub fn branch(&self,option: BranchOptions) -> Result<(), std::io::Error>{
        Branch::branch(&self.path, option)
    }
}