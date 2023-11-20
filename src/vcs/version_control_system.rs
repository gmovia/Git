use crate::{
    vcs::files::vcs_file::VCSFile,
    utils::files::files::read,
    types::types::{ChangesNotStagedForCommit, ChangesToBeCommited, UntrackedFiles},
    vcs::{commands::{status::Status, add::Add, init::Init, hash_object::HashObject,cat_file::CatFile}, files::repository::Repository}, constants::constants::{RESPONSE_NOK_GIT_IGNORE, RESPONSE_OK_IGNORE}, client::client::Client,};

use super::{commands::{hash_object::WriteOption, rm::{Rm, RemoveOption}, commit::Commit, log::Log, branch::{Branch, BranchOptions}, checkout::{Checkout, CheckoutOptions}, merge::Merge, reset::Reset, ls_files::{LsFilesOptions, LsFiles}, ls_tree::LsTree, check_ignore::CheckIgnore}, entities::conflict::Conflict, files::{repositories::Repositories, current_repository::CurrentRepository}};
use std::{collections::HashMap, path::Path};
use super::files::index::Index;

#[derive(Debug, Clone)]
pub struct VersionControlSystem;

impl VersionControlSystem {

    pub fn init(path: &Path, args: Vec<String>){
        let _ = Repositories::write(path);
        let _ = Init::git_init(&path.to_path_buf(), args);
    }
    
    pub fn status() -> Result<(UntrackedFiles, ChangesNotStagedForCommit, ChangesToBeCommited), std::io::Error> {
        let current = CurrentRepository::read()?;
        let files = read(&current)?;
        let staging_area = Index::read_index()?;
        let repository = Repository::read_repository()?;
        Ok(Status::status(&files, &staging_area, &repository))
    }

    pub fn add(path: &Path) -> Result<HashMap<String, VCSFile>, std::io::Error> {
        Add::add(path)        
    }

    pub fn reset(path: &Path) -> Result<HashMap<String, VCSFile>, std::io::Error>{
        Reset::reset(path.to_path_buf())
    }

    pub fn hash_object(path: &Path, option: WriteOption, _type: &str) -> Result<String, std::io::Error>{
        let current = CurrentRepository::read()?;
        let object_path = Init::get_object_path(&current)?;
        HashObject::hash_object(path, object_path, option, _type)
    }

    pub fn cat_file(hash: &str) -> Result<String, std::io::Error>{
        let current = CurrentRepository::read()?;
        let object_path = Init::get_object_path(&current)?;
        CatFile::cat_file(hash, object_path)
    }

    pub fn rm(path: &Path, option: RemoveOption) -> Result<HashMap<String, VCSFile>, std::io::Error> {
        Rm::rm(path, option)
    }

    pub fn commit(message: String) -> Result<HashMap<String, String>, std::io::Error>{
        Commit::commit(message)
    }

    pub fn log() -> Result<String, std::io::Error> {
        Log::log()
    }

    pub fn branch(option: BranchOptions) -> Result<Vec<String>, std::io::Error>{
        let current = CurrentRepository::read()?;
        Branch::branch(&current, option)
    }
    
    pub fn checkout(option: CheckoutOptions) -> Result<(), std::io::Error>{
        let current = CurrentRepository::read()?;
        Checkout::checkout(&current, option)
    }

    pub fn merge(branch: &str) -> Result<HashMap<String, Conflict>,std::io::Error> {
        Merge::merge(branch, HashMap::new())
    }

    pub fn resolve_conflicts(branch: &str, conflicts: HashMap<String, Conflict>) -> Result<HashMap<String, Conflict>,std::io::Error> {
        Merge::merge(branch, conflicts)
    }

    pub fn git_clone(message: String)-> Result<(), std::io::Error>{
        let current = CurrentRepository::read()?;
        let _ = Client::client(message, &current);
        Ok(())
    }

    pub fn ls_files(option: LsFilesOptions) -> Result<Vec<String>,std::io::Error>{
        let current = CurrentRepository::read()?;
        LsFiles::ls_files(option, &current)
    }

    pub fn ls_tree(branch: &str) -> Result<Vec<String>, std::io::Error>{
        let current = CurrentRepository::read()?;
        LsTree::ls_tree(branch, &current)
    }

    pub fn check_ignore(path: &Path) -> Result<String, std::io::Error> {
        if CheckIgnore::check_ignore(&path)?{
            return Ok(RESPONSE_OK_IGNORE.to_string());
        }
        return Ok(RESPONSE_NOK_GIT_IGNORE.to_string());
    }
}