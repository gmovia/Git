extern crate sha1;
use sha1::{Digest, Sha1};

use std::{fs, path::Path, io::Write};

pub struct HashObject;

impl HashObject{

    pub fn write_object(hash: &str, data: &[u8]) -> Result<(), std::io::Error> { // POR AHORA ALMACENO SOLO EL CONTENIDO, HAY QUE ALMACENAR ALGO MAS?
        let object_dir = ".git/objects"; // NO ME CREA NADA!
        let object_path = format!("{}/{}", object_dir, &hash[..2]);
        let object_filename = &hash[2..];

        // Crear el directorio de objetos si no existe
        fs::create_dir_all(&object_path)?;

        // Escribe el objeto en el archivo
        let mut object_file = fs::File::create(format!("{}/{}", object_path, object_filename))?;
        object_file.write_all(data)?;

        Ok(())
    }

    // CASO BASICO SIN FLAGS, ya esta cubierto
    // SI LE LLEGA -w entonces GUARDA el archivo (no se como ni que) 
    pub fn hash_object(path: &Path) -> Result<String, std::io::Error>{
        if path.is_dir(){
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "The path is an directory"));
        }

        let file = fs::read(path)?;
        let file_size = file.len().to_string();
        let hash_input = ["blob ".as_bytes(), file_size.as_bytes(), b"\0", &file].concat();
        
        let mut hasher = Sha1::new();
        hasher.update(hash_input);

        let result_in_bytes = hasher.finalize().to_vec();
        let hex_hash = result_in_bytes.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();

        // falta escribir en el archivo
        //HashObject::write_object(&hex_hash, &file)?; //ME TIRA ERROR DE PERMISOS DE LECTURA, si desactivas no pasan los test => mirar abajo
        // ANDA PERO TENES QUE EJECUTAR sudo cargo test
        // IGUAL NO ME CREA NADA!!
        Ok(hex_hash) // convert bytes in string
    }        
}
