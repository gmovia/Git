use std::{path::PathBuf, fs};

use crate::utils::files::files::{read, is_excluded_directory};



pub struct LsFiles;

pub enum LsFilesOptions {
    Everything,
    OnlyModified,
    OnlyStaging,
    OnlyDeleted,
}

impl LsFiles {

    pub fn ls_files(option: LsFilesOptions, path: &PathBuf) -> Result<Vec<String>, std::io::Error> {
        let mut files: Vec<String> = Vec::new();
        match option {
            LsFilesOptions::Everything => {Ok(Self::get_everything(path,&mut files)?)},
            LsFilesOptions::OnlyModified => todo!(),
            LsFilesOptions::OnlyStaging => todo!(),
            LsFilesOptions::OnlyDeleted => todo!(),
        }
    }

    pub fn get_everything(path: &PathBuf, files: &mut Vec<String>) -> Result<Vec<String>,std::io::Error>{
        let files_hashmap = read(&path)?;
        for (key, _) in files_hashmap {
            files.push(key);
        }
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if is_excluded_directory(&entry) {
                        Self::get_everything(&entry.path(), files)?;
                        
                    }
                }
            } 
        }
        Ok(files.to_vec())
    }
}