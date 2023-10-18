use std::{path::Path, fs::File};

use super::init::Init;


pub struct Branch;

impl Branch{

    pub fn branch(path: &str, branch_name: &str) -> Result<(), std::io::Error>{
        Self::create_new_branch(path, branch_name)?;
        Ok(())
    }

    pub fn create_new_branch(path: &str,branch_name: &str) -> Result<(),std::io::Error> { 
        let p = Path::new(path);
        let branch_path = p.join(".rust_git").join("refs").join("heads").join(branch_name);
        let _ = File::create(&branch_path)?;
        Init::create_log_file(path, branch_name)?;
        Ok(())
    }
}