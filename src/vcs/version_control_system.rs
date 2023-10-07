use crate::{
    vcs::files::vcs_file::VCSFile,
    utils::{
        files::files::read,
        sets::sets::{difference, idem_set_different_content},
    },
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

    /// Devuelve la informacion de los archivos creados y modificados recientemente (en comparacion con el repositorio local).
    /// Tambien me da informacion de los archivos eliminados recientemente.

    pub fn status(&self) -> Result<(HashMap<String, String>, HashMap<String, String>, HashMap<String, String>), std::io::Error> {
        let files = read(Path::new(&self.path.clone()))?;

        let mut area: HashMap<String, String> = HashMap::new();
        for (key, value) in &self.staging_area{
            area.insert(key.to_string(), value.content.to_string());
        }
            
        // UNTRACKED FILES
        let mut untracked_files = HashMap::new();
        for key in difference(difference(files.clone(), self.local_repository.clone()), area.clone()).keys(){
            untracked_files.insert(key.to_string(), "CREATED".to_string());            
        }

        // CHANGES TO BE COMMITED
        let mut changes_to_be_commited = HashMap::new();
        for (key, value) in self.staging_area.clone(){
            changes_to_be_commited.insert(key, value.state.to_string());
        }
        
        // CHANGES NOT STAGED FOR COMMIT
        let mut changes_not_staged_for_commit = HashMap::new();
        
        for key in difference(difference(self.local_repository.clone(), files.clone()),area.clone()).keys(){
            changes_not_staged_for_commit.insert(key.to_string(), "DELETED".to_string());
        }
        
        for key in difference(idem_set_different_content(files.clone(), self.local_repository.clone()),area.clone()).keys(){
            changes_not_staged_for_commit.insert(key.to_string(), "MODIFIED".to_string());
        }
        
        for key in idem_set_different_content(files.clone(), area.clone()).keys(){
            changes_not_staged_for_commit.insert(key.to_string(), "MODIFIED".to_string()); 
        }

        Ok((untracked_files, changes_not_staged_for_commit, changes_to_be_commited))
    }

    /// Recibe un path
    /// Agrega los archivos que se encuentran dentro del path al area de staging
    /// Devuelve el area de staging

    pub fn add(&mut self, path: &Path) -> Result<HashMap<String, VCSFile>, std::io::Error> {
        let (untracked_files, changes_not_staged_for_commit, _) = self.status()?;
    
        if let Ok(files) = read(path) {
            for (key, value) in &files {
                let state = match (untracked_files.get(key), changes_not_staged_for_commit.get(key)) {
                    (Some(state), _) => state.to_string(),
                    (_, Some(_)) if !self.local_repository.contains_key(key) => "CREATED".to_string(),
                    (_, Some(state)) => state.to_string(),
                    _ => continue,
                };
                let file = VCSFile::new(key.clone(), value.clone(), state);
                self.staging_area.insert(key.clone(), file);
            }
        }
    
        Ok(self.staging_area.clone())
    }
    
}
