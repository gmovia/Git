use std::{path::Path, fs, collections::HashMap};
use crate::{vcs::files::{vcs_file::VCSFile, repository::Repository, index::Index}, utils::files::files::read, constants::constants::{STATE_DELETED, NULL}};
pub struct Rm;

#[derive(Debug,Clone)]
pub enum RemoveOption {
    Directory,
    NoDirectory,
}

impl Rm{
    /// Recibe un path 
    /// Se elimina al path correspondiente segun sea archivo o directorio
    pub fn remove_from_workspace(path: &str) -> std::io::Result<()> {
        let path = Path::new(path);
        let metadata = fs::metadata(path)?;
    
        if metadata.is_file() {
            return fs::remove_file(path);
        } 
        if metadata.is_dir() {
            return fs::remove_dir_all(path);
        }
        Ok(())
    }

    pub fn rm(path: &Path, option: RemoveOption) -> Result<HashMap<String, VCSFile>, std::io::Error> {
        match option {
            RemoveOption::NoDirectory => Rm::rm_(path),
            RemoveOption::Directory => Rm:: rm_r(path)
        }
    }
    
    /// Recibe el sistema control de versiones y un path
    /// Setea estado eliminado a los archivos correspondiente en el area de staging
    /// Devuelve el area de staging
    pub fn rm_(path: &Path) -> Result<HashMap<String, VCSFile>, std::io::Error> {   

        if path.is_dir(){
            return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("fatal: not removing '{:?}' recursively without -r", path)));
        }

        let mut staging_area = Index::read_index()?;
        
        if let Ok(files) = read(path){
            if files.is_empty(){
                return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("fatal: pathspec '{:?}' did not match any files", path)));
            }
            for key in files.keys(){
                if Repository::read_repository()?.contains_key(key){
                    Rm::remove_from_workspace(key)?;
                    let file = VCSFile::new(key.clone(), NULL.to_string(), STATE_DELETED.to_string());
                    staging_area.insert(key.to_owned(), file);
                    let _ = Index::write_index(&staging_area);
                    return Ok(staging_area.clone());
                }
                else{
                    println!("fatal: pathspec '{}' did not match any files", key);                
                }
            }
        }
        Ok(staging_area.clone())
    }

    /// Recibe el sistema control de versiones y un path correspondiente a un directorio
    /// Recorre el path del directorio y a rm manda el archivo leido para ser seteado con el estado correspondiente
    /// Devuelve el area de staging
    pub fn rm_r(dir_path: &Path) -> Result<HashMap<String, VCSFile>, std::io::Error> {
        let mut result = HashMap::new();
        if let Ok(files) = read(dir_path) {
            if files.is_empty() {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("fatal: pathspec '{:?}' did not match any files", dir_path)));
            }
            for key in files.keys() {
                let file_path = Path::new(key);
                result = Rm::rm_(file_path)?;
            }
        }
        Ok(result)
    }
}




