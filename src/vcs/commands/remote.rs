use std::{path::{Path, PathBuf}, fs::{OpenOptions, self}, io::{Write, Read, BufReader, BufRead, self}};

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

    pub fn get_pathbuf_of_repo_remote(repo_name: &str) -> Result<PathBuf, std::io::Error> {
        let open_path = Path::new(repo_name);
        let path = Init::get_current_config(open_path)?;
    
        let file = fs::File::open(path)?;
        let reader = io::BufReader::new(file);
    
        for line in reader.lines() {
            let line = line?;
            if line.contains("[remote 'origin']") && line.starts_with("path") {
                let parts: Vec<&str> = line.split('=').map(|s| s.trim()).collect();
                if let Some(value) = parts.get(1) {
                    let mut path_buf = PathBuf::new();
                    path_buf.push(value);
                    return Ok(path_buf);
                }
            }
        }
        Err(io::Error::new(io::ErrorKind::NotFound, "Path not found"))
    }

    pub fn get_path_of_repo_remote(repo_name: &str) -> Result<PathBuf, std::io::Error> {
        let open_path = Path::new(repo_name);
        let path = Init::get_current_config(open_path)?;
    
        let file = fs::File::open(path)?;
        let reader = io::BufReader::new(file);
    
        for line in reader.lines() {
            let line = line?;
            if line.contains("[remote 'origin']") && line.starts_with("path") {
                let parts: Vec<&str> = line.split('=').map(|s| s.trim()).collect();
                if let Some(value) = parts.get(1) {
                    let split:Vec<&str> = value.split("/").collect();
                    let mut path_buf = PathBuf::new();
                    path_buf.push(split[1]);
                    return Ok(path_buf);
                }
            }
        }
        Err(io::Error::new(io::ErrorKind::NotFound, "Path not found"))
    }
    

}