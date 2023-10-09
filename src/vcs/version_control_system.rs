use crate::{
    vcs::files::vcs_file::VCSFile,
    utils::files::files::read,
    types::types::{ChangesNotStagedForCommit, ChangesToBeCommited, UntrackedFiles},
    vcs::commands::{status::Status, add::Add},
};
use std::{collections::HashMap, path::Path};

pub struct VersionControlSystem {
    pub path: String,
    pub local_repository: HashMap<String, String>,
    pub staging_area: HashMap<String, VCSFile>,
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

    /// Devuelve la informacion de los archivos creados, modificados y eliminados recientemente, junto con el area de staging.
    pub fn status(&self) -> Result<(UntrackedFiles, ChangesNotStagedForCommit, ChangesToBeCommited), std::io::Error> {
        let files = read(Path::new(&self.path.clone()))?;
        Ok(Status::status(&files, &self.staging_area, &self.local_repository))
    }

    /// Recibe un path
    /// Agrega los archivos que se encuentran dentro del path al area de staging
    /// Devuelve el area de staging
    pub fn add(&mut self, path: &Path) -> Result<HashMap<String, VCSFile>, std::io::Error> {
        Add::add(self, path)        
    }
}
