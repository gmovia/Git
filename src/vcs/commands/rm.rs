use std::{path::Path, fs, collections::HashMap};
use crate::{
    vcs::{files::vcs_file::VCSFile, version_control_system::VersionControlSystem},
    utils::files::files::read,
};
pub struct Rm;

impl Rm{
    
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
                    return Ok(staging_area.clone());
                }
                else{
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("fatal: pathspec '{}' did not match any files", key)));
                }
            }
        }
        let _ = vcs.index.read_staging_write_index(&staging_area);
        Ok(staging_area.clone())
    }


    pub fn rm_r(&mut self, vcs: &mut VersionControlSystem, path: &Path) -> Result<HashMap<String, VCSFile>, std::io::Error> {
        let mut result = HashMap::new();
        if let Ok(files) = read(path) {
            for (key, _) in &files {
                let file_path = Path::new(key);
                result = Rm::rm(vcs,file_path)?;
            }
        }
        Ok(result)
    }
}