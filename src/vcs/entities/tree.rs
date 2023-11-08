use std::{collections::HashMap, path::{PathBuf, Path}, fs::{OpenOptions, self}, io::Write};
use crate::vcs::{files::repository::Repository, commands::{hash_object::{HashObject, WriteOption}, init::Init, cat_file::CatFile}};

pub struct Tree{
    pub code: String,
    pub repository_hash: String
}

impl Tree{
    /// Recibe el repositorio local y el path del repositorio => Crea el archivo tree y devuelve su hash
    pub fn create(repository: &HashMap<String, String>, path: &PathBuf) -> Result<String, std::io::Error>{
        let tree_path = Path::new(&path).join("tree");
        let mut tree_file = OpenOptions::new().write(true).create(true).append(true).open(&tree_path)?; 

        let repository_hash = Repository::write_repository(&repository)?;
        
        let entry = format!("tree {}\n", repository_hash);
        tree_file.write_all(entry.as_bytes())?;
        
        let hash = HashObject::hash_object(&tree_path, Init::get_object_path(&path)?, WriteOption::Write)?;
        
        let _ = fs::remove_file(tree_path);
        Ok(hash) // NO CONFUNDIR! hash es del TREE (es el hash de "tree"+el hash del repositorio) repository_hash es el HASH donde estan los BLOBS
    }

    /// Recibe el hash del tree y el path del repositorio => Devuelve el tree con su codigo y hash
    /// Ejemplo. 
    ///     Si en la tabla de commit hay un commit con: 1  a123b   primer commit    2023-02-17, y a123b apunta a => tree b9231
    ///     Devuelve (tree, b9231)
    pub fn read(hash: &str, repo_path: &PathBuf) -> Result<Tree, std::io::Error>{
        let tree = CatFile::cat_file(hash, Init::get_object_path(repo_path)?)?;
        let tree_lines: Vec<&str> = tree.split_whitespace().collect();
        Ok(Tree{code: tree_lines[0].to_string(), repository_hash: tree_lines[1].to_string()})
    }
}