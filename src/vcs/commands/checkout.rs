use std::{path::Path, fs::File, io::Write};

use super::branch::Branch;


pub struct Checkout;

pub enum CheckoutOptions<'a>{
    ChangeBranch(&'a str),
    CreateAndChangeBranch(&'a str),
    ReviewCommit(&'a str),
}

impl Checkout{

    pub fn checkout(path: &str, option: CheckoutOptions) -> Result<(), std::io::Error>{
        match option {
            CheckoutOptions::ChangeBranch(branch_name) => {Self::change_branch(path, branch_name)?;},
            CheckoutOptions::CreateAndChangeBranch(branch_name) => {Self::create_and_change_branch(path, branch_name)?;},
            CheckoutOptions::ReviewCommit(hash) => {Self::review_commit(path, hash)?;},
        }
        Ok(())
    }

    pub fn change_branch(path: &str, branch_name: &str) -> Result<(), std::io::Error>{
        let p = Path::new(path);
        let rust_git_path = p.join(".rust_git");
        let head_path = rust_git_path.join("HEAD");
        let mut file = File::create(head_path)?;
        file.write_all(format!("refs/heads/{}", branch_name).as_bytes())?;
        Ok(())
    }

    pub fn create_and_change_branch(path: &str, branch_name: &str) -> Result<(), std::io::Error>{
        Branch::create_new_branch(path.into(), branch_name)?;
        Self::change_branch(path, branch_name)?;
        Ok(())
    }

    pub fn review_commit(path: &str,hash: &str) -> Result<(), std::io::Error>{ //ANALIZAR MEJOR
    //     CatFile::cat_file(hash, Init::get_object_path(&path.to_string())?)?;
         Ok(())
    }
}