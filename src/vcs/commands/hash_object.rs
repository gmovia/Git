use crate::{
    constants::constant::{COMMIT_CODE, TAG_CODE, TREE_CODE},
    utils::hashers::hasher::Hasher,
};
use std::num::ParseIntError;
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

pub enum WriteOption {
    Write,
    NoWrite,
}

pub struct HashObject;

impl HashObject {
    /// Recibe un hash y un contenido.
    /// Escribe el contenido dentro de un archivo cuya ruta depende del hash.
    pub fn write_object(
        hash: &str,
        object_path: PathBuf,
        data: &[u8],
    ) -> Result<(), std::io::Error> {
        let folder_name = hash.chars().take(2).collect::<String>();
        let object = object_path.join(&folder_name);

        fs::create_dir_all(object)?;
        let file_path = object_path.join(format!("{}/{}", folder_name, &hash[2..]).as_str());

        if !file_path.exists() {
            let mut file = File::create(&file_path)?;
            file.write_all(data)?;
        }

        Ok(())
    }

    /// Recibe una ruta y una opcion.
    /// Si la opcion es Write entonces escribe en el archivo objects, cuya ruta se calcula a partir del hash.
    /// Devuelve el hash del archivo.
    pub fn hash_object(
        path: &Path,
        object_path: PathBuf,
        option: WriteOption,
        entity_type: &str,
    ) -> Result<String, std::io::Error> {
        if path.is_dir() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "The path is an directory",
            ));
        }

        if fs::metadata(path).is_err() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "No such file or directory",
            ));
        }

        let content = fs::read_to_string(path)?;
        let hash = Self::hash(&content, entity_type)?;

        match option {
            WriteOption::Write => HashObject::write_object(&hash, object_path, content.as_bytes())?,
            WriteOption::NoWrite => (),
        }

        Ok(hash)
    }

    pub fn parse_input(entries: Vec<&str>) -> Result<Vec<u8>, std::io::Error> {
        let mut tree_entries: Vec<u8> = Vec::new();

        for entry in entries {
            let entry_split: Vec<&str> = entry.split_whitespace().collect();
            if !entry_split.is_empty() {
                if let Ok(hash_bytes) = Self::decode_hex(entry_split[2]) {
                    tree_entries.extend_from_slice(
                        format!("{} {}\0", entry_split[0], entry_split[3]).as_bytes(),
                    );
                    tree_entries.extend_from_slice(&hash_bytes);
                }
            }
        }

        Ok(tree_entries)
    }

    pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
            .collect()
    }

    pub fn hash(content: &String, entity_type: &str) -> Result<String, std::io::Error> {
        let input: Vec<u8> = match entity_type {
            TREE_CODE => {
                let array: Vec<&str> = content.split('\n').collect();
                let tree_entries: Vec<u8> = Self::parse_input(array)?;

                let lenght = tree_entries.len();
                let mut git_content = format!("tree {lenght}\0").into_bytes();
                git_content.extend(tree_entries);

                git_content
            }
            COMMIT_CODE => {
                let input = [
                    "commit ".as_bytes(),
                    content.len().to_string().as_bytes(),
                    b"\0",
                    content.as_bytes(),
                ]
                .concat();
                input
            }
            TAG_CODE => {
                let input = [
                    "tag ".as_bytes(),
                    content.len().to_string().as_bytes(),
                    b"\0",
                    content.as_bytes(),
                ]
                .concat();
                input
            }
            _ => {
                let input = [
                    "blob ".as_bytes(),
                    content.len().to_string().as_bytes(),
                    b"\0",
                    content.as_bytes(),
                ]
                .concat();
                input
            }
        };

        let hash = Hasher::hash(&input);
        Ok(hash)
    }
}
