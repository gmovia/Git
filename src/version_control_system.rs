use crate::{
    file::VSCFile,
    utils::{
        files::files::read,
        sets::sets::{difference, idem_set_different_content},
    },
};
use std::{collections::HashMap, path::Path};

pub struct VersionControlSystem {
    path: String,
    pub local_repository: HashMap<String, String>,
    pub staging_area: HashMap<String, VSCFile>,
}

impl VersionControlSystem {
    /// Inicializacion del versionControlSystem --> posee el repositorio local y la ruta de la carpeta a informar.
    pub fn init(path: String) -> VersionControlSystem {
        VersionControlSystem {
            path,
            local_repository: HashMap::new(),
            staging_area: HashMap::new(),
        }
    }

    /// Me devuelve la informacion de los archivos creados y modificados recientemente (en comparacion con el repositorio local).
    /// Tambien me da informacion de los archivos eliminados recientemente.

    pub fn status(&self) -> Result<HashMap<String, String>, std::io::Error> {
        let files = read(Path::new(&self.path.clone()))?;
        let mut status = HashMap::new();
        for key in difference(files.clone(), self.local_repository.clone()).keys() {
            status.insert(key.clone(), "CREATED".to_string());
        }
        for key in idem_set_different_content(files.clone(), self.local_repository.clone()).keys() {
            status.insert(key.clone(), "MODIFIED".to_string());
        }
        for key in difference(self.local_repository.clone(), files).keys() {
            status.insert(key.clone(), "DELETED".to_string());
        }
        Ok(status)
    }

    pub fn add(&mut self, path: &Path) -> Result<HashMap<String, VSCFile>, std::io::Error> {
        let status = self.status()?;
        let files = read(path)?;

        for (key, value) in &files {
            if status.contains_key(key) {
                if let Some(state) = status.get(key) {
                    let file = VSCFile::new(key.clone(), value.clone(), state.clone());
                    self.staging_area.insert(key.to_string(), file);
                }
            }
        }
        Ok(self.staging_area.clone())
    }
}
