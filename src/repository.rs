use std::{collections::HashMap, fs::File};

pub struct Repository {
    name: String,
    files: HashMap<String, File>,
}

impl Repository {

    fn init(repository_name: String ) -> Repository {

        let mut new_files: HashMap<String, File> = HashMap::new();
        Repository { name: repository_name, files: new_files }
    }

    


}