use crate::{
    file::VSCFile,
    utils::{
        files::files::read,
        sets::sets::{difference, idem_set_different_content},
    }, repository::{self, Repository},
};
use std::{collections::HashMap, path::Path};

pub struct VersionControlSystem {
    pub path: String,
    pub local_repository: Repository,
    pub staging_area: HashMap<String, VSCFile>,
}

impl VersionControlSystem {
    /// Inicializacion del versionControlSystem --> posee el repositorio local y la ruta de la carpeta a informar.
    pub fn init(repository_name:&str, path: String) -> VersionControlSystem {
        
        let mut repository = Repository::init(repository_name);
        
        VersionControlSystem {
            path,
            local_repository: repository,
            staging_area: HashMap::new(),
        }
    }


}