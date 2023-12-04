use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct CatFile;

impl CatFile {
    /// Recibe un hash y un contenido.
    /// Escribe el contenido dentro de un archivo cuya ruta depende del hash.
    pub fn cat_file(hash: &str, object_path: PathBuf) -> Result<String, std::io::Error> {
        let string_path = CatFile::get_hash_path(hash, object_path)?;
        let path = Path::new(&string_path);
        if !path.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "No such file or directory",
            ));
        }
        let data = fs::read(path)?;
        Ok(String::from_utf8_lossy(&data).to_string())
    }

    pub fn cat_file_bytes(hash: &str, object_path: PathBuf) -> Result<Vec<u8>, std::io::Error> {
        let string_path = CatFile::get_hash_path(hash, object_path)?;
        let path = Path::new(&string_path);
        if !path.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "No such file or directory",
            ));
        }
        let data = fs::read(path)?;
        Ok(data)
    }

    pub fn get_hash_path(hash: &str, object_path: PathBuf) -> Result<String, std::io::Error> {
        let folder_name = hash.chars().take(2).collect::<String>();

        let file_path = object_path.join(format!("{}/{}", folder_name, &hash[2..]).as_str());
        let path = Path::new(&file_path);
        Ok(path.display().to_string())
    }
}
