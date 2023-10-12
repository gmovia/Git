use crate::{
    vcs::{files::vcs_file::VCSFile, version_control_system::VersionControlSystem},
    utils::files::files::read,
};

use std::{collections::HashMap, path::{Path, self}, fs::{self, OpenOptions, File}, io::{Write, Read, BufReader, BufRead}, env};
pub struct Add;

impl Add{
    /// Recibe el sistema de control de versiones y un path
    /// Inserta los archivos correspondientes al area de staging, junto con sus respectivos estados.
    /// Devuelve el area de staging.
    pub fn add(vcs: &mut VersionControlSystem, path: &Path) -> Result<HashMap<String, VCSFile>, std::io::Error> {
        let (untracked_files, changes_not_staged_for_commit, _) = vcs.status()?;

        if let Ok(files) = read(path) {
            for (key, value) in &files {
                let state = match (untracked_files.get(key), changes_not_staged_for_commit.get(key)) {
                    (Some(state), _) => state.to_string(),
                    (_, Some(_)) if !vcs.local_repository.contains_key(key) => "CREATED".to_string(),
                    (_, Some(state)) => state.to_string(),
                    _ => continue,
                };
                let file = VCSFile::new(key.clone(), value.clone(), state.clone());
                Add::add_index(&path, &state)?;
                vcs.staging_area.insert(key.clone(), file);
            }
        }
    
        if vcs.local_repository.contains_key(&path.display().to_string()) && !read(path)?.contains_key(&path.display().to_string()){
            let file = VCSFile::new(path.display().to_string(), "".to_string(), "DELETED".to_string());
            vcs.staging_area.insert(path.display().to_string(), file);
            Add::add_index(&path, &"DELETED".to_string())?;
        }
        Ok(vcs.staging_area.clone())
    }

    /// Recibe el path del archivo y su estado
    /// Escribe en el archivo index el path, el estado y el contenido 
    pub fn add_index(path: &Path, state: &String) -> Result<(),std::io::Error> {
        let index_path = Path::new(".rust_git/index");
        fs::create_dir_all(".rust_git")?;

        let mut index_file = OpenOptions::new().write(true).create(true).append(true).open(&index_path)?;
        
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        if let Ok(exist) = Add::path_exist_in_index(path, index_path) {
            if exist {
                Add::overwrite_aggregate(path, index_path)?;
            }
            else {
                index_file.write_all(path.to_str().unwrap().as_bytes())?;
                index_file.write_all("-".as_bytes())?;
                index_file.write_all(state.as_bytes())?;
                index_file.write_all("-".as_bytes())?;
                index_file.write_all(content.as_bytes())?;                
                index_file.write_all("\n".as_bytes())?;
            }
        }

        Ok(())
    }

    /// Recibe el path del archivo y el path del archivo .rust_git/index
    /// Verifica si el path del archivo se encuentra escrito dentro del index 
    /// Devuelve un booleano en caso de que este
    pub fn path_exist_in_index(path: &Path, index_path: &Path) -> Result<bool,std::io::Error> {
        let file = File::open(index_path)?;
        let reader = BufReader::new(&file);
        
        let mut existe = false;
        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split('-').collect();
            
            if parts.len() == 3 && parts[0] == path.to_str().unwrap() {
                existe = true;
            }
        }
        Ok(existe)
    }

    /// Recibe el path del archivo y el path del archivo .rust_git/index
    /// Sobre escribe el archivo index para un path especifico 
    pub fn overwrite_aggregate(file_path: &Path, index_path: &Path) -> Result<(),std::io::Error> {
        let file = File::open(index_path)?;
        let reader = BufReader::new(&file);

        let mut lines = Vec::new();

        let mut file = File::open(file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        for line in reader.lines() {
            let line = line?;
            let mut parts: Vec<&str> = line.split('-').collect();

            if parts.len() == 3 && parts[0] == file_path.to_str().unwrap() {
                parts[2] = &content;
            }

            let modified_line = parts.join("-");
            lines.push(modified_line);
        }

        let mut file = File::create(index_path)?;

        for line in lines {
            writeln!(file, "{}", line)?;
        }
        Ok(())  
    }

}