use crate::{
    vcs::files::vcs_file::VCSFile,
    utils::files::file::read,
    types::set_type::{ChangesNotStagedForCommit, ChangesToBeCommited, UntrackedFiles},
    vcs::{commands::{status::Status, add::Add, init::Init, hash_object::HashObject,cat_file::CatFile}, files::repository::Repository}, constants::constant::{RESPONSE_NOK_IGNORE, RESPONSE_OK_IGNORE}, clients::client::Client,};

use super::{commands::{hash_object::WriteOption, rm::{Rm, RemoveOption}, commit::Commit, log::Log, branch::{Branch, BranchOptions}, checkout::{Checkout, CheckoutOptions}, merge::Merge, reset::Reset, ls_files::{LsFilesOptions, LsFiles}, ls_tree::LsTree, check_ignore::CheckIgnore, tag::{TagOptions, Tag}, show_ref::{ShowRefOptions, ShowRef}, remote::Remote, pull::Pull}, entities::conflict::Conflict, files::{repositories::Repositories, current_repository::CurrentRepository}};
use std::{collections::HashMap, path::{Path, PathBuf}};
use super::files::index::Index;

#[derive(Debug, Clone)]
pub struct VersionControlSystem;

impl VersionControlSystem {

    pub fn init(path: &Path, args: Vec<String>){
        let _ = Repositories::write(path);
        Init::git_init(path, args);
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
        Reset::reset(path)
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


    pub fn ls_files(option: LsFilesOptions) -> Result<Vec<String>,std::io::Error>{
        let current = CurrentRepository::read()?;
        LsFiles::ls_files(option, &current)
    }


    pub fn ls_tree(branch: &str) -> Result<Vec<String>, std::io::Error> {
        let current = CurrentRepository::read()?;
        LsTree::ls_tree(branch, &current)
    }


    pub fn check_ignore(path: &Path) -> Result<String, std::io::Error> {
        if CheckIgnore::check_ignore(path)?{
            return Ok(RESPONSE_OK_IGNORE.to_string());
        }
        Ok(RESPONSE_NOK_IGNORE.to_string())
    }

    pub fn tag(option: TagOptions) -> Result<Vec<String>, std::io::Error> {
        let current = CurrentRepository::read()?;
        Tag::tag(&current, option)
    }

    pub fn show_ref(option: ShowRefOptions) -> Result<HashMap<String, String>, std::io::Error> {
        let current = CurrentRepository::read()?;
        ShowRef::show_ref(&current, option)
    }

    pub fn clone(message: String, path_to_clone: &Path)-> Result<(), std::io::Error>{
        Client::client(message, path_to_clone)
    }

    pub fn fetch(message: String)-> Result<(), std::io::Error>{
        let input: Vec<&str>  = message.split_ascii_whitespace().collect();
        let current = CurrentRepository::read()?;
        let repo_to_fetch = Remote::get_path_of_repo_remote(&current, input[2])?;
        let _ = Client::client(message, &repo_to_fetch);
        Ok(())
    }
    
    pub fn pull(message: String) -> Result<(), std::io::Error> {
        Pull::pull(message)
    }

    pub fn push(message: String)-> Result<(), std::io::Error>{
        let input: Vec<&str>  = message.split_ascii_whitespace().collect();
        let current = CurrentRepository::read()?;
        let repo_to_push = Remote::get_path_of_repo_remote(&current, input[2].trim_end_matches("\n"))?;
        let _ = Client::client(message, &repo_to_push);
        Ok(())
    }

    pub fn remote(new_repo_name :String, repo_server: &Path) -> Result<(), std::io::Error>{
        let current = CurrentRepository::read()?;
        Remote::remote(&current, new_repo_name, repo_server)?;
        Ok(())
    }
}