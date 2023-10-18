use std::{path::Path, fs::{File, self}, io};

use super::init::Init;


pub struct Branch;

pub enum BranchOptions<'a>{
    NewBranch(&'a str),
    DeleteBranch(&'a str),
    GetBranchs,
}

impl Branch{

    pub fn branch(path: &str, option: BranchOptions) -> Result<(), std::io::Error>{
        match option{
            BranchOptions::NewBranch(branch_name) => {Self::create_new_branch(path, branch_name)?;},
            BranchOptions::DeleteBranch(branch_name) => {Self::delete_branch(path, branch_name)?;},
            BranchOptions::GetBranchs => {Self::get_branchs(path)?;},
        }
        Ok(())
    }



    pub fn create_new_branch(path: &str,branch_name: &str) -> Result<(),std::io::Error> { 
        let p = Path::new(path);
        let branch_path = p.join(".rust_git").join("refs").join("heads").join(branch_name);
        let _ = File::create(&branch_path)?;
        Init::create_log_file(path, branch_name)?;
        Ok(())
    }

    pub fn delete_branch(path: &str, branch_name: &str) -> Result<(),std::io::Error>{
        let p = Path::new(path);
        let branch_path = p.join(".rust_git").join("refs").join("heads").join(branch_name);
        let logs_path = p.join(".rust_git").join("logs").join(branch_name);
        if logs_path == Init::get_commits_path(&path.to_string())?{
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Can't remove the actual branch"));
        }
        fs::remove_file(branch_path)?;
        fs::remove_file(logs_path)?;
        Ok(())
    }

    pub fn get_branchs(path: &str) -> Result<(),std::io::Error>{
        let p = Path::new(path);
        let branchs_dir_path = p.join(".rust_git").join("refs").join("heads");
        if let Ok(entries) = fs::read_dir(branchs_dir_path){
            for entry in entries{
                if let Ok(entry) = entry{
                    if let Some(file_name) = entry.path().file_name(){
                        println!("{:?}",file_name.to_string_lossy().to_string());
                    }

                }
            }
        }
        Ok(())
    }
}