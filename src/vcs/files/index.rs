use crate::vcs::files::vcs_file::VCSFile;
use std::{path::Path, fs::{self, OpenOptions, File}, io::Write, collections::HashMap};

/// Recibe el area de staging.
/// Vacia index y luego escribe cada archivo del area de staging en el.
pub fn write(staging_area: &HashMap<String, VCSFile>) -> Result<(),std::io::Error>{
    fs::create_dir_all(".rust_git")?;
    let index_path = Path::new(".rust_git/index");
    let mut index_file = OpenOptions::new().write(true).create(true).append(true).open(&index_path)?;
    let _ = clear(&mut index_file);
    for value in staging_area.values(){
        let _ = add(&mut index_file,&value);
    }
    Ok(())
}

/// Recibe index.
/// Vacia index.
pub fn clear(index: &mut File) -> Result<(), std::io::Error>{
    let _ = index.set_len(0);
    Ok(())
}

/// Recibe index y un archivo
/// Escribe el archivo en index
pub fn add(index: &mut File, file: &VCSFile) -> Result<(),std::io::Error> {
    index.write_all(file.path.as_bytes())?;
    index.write_all("-".as_bytes())?;
    index.write_all(file.state.as_bytes())?;
    index.write_all("-".as_bytes())?;
    index.write_all(file.content.as_bytes())?;                
    index.write_all("\n".as_bytes())?;
    Ok(())
}