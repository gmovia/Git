use std::{path::{Path, PathBuf}, fs::{self, OpenOptions}, io::{self, Write}};
use crate::vcs::files::current_commit::CurrentCommit;

use super::init::Init;

pub struct Branch;

pub enum BranchOptions<'a>{
    NewBranch(&'a str),
    DeleteBranch(&'a str),
    GetBranches,
    GetCurrentBranch,
}

impl Branch{

    /// Matcheo la opcion
    pub fn branch(path: &PathBuf, option: BranchOptions) -> Result<Vec<String>, std::io::Error>{
        match option{
            BranchOptions::NewBranch(branch_name) => {Ok(Self::create_new_branch(path, branch_name)?)},
            BranchOptions::DeleteBranch(branch_name) => {Ok(Self::delete_branch(path, branch_name)?)},
            BranchOptions::GetBranches => {Ok(Self::get_branches(path)?)},
            BranchOptions::GetCurrentBranch => {Ok(vec![Self::get_current_branch(path)?])}
        }
    }

    // Obtiene la rama actual
    pub fn get_current_branch(path: &Path) -> Result<String, std::io::Error> {
        Init::get_current_branch(path)
    }

    /// creo un archivo branch_name en el path /refs/heads/
    /// luego genero el archivo en /logs/ con copia de los commits que estaban en la rama anterior
    pub fn create_new_branch(path: &PathBuf, branch_name: &str) -> Result<Vec<String>,std::io::Error> { 
        let branch_head_path = path.join(".rust_git").join("refs").join("heads").join(branch_name);        
        let mut branch_head = OpenOptions::new().write(true).create(true).append(true).open(branch_head_path)?;

        let branch_log_path = path.join(".rust_git").join("logs").join(branch_name);
        let mut branch_log = OpenOptions::new().write(true).create(true).append(true).open(branch_log_path)?;
        
        let current_log = Init::get_current_log(path)?;
        let table = fs::read_to_string(current_log)?;
        
        let commit_hash = CurrentCommit::read()?;
        branch_head.write_all(commit_hash.as_bytes())?;
        branch_log.write_all(table.as_bytes())?;

        Self::get_branches(path)
    }

    pub fn create_new_branch_with_hash(path: &PathBuf, branch_name: &str, hash: &str) -> Result<Vec<String>,std::io::Error> { 
        let branch_head_path = path.join(".rust_git").join("refs").join("heads").join(branch_name);        
        let mut branch_head = OpenOptions::new().write(true).create(true).append(false).open(&branch_head_path)?;

        let branch_log_path = path.join(".rust_git").join("logs").join(branch_name);
        let mut branch_log = OpenOptions::new().write(true).create(true).append(true).open(&branch_log_path)?;
        
        let current_log = Init::get_current_log(&path)?;
        let table = fs::read_to_string(current_log)?;
        
        branch_head.write_all(hash.as_bytes())?;
        branch_log.write_all(table.as_bytes())?;

        Ok(Self::get_branches(path)?)
    }

    /// matcheo el archivo branch_name en /refs/heads/ y en /logs/
    /// si no estoy parada en esa rama, entonces lo elimino de los dos directorios
    pub fn delete_branch(path: &PathBuf, branch_name: &str) -> Result<Vec<String>,std::io::Error>{
        let p = Path::new(path);
        if let Ok(branches) = Self::get_branches(&p.to_path_buf()) {
            if !branches.contains(&branch_name.to_string()){
                return Err(io::Error::new(io::ErrorKind::NotFound, "Can't find the branch"));
            }
        }
        let branch_path = p.join(".rust_git").join("refs").join("heads").join(branch_name);
        let logs_path = p.join(".rust_git").join("logs").join(branch_name);
        if logs_path == Init::get_current_log(path)?{
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Can't remove the actual branch"));
        }
        fs::remove_file(branch_path)?;
        fs::remove_file(logs_path)?;
        Self::get_branches(path)
    }

    /// obtengo todas las entradas del directorio /refs/heads/ que serian todas las ramas que tenemos
    pub fn get_branches(path: &PathBuf) -> Result<Vec<String>,std::io::Error>{
        let mut branches: Vec<String> = Vec::new();
        let p = Path::new(path);
        let branchs_dir_path = p.join(".rust_git").join("refs").join("heads");
        if let Ok(entries) = fs::read_dir(branchs_dir_path){
            for entry in entries.flatten(){
                if let Some(file_name) = entry.path().file_name(){
                    branches.push(file_name.to_string_lossy().to_string());
                }

                }
        }
        Ok(branches)
    }        
}