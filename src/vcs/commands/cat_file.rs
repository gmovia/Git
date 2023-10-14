use std::{path::{Path, PathBuf}, fs};


pub struct CatFile;

impl CatFile{
    /// Recibe un hash y un contenido.
    /// Escribe el contenido dentro de un archivo cuya ruta depende del hash.
    pub fn cat_file(hash: &str, object_path: PathBuf) -> Result<String, std::io::Error> {
        let folder_name = hash.chars().take(2).collect::<String>();

        //let object_path = Path::new(".rust_git/objects/");

        let file_path = object_path.join(format!("{}/{}",folder_name,&hash[2..]).as_str());
        let path = Path::new(&file_path);

        if !path.exists(){
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "No such file or directory"));
        }
        
        let data = fs::read_to_string(&path)?;

        Ok(data)

    }
}