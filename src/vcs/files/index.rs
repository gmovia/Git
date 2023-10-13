use crate::vcs::files::vcs_file::VCSFile;
use std::{path::Path, fs::{self, OpenOptions, File}, io::{Write, self, BufRead}, collections::HashMap};

pub struct Index{
    pub path: String
}

impl Index{
    pub fn init(vcs_path: &str) -> Index{
        let path = vcs_path.to_owned() + "/.rust_git";
        Index{path}
    }

    /// Recibe el area de staging.
    /// Vacia index y luego escribe cada archivo del area de staging en el.
    pub fn read_staging_write_index(&self, staging_area: &HashMap<String, VCSFile>) -> Result<(),std::io::Error>{
        fs::create_dir_all(&self.path)?;
        let index_path = Path::new(&self.path).join("index");
        let mut index_file = OpenOptions::new().write(true).create(true).append(true).open(&index_path)?;
        let _ = self.clear(&mut index_file);
        for value in staging_area.values(){
            let _ = self.add(&mut index_file,&value);
        }
        Ok(())
    }
    
    /// Lee index
    /// Crea el hashmap del area de staging insertando cada linea de index.
    /// Devuelve el hashmap.
    pub fn read_index_write_staging(&self) -> Result<HashMap<String,VCSFile>,std::io::Error>{
        let mut staging_area:HashMap<String, VCSFile>  = HashMap::new();
        let index_path = Path::new(&self.path).join("index");
        let index_file = OpenOptions::new().read(true).open(&index_path)?;
        let reader = io::BufReader::new(index_file);
        
        for line in reader.lines().filter_map(Result::ok){
            let parts: Vec<&str> = line.split("-").collect();
            let file = VCSFile::new(parts[0].to_string(), parts[2].to_string(), parts[1].to_string());
            staging_area.insert(parts[0].to_string(), file);
        }
        Ok(staging_area)
    }
    
    /// Recibe index.
    /// Vacia index.
    fn clear(&self, index: &mut File) -> Result<(), std::io::Error>{
        let _ = index.set_len(0);
        Ok(())
    }
    
    /// Recibe index y un archivo
    /// Escribe el archivo en index
    fn add(&self, index: &mut File, file: &VCSFile) -> Result<(),std::io::Error> {
        index.write_all(file.path.as_bytes())?;
        index.write_all("-".as_bytes())?;
        index.write_all(file.state.as_bytes())?;
        index.write_all("-".as_bytes())?;
        index.write_all(file.content.as_bytes())?;                
        index.write_all("\n".as_bytes())?;
        Ok(())
    }
}