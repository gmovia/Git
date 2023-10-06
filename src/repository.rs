use std::{collections::HashMap, fs::File};

use crate::commands::init::Init;

pub struct Repository {
    name: String,
    files: HashMap<String, File>,
}

impl Repository {

    pub fn init(repository_name: &str) -> Repository {

        let new_files: HashMap<String, File> = HashMap::new();
        
        let init = Init::git_init();

        Repository { name: repository_name.to_string(), files: new_files }
    }


}