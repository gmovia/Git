use crate::{
    vcs::files::vcs_file::VCSFile,
    utils::files::files::read,
    types::types::{ChangesNotStagedForCommit, ChangesToBeCommited, UntrackedFiles},
    vcs::commands::{status::Status, add::Add, init::Init},
};
use std::{collections::HashMap, path::Path};

use super::files::index::Index;

pub struct VersionControlSystem {
    pub path: String,
    pub local_repository: HashMap<String, String>,
    pub index: Index
}

impl VersionControlSystem {
    /// Inicializacion del versionControlSystem --> posee el repositorio local y la ruta de la carpeta a informar.
    pub fn init(path: &str, args: Vec<String>) -> VersionControlSystem {
        let _ = Init::git_init(path, args);
        VersionControlSystem {
            path: path.to_string(),
            local_repository: HashMap::new(),
            index: Index::init(path)
        }
    }

    /// Devuelve la informacion de los archivos creados, modificados y eliminados recientemente, junto con el area de staging.
    pub fn status(&self) -> Result<(UntrackedFiles, ChangesNotStagedForCommit, ChangesToBeCommited), std::io::Error> {
        let files = read(Path::new(&self.path.clone()))?;
        let staging_area = self.index.read_index_write_staging()?;
        Ok(Status::status(&files, &staging_area, &self.local_repository))
    }

    /// Recibe un path
    /// Agrega los archivos que se encuentran dentro del path al area de staging
    /// Devuelve el area de staging
    pub fn add(&mut self, path: &Path) -> Result<HashMap<String, VCSFile>, std::io::Error> {
        Add::add(self, path)        
    }

}