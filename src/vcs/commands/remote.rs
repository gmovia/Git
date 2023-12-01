use std::{path::{Path, PathBuf}, fs::{OpenOptions, self, File}, io::{Write, Read, BufReader, BufRead, self}};

use crate::vcs::files::current_repository::CurrentRepository;

use super::init::Init;

pub struct Remote;

impl Remote{
    //git remote add origin repo1
    pub fn remote(current_repo: &Path, new_repo_name: String, server_repo: &Path) -> Result<(), std::io::Error>{
        let _ = Self::write_config(current_repo, new_repo_name, server_repo);
        Ok(())
    }

    fn write_config(current_repo: &Path, new_repo_name: String, server_repo: &Path) -> Result<(), std::io::Error>{
        let mut config_file = OpenOptions::new().write(true).create(true).append(true).open(Init::get_current_config(&current_repo.to_path_buf())?)?; 
        let msge_format = format!("\n[remote {new_repo_name}]\n\tpath = {}",server_repo.display());

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

    pub fn read_remote_names() -> Result<Vec<String>, std::io::Error>{
        let path = CurrentRepository::read()?;
        let path_config = Init::get_current_config(&path)?;
        let file = File::open(&path_config)?;
        let reader = io::BufReader::new(file);
    
        let mut remote_names = Vec::new();
        let mut in_remote_section = false;
    
        for line in reader.lines() {
            let line = line?;
            let trimmed_line = line.trim();
    
            if trimmed_line.starts_with("[remote '") {
                in_remote_section = true;
                if let Some(name) = trimmed_line.strip_prefix("[remote '") {
                    if let Some(name) = name.strip_suffix("']") {
                        remote_names.push(name.to_string());
                    }
                }
            } else if in_remote_section && trimmed_line.starts_with("path = ") {
                in_remote_section = false;
            }
        }
        Ok(remote_names)
    }

    pub fn get_path_of_repo_remote(current_repo: &Path, repo_name: &str) -> Result<PathBuf, io::Error> {
        let path = Init::get_current_config(current_repo)?;    
        let content = fs::read_to_string(path)?;

        let content_split: Vec<&str> = content.split("\n").collect();
        println!("REPO NAME {:?} ", repo_name);
        for (index, line) in content_split.iter().enumerate(){
            if line.contains(repo_name){
                println!("LINE ----> {:?}", line);
                let parts: Vec<&str> = content_split[index+1].split('=').map(|s| s.trim()).collect();
                if let Some(value) = parts.get(1) {
                    let mut path_buf = PathBuf::new();
                    path_buf.push(value);
                    return Ok(path_buf);
                }
            }
        }
    
        Err(io::Error::new(io::ErrorKind::NotFound, "Path not found"))
    }
}