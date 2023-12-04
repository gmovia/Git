use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

use crate::{
    constants::constant::BLOB_CODE,
    utils::randoms::random::Random,
    vcs::commands::{
        cat_file::CatFile,
        hash_object::{HashObject, WriteOption},
        init::Init,
    },
};

#[derive(Debug, Clone)]

pub struct BlobEntity {
    pub content_type: String,
    pub path: String,
    pub blob_hash: String,
}

impl BlobEntity {
    /// Recibe el path del file y el path del repositorio
    /// Crea el archivo de blob, y devuelve su hash
    pub fn write(repo_path: &Path, content: &String) -> Result<String, std::io::Error> {
        let blob_path = Path::new(&repo_path).join(Random::random());
        let mut blob_file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&blob_path)?;

        blob_file.write_all(content.as_bytes())?;

        let blob_hash = HashObject::hash_object(
            Path::new(&blob_path),
            Init::get_object_path(repo_path)?,
            WriteOption::Write,
            BLOB_CODE,
        )?;
        let _ = fs::remove_file(blob_path);
        Ok(blob_hash)
    }

    /// Recibe el hash del file/blob
    /// Devuelve el contenido del file
    pub fn read(repo_path: &Path, blob_hash: String) -> Result<String, std::io::Error> {
        let object_path = Init::get_object_path(repo_path)?;
        let content = CatFile::cat_file(&blob_hash, object_path)?;
        Ok(content)
    }
}
