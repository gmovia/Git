use std::{collections::HashMap, fs, path::Path};

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
        let value = fs::read_to_string(path)?;
        files.insert(path.display().to_string(), value);
    }

    if path.is_dir() {
        if let Ok(entrys) = fs::read_dir(path) {
            for entry in entrys {
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