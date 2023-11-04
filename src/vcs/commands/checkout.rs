use std::{path::{Path, PathBuf}, fs::File, io::{Write, self}};

use crate::{vcs::files::{repository::Repository, index::Index}, utils::files::files::{delete_all_files_and_folders, create_file_and_their_folders}};

use super::{branch::Branch, cat_file::CatFile, init::Init};


pub struct Checkout;

#[derive(Clone)]
pub enum CheckoutOptions<'a>{
    ChangeBranch(&'a str),
    CreateAndChangeBranch(&'a str),
}

impl Checkout{

    pub fn checkout(path: &PathBuf, option: CheckoutOptions) -> Result<(), std::io::Error>{
        match option {
            CheckoutOptions::ChangeBranch(branch_name) => {Self::change_branch(path, branch_name)?;},
            CheckoutOptions::CreateAndChangeBranch(branch_name) => {Self::create_and_change_branch(path, branch_name)?;},
        }
        Ok(())
    }

    pub fn change_branch(path: &PathBuf, branch_name: &str) -> Result<(), std::io::Error>{
        let index = Index::init(&path.to_path_buf());
        if !index.read_index()?.is_empty(){
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Can't change branch if you have changes to be commited"));
        }
        let rust_git_path = path.join(".rust_git");
        let head_path = rust_git_path.join("HEAD");
        let mut file = File::create(head_path)?;
        file.write_all(format!("refs/heads/{}", branch_name).as_bytes())?;
        Self::update_cd(path)?;
        Ok(())
    }


    pub fn update_cd(path: &PathBuf) -> Result<(), std::io::Error>{
        let repository = Repository::init(path.to_path_buf());
        let repository_hashmap = repository.read_repository()?;

        delete_all_files_and_folders(path)?;

        for (key, value) in repository_hashmap{
            let content = CatFile::cat_file(&value, Init::get_object_path(path)?)?;
            create_file_and_their_folders(Path::new(&key), &content)?
        }
        Ok(())
    }

    pub fn create_and_change_branch(path: &PathBuf, branch_name: &str) -> Result<(), std::io::Error>{
        Branch::create_new_branch(path, branch_name)?;
        Self::change_branch(path, branch_name)?;
        Ok(())
    }

}