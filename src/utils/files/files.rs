use std::{collections::HashMap, fs, path::Path};

/// Recibe un string que representa una ruta.
/// Devuelve los archivos y carpetas que esta contiene en formato HashMap. La clave representa la ruta al archivo y el valor su contenido.

pub fn read(path: String) -> Result<HashMap<String, String>, std::io::Error> {
    read_files(path, HashMap::new())
}

fn read_files(
    path: String,
    mut files: HashMap<String, String>,
) -> Result<HashMap<String, String>, std::io::Error> {
    files.insert(path.clone(), path.clone());

    let path = Path::new(&path);

    if let Ok(entrys) = fs::read_dir(path) {
        for entry in entrys {
            if let Ok(entry) = entry {
                if entry.path().is_file() {
                    let value = fs::read_to_string(&entry.path())?;
                    if let Ok(path_name) = entry.path().into_os_string().into_string() {
                        files.insert(path_name, value);
                    }
                }
                if entry.metadata()?.is_dir() {
                    if let Ok(path_name) = entry.path().into_os_string().into_string() {
                        files = read_files(path_name, files.clone())?;
                    }
                }
            }
        }
    }
    Ok(files)
}
