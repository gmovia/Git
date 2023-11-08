use std::{fs::{self, File}, path::{Path, PathBuf}, io::Write};
use crate::utils::hasher::hasher::Hasher;

pub enum WriteOption {
    Write,
    NoWrite,
}

pub struct HashObject;

impl HashObject{
    /// Recibe un hash y un contenido.
    /// Escribe el contenido dentro de un archivo cuya ruta depende del hash.
    pub fn write_object(hash: &str, object_path: PathBuf, data: &[u8]) -> Result<(), std::io::Error> {
        let folder_name = hash.chars().take(2).collect::<String>();
        let object = object_path.join(format!("{}",folder_name));

        fs::create_dir_all(&object)?;
        let file_path = object_path.join(format!("{}/{}",folder_name,&hash[2..]).as_str());

        if !file_path.exists() {
            let mut file = File::create(&file_path)?;
            file.write_all(data)?;
        }

        Ok(())
    }

    /// Recibe una ruta y una opcion. 
    /// Si la opcion es Write entonces escribe en el archivo objects, cuya ruta se calcula a partir del hash.
    /// Devuelve el hash del archivo.
    pub fn hash_object(path: &Path, object_path: PathBuf, option: WriteOption) -> Result<String, std::io::Error>{ // mejorar el char
        if path.is_dir(){
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "The path is an directory"));
        }
        
        if !fs::metadata(path).is_ok(){
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "No such file or directory"));
        }

        let file = fs::read(path)?;
        let hash = Hasher::hash( &file);

        match option{
            WriteOption::Write => HashObject::write_object(&hash, object_path, &file)?, // aca rompe
            WriteOption::NoWrite => ()
        }

        Ok(hash) 
    }   
}
