use std::{collections::HashMap, path::{PathBuf, Path}, fs::{OpenOptions, self}, io::Write};
use crate::vcs::{files::repository::Repository, commands::{hash_object::{HashObject, WriteOption}, init::Init, cat_file::CatFile}};

use super::{tree_entity::TreeEntity, blob_entity::BlobEntity};

pub struct CommitEntity{    // tree + el hash del tree
    pub content_type: String, // "tree"
    pub tree_hash: String
}

impl CommitEntity{
    /// Recibe el conjunto de blobs que se va a escribir y el path del repositorio
    /// Crea el archivo de commit y devuelve su hash => el que se encuentra en la tabla de commits => id hash_commit message date
    pub fn write(repo_path: &PathBuf, tree_hash: &String) -> Result<String, std::io::Error>{
        let commit_path = Path::new(&repo_path).join("commit");
        let mut commit_file = OpenOptions::new().write(true).create(true).append(true).open(&commit_path)?; 

        let entry = format!("tree {}\n", tree_hash);
        commit_file.write_all(entry.as_bytes())?;
        
        let commit_hash = HashObject::hash_object(&commit_path, Init::get_object_path(&repo_path)?, WriteOption::Write)?;
        
        let _ = fs::remove_file(commit_path);
        Ok(commit_hash)
    }

    /// Recibe el hash del commit
    /// Devuelve el contenido de ese commit => tree hash_tree
    pub fn read(repo_path: &PathBuf, commit_hash: &str) -> Result<CommitEntity, std::io::Error>{
        let commit = CatFile::cat_file(commit_hash, Init::get_object_path(repo_path)?)?;
        let commit_lines: Vec<&str> = commit.split_whitespace().collect();
        Ok(CommitEntity{content_type: commit_lines[0].to_string(), tree_hash: commit_lines[1].to_string()})
    }
}