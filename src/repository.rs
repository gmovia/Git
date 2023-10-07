use std::{collections::HashMap, fs::File};

use crate::commands::init::Init;

pub struct Repository {
    name: String,
    files: HashMap<String, File>,
    is_shared: bool,
}

impl Repository {

    /// Inicia el repositorio 
    pub fn init(repository_name: &str) -> Repository {

        let new_files: HashMap<String, File> = HashMap::new();

        Repository { name: repository_name.to_string(), files: new_files, is_shared: true }
    }


}