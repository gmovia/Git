use std::{path::{Path, PathBuf}, fs::{File, self}, io};

use super::init::Init;


pub struct Branch;

pub enum BranchOptions<'a>{
    NewBranch(&'a str),
    DeleteBranch(&'a str),
    GetBranches,
}

impl Branch{

    /// Matcheo la opcion
    pub fn branch(path: &PathBuf, option: BranchOptions) -> Result<(), std::io::Error>{
        match option{
            BranchOptions::NewBranch(branch_name) => {Self::create_new_branch(path.to_path_buf(), branch_name)?;},
            BranchOptions::DeleteBranch(branch_name) => {Self::delete_branch(&path, branch_name)?;},
            BranchOptions::GetBranches => {Self::get_branches(&path)?;},
        }
        Ok(())
    }


    /// creo un archivo branch_name en el path /refs/heads/
    /// luego genero el archivo en /logs/ con copia de los commits que estaban en la rama anterior
    pub fn create_new_branch(path: PathBuf,branch_name: &str) -> Result<(),std::io::Error> { 
        let branch_path = path.join(".git").join("refs").join("heads").join(branch_name);
        let _ = File::create(&branch_path)?;
        Init::create_log_file(path, branch_name)?;
        Ok(())
    }

    /// matcheo el archivo branch_name en /refs/heads/ y en /logs/
    /// si no estoy parada en esa rama, entonces lo elimino de los dos directorios
    pub fn delete_branch(path: &PathBuf, branch_name: &str) -> Result<(),std::io::Error>{
        let p = Path::new(path);
        let branch_path = p.join(".git").join("refs").join("heads").join(branch_name);
        let logs_path = p.join(".git").join("logs").join(branch_name);
        if logs_path == Init::get_commits_path(&path)?{
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Can't remove the actual branch"));
        }
        fs::remove_file(branch_path)?;
        fs::remove_file(logs_path)?;
        Ok(())
    }

    /// obtengo todas las entradas del directorio /refs/heads/ que serian todas las ramas que tenemos
    pub fn get_branches(path: &PathBuf) -> Result<Vec<String>,std::io::Error>{
        let mut branches: Vec<String> = Vec::new();
        let p = Path::new(path);
        let branchs_dir_path = p.join(".git").join("refs").join("heads");
        if let Ok(entries) = fs::read_dir(branchs_dir_path){
            for entry in entries{
                if let Ok(entry) = entry{
                    if let Some(file_name) = entry.path().file_name(){
                        branches.push(file_name.to_string_lossy().to_string());
                        println!("{:?}",file_name.to_string_lossy().to_string());
                    }

                }
            }
        }
        Ok(branches)
    }
}