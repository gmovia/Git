use std::{collections::HashMap, fs, path::Path, io::Write};
use crate::utils::hasher::hasher::Hasher;

/// Recibe un string que representa una ruta.
/// Devuelve los archivos y carpetas que esta contiene en formato HashMap. La clave representa la ruta al archivo y el valor su contenido.

pub fn read(path: &Path) -> Result<HashMap<String, String>, std::io::Error> {
    let mut files: HashMap<String, String> = HashMap::new();
    let _ = read_files(path, &mut files);
    Ok(files)
}

fn is_excluded_directory(entry: &std::fs::DirEntry) -> bool {
    let excluded_directories = ["target", ".git", ".gitignore", ".rust_git"];
    if let Some(name) = entry.file_name().to_str() {
        excluded_directories.contains(&name)
    } else {
        false
    }
}

fn read_files(path: &Path, files: &mut HashMap<String, String>) -> Result<(), std::io::Error>{
    if path.is_file() {
        let value = fs::read(path)?;
        let hash = Hasher::hash( &value);
        files.insert(path.display().to_string(), hash);
    }

    if path.is_dir() {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if !is_excluded_directory(&entry){
                        let _ = read_files(&entry.path(), files);
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn create_file_and_their_folders(path: &Path, content: &str) -> Result<(), std::io::Error>{
    if let Some(parent) = path.parent(){
        if !parent.exists(){
            fs::create_dir_all(parent)?;
        }
    }
    let mut file = fs::File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn delete_all_files_and_folders(path: &Path) -> Result<(), std::io::Error>{
    if let Ok(entries) = fs::read_dir(path){
        for entry in entries{
            if let Ok(entry) = entry{     
                if !is_excluded_directory(&entry){
                    let path = entry.path();
                    if path.is_dir(){
                        fs::remove_dir_all(path)?
                    }
                    else{
                        fs::remove_file(path)?
                    }
                }
            }
        }
    }
    Ok(())
}