use std::{path::Path, fs::OpenOptions, io::Write};

use super::init::Init;

pub struct Remote;

impl Remote{
    //git remote add origin test_folder/repo1
    pub fn remote(current_repo: &Path, new_repo_name: String, server_repo: &Path) -> Result<(), std::io::Error>{
        let _ = Self::write_config(current_repo, new_repo_name, server_repo);
        Ok(())
    }

    fn write_config(current_repo: &Path, new_repo_name: String, server_repo: &Path) -> Result<(), std::io::Error>{
        let mut config_file = OpenOptions::new().write(true).create(true).append(true).open(Init::get_current_config(&current_repo.to_path_buf())?)?; 
        let msge_format = format!("[remote {}{new_repo_name}{}]\n\tpath = {}","'", "'",server_repo.display());

        config_file.write_all(msge_format.as_bytes())?;
        Ok(())
    }

}