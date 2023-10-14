extern crate sha1;
use sha1::{Digest, Sha1};

use std::{fs::{self, File}, path::{Path, PathBuf}, io::Write};

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

        //let path = Path::new(".rust_git/objects/");
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
        let hex_hash = HashObject::hash(path)?;
        let file = fs::read(path)?;
        match option{
            WriteOption::Write => HashObject::write_object(&hex_hash, object_path, &file)?,
            WriteOption::NoWrite => ()
        }

        Ok(hex_hash) 
    }     

    pub fn hash(path: &Path) -> Result<String, std::io::Error>{
        if path.is_dir(){
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "The path is an directory"));
        }
        
        if !fs::metadata(path).is_ok(){
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "No such file or directory"));
        }

        let file = fs::read(path)?;
        let file_size = file.len().to_string();
        let hash_input = ["blob ".as_bytes(), file_size.as_bytes(), b"\0", &file].concat();
        
        let mut hasher = Sha1::new();
        hasher.update(hash_input);

        let result_in_bytes = hasher.finalize().to_vec();
        let hex_hash = result_in_bytes.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();

        Ok(hex_hash)
    }   
}
