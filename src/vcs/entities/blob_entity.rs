use std::{path::{Path, PathBuf}, fs::{OpenOptions, self}, io::Write};

use crate::vcs::commands::{hash_object::{HashObject, WriteOption}, init::Init, cat_file::CatFile};

pub struct BlobEntity{ // content
    pub content_type: String,
    pub path: String,
    pub blob_hash: String   
}

impl BlobEntity{
    /// Recibe el path del file y el path del repositorio
    /// Crea el archivo de blob, y devuelve su hash
    pub fn write(repo_path: PathBuf, content: &String) -> Result<String, std::io::Error>{
        let blob_path = Path::new(&repo_path).join("blob");
        let mut blob_file = OpenOptions::new().write(true).create(true).append(true).open(&blob_path)?; 

        blob_file.write_all(content.as_bytes())?;

        let blob_hash = HashObject::hash_object(Path::new(&blob_path), Init::get_object_path(&repo_path)?, WriteOption::Write)?;
        let _ = fs::remove_file(blob_path);
        Ok(blob_hash)
    }

    /// Recibe el hash del file/blob
    /// Devuelve el contenido del file
    pub fn read(repo_path: PathBuf, blob_hash: String) -> Result<String, std::io::Error>{
        let object_path = Init::get_object_path(&repo_path)?;
        let content = CatFile::cat_file(&blob_hash, object_path)?;
        Ok(content)
    }
}