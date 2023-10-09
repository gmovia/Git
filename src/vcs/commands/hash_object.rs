extern crate sha1;
use sha1::{Digest, Sha1};

use std::{fs::{self, File}, path::Path, io::Write};

pub struct HashObject;

impl HashObject{

    pub fn write_object(hash: &str, data: &[u8]) -> Result<(), std::io::Error> {
        let folder_name = hash.chars().take(2).collect::<String>();

        let path = Path::new(".rust_git/objects/");
        let object = path.join(format!("{}",folder_name));

        fs::create_dir_all(&object)?;
        let file_path = path.join(format!("{}/{}",folder_name,&hash[2..]).as_str());

        if !file_path.exists() {
            let mut file = File::create(&file_path)?;
            file.write_all(data)?;
        }

        Ok(())
    }

    // SI LE LLEGA -w entonces GUARDA y sino no, falta implementar esos 2 casos, ahora guarda todo 
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

        let _ = HashObject::write_object(&hex_hash, &file);

        Ok(hex_hash) 
    }        
}
