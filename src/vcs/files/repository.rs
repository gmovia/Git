use std::{collections::HashMap, path::{Path, PathBuf}, fs::{OpenOptions, self}, io::{self, BufRead, Write}};
use crate::{vcs::{commands::{hash_object::{WriteOption, HashObject},  init::Init, cat_file::CatFile}, entities::tree::Tree}, constants::constants::END_OF_LINE};
use super::{commits_table::CommitsTable, current_repository::CurrentRepository};

#[derive(Debug, Clone)]
pub struct Repository;

impl Repository{

    /// PARA EL CLIENTE
    /// Obtengo el estado actual del repositorio local asociado al repositorio que actualmente tiene activo el cliente 
    /// Va a la tabla de commits de la branch actual y obtiene los blobs relacionados al ultimo commit
     pub fn read_repository() -> Result<HashMap<String,String>,std::io::Error>{
        let current_path = CurrentRepository::read()?;
        let mut local_repository:HashMap<String, String>  = HashMap::new();
        
        let commits_file = OpenOptions::new().read(true).open(Init::get_commits_path(&current_path)?)?;

        let reader = io::BufReader::new(commits_file);
        
        if let Some(last_commit) = reader.lines().filter_map(Result::ok).last(){
            let parts: Vec<&str> = last_commit.split("-").collect(); // parts[0] = id ; parts[1] = hash ; parts[2] = message ; parts[3] = date ; parts[4] = tree
            local_repository.extend(Repository::read_repository_of_commit(current_path.clone(), &Init::get_current_branch(&current_path)?, parts[1])?);
        }
        
        Ok(local_repository)
    }

    /// PARA TODOS
    /// Recibe el path del repositorio (cliente o servidor), una branch y el hash del commit 
    /// Obtiene su repositorio asociado. Es la funcion generalizada del caso anterior (en el, tengo constante todos estos datos)
    pub fn read_repository_of_commit(repo_path: PathBuf, branch: &str, commit_hash: &str) -> Result<HashMap<String, String>,std::io::Error>{
        let mut repository: HashMap<String, String> = HashMap::new();
        let commits_table = CommitsTable::read(repo_path.clone(), branch)?;

        for commit in commits_table {
            if commit.hash == commit_hash {
                let tree = Tree::read(commit_hash, &repo_path)?;
                
                let blobs = CatFile::cat_file(&tree.repository_hash, Init::get_object_path(&repo_path)?)?;
                let blobs_lines: Vec<&str> = blobs.split("\n").collect();

                for blob in blobs_lines{
                    if blob != END_OF_LINE{
                        let blobs_parts: Vec<&str> = blob.split_whitespace().collect(); // line_partes[0] = blob line_parts[1] = path ; line_parts[2] = content
                        repository.insert(blobs_parts[1].to_string(), blobs_parts[2].to_string());
                    }
                }
            }
        }
        Ok(repository)
    }

    /// PARA EL CLIENTE (como todos los comandos que usen CurrentRepository)
    /// Creo el archivo que contiene todos los blobs relacionados al repositorio y devuelvo su hash
    /// Es el hash que se encontrara dentro del tree! Ejemplo: "tree a123basd.."
    pub fn write_repository(repository: &HashMap<String,String>) -> Result<String, std::io::Error>{
        let current_path = CurrentRepository::read()?;

        let path = Path::new(&current_path).join("temp");
        let mut commit_file = OpenOptions::new().write(true).create(true).append(true).open(&path)?; 
        
        for (key, value) in repository {
            let entry = format!("blob {} {}\n", key, value);
            commit_file.write_all(entry.as_bytes())?;
        }
        
        let hash = HashObject::hash_object(&path, Init::get_object_path(&current_path)?, WriteOption::Write)?;
        let _ = fs::remove_file(path);
        Ok(hash)
    }
}
