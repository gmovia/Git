use crate::{
    staging_area::StagingArea,
    utils::{
        files::files::read,
        sets::sets::{difference, idem_set_different_content},
    },
};
use std::{collections::HashMap, path::Path};

pub struct VersionControlSystem {
    path: String,
    pub local_repository: HashMap<String, String>,
    pub staging_area: StagingArea,
}

impl VersionControlSystem {
    /// Inicializacion del versionControlSystem --> posee el repositorio local y la ruta de la carpeta a informar.
    pub fn init(path: String) -> VersionControlSystem {
        VersionControlSystem {
            path,
            local_repository: HashMap::new(),
            staging_area: StagingArea::new(),
        }
    }

    /// Git Status.
    /// Me devuelve la informacion de los archivos creados y modificados recientemente (en comparacion con el repositorio local).
    /// Tambien me da informacion de los archivos eliminados recientemente.
    pub fn status(&self) -> Result<Vec<String>, std::io::Error> {
        let files = read(Path::new(&self.path.clone()))?;
        let mut status: Vec<String> = Vec::new();
        for key in difference(files.clone(), self.local_repository.clone()).keys() {
            status.push(format!("CREATE: {:?}", key));
        }
        for key in idem_set_different_content(files.clone(), self.local_repository.clone()).keys() {
            status.push(format!("UPDATE: {:?}", key));
        }
        for key in difference(self.local_repository.clone(), files).keys() {
            status.push(format!("DELETE: {:?}", key));
        }
        Ok(status)
    }

    pub fn return_status(
        &self,
        files: &HashMap<String, String>,
        area: &HashMap<String, String>,
    ) -> HashMap<String, String> {
        let mut file_statuses = HashMap::new();

        let new_files = difference(files.clone(), area.clone());
        for key in new_files.keys() {
            file_statuses.insert(key.clone(), "new file".to_string());
        }

        let updated_files = idem_set_different_content(files.clone(), area.clone());
        for key in updated_files.keys() {
            file_statuses.insert(key.clone(), "modified".to_string());
        }

        let deleted_files = difference(area.clone(), files.clone());
        for key in deleted_files.keys() {
            file_statuses.insert(key.clone(), "deleted".to_string());
        }

        file_statuses
    }

    //1. Func para un git add . 
    pub fn add(&mut self) -> Result<Vec<String>, std::io::Error> {
        let current_dir = Path::new(&self.path);
        let files = read(current_dir)?;
        let status = Vec::new();

        let file_statuses = self.return_status(&files, &self.staging_area.area);

        for (key, file_status) in file_statuses {
            match file_status.as_str() {
                "new file" | "modified" => {
                    self.staging_area
                        .area
                        .insert(key.clone(), files[&key].clone());
                }
                "deleted" => {
                    self.staging_area.area.remove(&key);
                }
                _ => (),
            }
        }

        Ok(status)
    }
}
