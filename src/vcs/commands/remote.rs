use std::{path::Path, fs::OpenOptions};

use super::init::Init;

pub struct Remote;

impl Remote{

    pub fn remote(current_repo: &Path, new_repo_name: String, server_repo: &Path) -> Result<(), std::io::Error>{
        let _ = Self::write_config(current_repo, new_repo_name, server_repo);
        Ok(())
    }

    fn write_config(current_repo: &Path, new_repo_name: String, server_repo: &Path) -> Result<(), std::io::Error>{
        let mut config_file = OpenOptions::new().write(true).create(true).append(true).open(Init::get_current_config(&current_repo)?)?; 
        Ok(())
    }

}