use std::{path::Path, fs::{OpenOptions, self}, io::{Write, Read, BufReader, BufRead}};

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

    pub fn remote_added(current_repo: &Path) -> Result<bool, std::io::Error>{
        let path = Init::get_current_config(current_repo)?;
        let file = fs::File::open(path)?;
        let reader = BufReader::new(file); 
    
        for line in reader.lines() {
            let line = line?;
            if line.contains("remote") {
                return Ok(true);
            }
        }
        Ok(false)
    }
    

}