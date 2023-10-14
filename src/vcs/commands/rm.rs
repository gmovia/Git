use std::{path::Path, fs, collections::HashMap};
use crate::{
    vcs::{files::vcs_file::VCSFile, version_control_system::VersionControlSystem},
    utils::files::files::read,
};
pub struct Rm;

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

    pub fn git_rm(vcs: &mut VersionControlSystem, path: &Path, args: Vec<String>) -> Result<HashMap<String, VCSFile>, std::io::Error> {
        if args.iter().any(|arg| arg.contains("-r")) {
            Rm::rm_r(vcs, path)
        } else {
            Rm::rm(vcs, path)
        }
    }
    
    
    /// Recibe el sistema control de versiones y un path
    /// Setea estado eliminado a los archivos correspondiente en el area de staging
    /// Devuelve el area de staging
    pub fn rm(vcs: &mut VersionControlSystem, path: &Path) -> Result<HashMap<String, VCSFile>, std::io::Error> {   

        if path.is_dir(){
            return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("fatal: not removing '{:?}' recursively without -r", path)));
        }

        let mut staging_area = vcs.index.read_index_write_staging()?;
        
        if let Ok(files) = read(path){
            for(key,value) in &files{
                if vcs.local_repository.contains_key(key){
                    Rm::remove_from_workspace(&key)?;
                    let file = VCSFile::new(key.clone(), value.clone(), "DELETED".to_string());
                    staging_area.insert(key.to_owned(), file);
                    let _ = vcs.index.read_staging_write_index(&staging_area);
                    return Ok(staging_area.clone());
                }
                else{
                    println!("fatal: pathspec '{}' did not match any files", key);                }
            }
        }
        Ok(staging_area.clone())
    }

    /// Recibe el sistema control de versiones y un path correspondiente a un directorio
    /// Recorre el path del directorio y a rm manda el archivo leido para ser seteado con el estado correspondiente
    /// Devuelve el area de staging
    pub fn rm_r(vcs: &mut VersionControlSystem, dir_path: &Path) -> Result<HashMap<String, VCSFile>, std::io::Error> {
        let mut result = HashMap::new();
        if let Ok(files) = read(dir_path) {
            for (key, _) in &files {
                let file_path = Path::new(key);
                result = Rm::rm(vcs,file_path)?;
            }
        }
        Ok(result)
    }
}