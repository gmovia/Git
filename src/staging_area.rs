use crate::utils::files::files::read;
use std::{collections::HashMap, path::Path};
pub struct StagingArea {
    pub area: HashMap<String, String>,
}

impl StagingArea {
    pub fn new() -> StagingArea {
        StagingArea {
            area: HashMap::new(),
        }
    }

    /// Recibe un path.
    /// Agrega al area de staging todos los archivos y carpetas.

    pub fn add(&mut self, path: &Path) -> Result<(), std::io::Error> {
        let files = read(path)?;
        for (key, value) in files {
            self.area.insert(key, value);
        }
        Ok(())
    }

    // Recibe un path.
    // Elimina del area de staging todos los archivos y carpetas.

    pub fn remove(&mut self, path: &Path) -> Result<(), std::io::Error> {
        let files = read(path)?;
        for key in files.keys(){
            self.area.remove(key);
        }
        Ok(())    
    }
}
