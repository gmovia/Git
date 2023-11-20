use crate::vcs::files::vcs_file::VCSFile;
use std::{path::PathBuf, fs::{OpenOptions, File}, io::{Write, self, BufRead}, collections::HashMap};
use super::current_repository::CurrentRepository;

#[derive(Debug, Clone)]
pub struct Index;

impl Index{
    pub fn index_path() -> Result<PathBuf, io::Error>{
        let vcs_path = CurrentRepository::read()?;
        let path = vcs_path.join(".rust_git").join("index");
        Ok(path)
    }

    /// Vacia index
    pub fn clear() -> Result<(), std::io::Error>{
        Self::write_index(&HashMap::new())
    }

    /// Recibe el area de staging.
    /// Vacia index y luego escribe cada archivo del area de staging en el.
    pub fn write_index(staging_area: &HashMap<String, VCSFile>) -> Result<(),std::io::Error>{
        let mut index_file = OpenOptions::new().read(true).write(true).open(Self::index_path()?)?;
        index_file.set_len(0)?;
        for value in staging_area.values(){
            let _ = Self::add(&mut index_file,value);
        }
        Ok(())
    }
    
    /// Lee index
    /// Crea el hashmap del area de staging insertando cada linea de index.
    /// Devuelve el hashmap.
    pub fn read_index() -> Result<HashMap<String,VCSFile>,std::io::Error>{
        let mut staging_area:HashMap<String, VCSFile>  = HashMap::new();
        let index_file = OpenOptions::new().read(true).open(Self::index_path()?)?;
        let reader = io::BufReader::new(index_file);
        for line in reader.lines().map_while(Result::ok){
            let parts: Vec<&str> = line.split('-').collect();
            let file = VCSFile::new(parts[0].to_string(), parts[2].to_string(), parts[1].to_string());
            staging_area.insert(parts[0].to_string(), file);
        }
        Ok(staging_area)
    }
    
    /// Recibe index y un archivo
    /// Escribe el archivo en index
    fn add(index: &mut File, file: &VCSFile) -> Result<(),std::io::Error> {
        let line = format!("{}-{}-{}\n", file.path, file.state, file.content);
        index.write_all(line.as_bytes())?;
        Ok(())
    }
}