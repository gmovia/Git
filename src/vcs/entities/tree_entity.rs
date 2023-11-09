use std::{collections::HashMap, path::{PathBuf, Path}, fs::{OpenOptions, self}, io::Write};
use crate::{vcs::commands::{hash_object::{HashObject, WriteOption}, init::Init, cat_file::CatFile}, constants::constants::END_OF_LINE};

use super::blob_entity::BlobEntity;

pub struct TreeEntity; // conjunto de blobs y cada uno con => blob + path + hash_blob

impl TreeEntity{
    /// Recibe el conjunto de blobs que se van a escribir y el path del repositorio
    /// Crea el archivo de tree y devuelve su hash => es el que se encuentra en la entrada del commit => tree hash_tree
    pub fn write(repo_path: &PathBuf, blobs: &Vec<BlobEntity>) -> Result<String, std::io::Error>{
        let tree_path = Path::new(&repo_path).join("tree");
        let mut tree_file = OpenOptions::new().write(true).create(true).append(true).open(&tree_path)?; 

        for blob in blobs {
            let entry = format!("{} {} {}\n", blob.content_type, blob.path, blob.blob_hash);
            tree_file.write_all(entry.as_bytes())?;
        }

        let tree_hash = HashObject::hash_object(&tree_path, Init::get_object_path(&repo_path)?, WriteOption::Write)?;
        let _ = fs::remove_file(tree_path);
        Ok(tree_hash)
    }

    /// Recibe el hash del tree y el path del repositorio
    /// Devuelve la lista de blobs que contiene el tree => blob file_path hash_blob
    pub fn read(repo_path: &PathBuf, tree_hash: String) -> Result<Vec<BlobEntity>, std::io::Error>{
        let mut blobs: Vec<BlobEntity> = Vec::new();
        let content = CatFile::cat_file(&tree_hash, Init::get_object_path(&repo_path)?)?;
        let blobs_lines: Vec<&str> = content.split("\n").collect();
        for blob_line in blobs_lines{
            if blob_line != END_OF_LINE{
                let blobs_parts: Vec<&str> = blob_line.split_whitespace().collect();
                let blob = BlobEntity{content_type: blobs_parts[0].to_string(), path: blobs_parts[1].to_string(), blob_hash: blobs_parts[2].to_string()};
                blobs.push(blob);
            }
        }
        Ok(blobs)
    }
}

