use std::{collections::HashMap, path::Path};
use crate::utils::{files::files:: read, sets::sets::{difference, idem_set_different_content}};


pub struct VersionControlSystem{
    path: String,
    pub local_repository: HashMap<String, String>,
}

impl VersionControlSystem{

    /// Inicializacion del versionControlSystem --> posee el repositorio local y la ruta de la carpeta a informar.
    pub fn init(path: String) -> VersionControlSystem{
        VersionControlSystem{
            path,
            local_repository: HashMap::new(), 
        }
    }

    /// Git Status.
    /// Me devuelve la informacion de los archivos creados y modificados recientemente (en comparacion con el repositorio local).
    /// Tambien me da informacion de los archivos eliminados recientemente.
    pub fn status(&self) -> Result<Vec<String>, std::io::Error>{
        let files = read(Path::new(&self.path.clone()))?;
        let mut status: Vec<String> = Vec::new();
        for key in difference(files.clone(), self.local_repository.clone()).keys(){
            status.push(format!("CREATE: {:?}",key));
        }
        for key in idem_set_different_content(files.clone(), self.local_repository.clone()).keys(){
            status.push(format!("UPDATE: {:?}",key));
        }
        for key in difference(self.local_repository.clone(),files).keys(){
            status.push(format!("DELETE: {:?}",key));
        }
        Ok(status)
    }

}
