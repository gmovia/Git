use super::files::index::Index;
use super::{
    commands::{
        branch::{Branch, BranchOptions},
        check_ignore::CheckIgnore,
        checkout::{Checkout, CheckoutOptions},
        commit::Commit,
        hash_object::WriteOption,
        log::Log,
        ls_files::{LsFiles, LsFilesOptions},
        ls_tree::LsTree,
        merge::Merge,
        pull::Pull,
        rebase::Rebase,
        remote::{Remote, RemoteOption},
        reset::Reset,
        rm::{RemoveOption, Rm},
        show_ref::{ShowRef, ShowRefOptions},
        tag::{Tag, TagOptions},
    },
    entities::conflict::Conflict,
    files::{current_repository::CurrentRepository, repositories::Repositories},
};
use crate::{
    clients::client::Client,
    constants::constant::{RESPONSE_NOK_IGNORE, RESPONSE_OK_IGNORE},
    types::set_type::{ChangesNotStagedForCommit, ChangesToBeCommited, UntrackedFiles},
    utils::files::file::read,
    vcs::files::vcs_file::VCSFile,
    vcs::{
        commands::{
            add::Add, cat_file::CatFile, hash_object::HashObject, init::Init, status::Status,
        },
        files::repository::Repository,
    },
};
use std::{collections::HashMap, path::Path};

/// El VersionControlSystem es una especie de handler para el llamado de los diferentes comandos disponibles.
#[derive(Debug, Clone)]
pub struct VersionControlSystem;

impl VersionControlSystem {
    /// Esta funcion handlea lo que respecta al comando init
    pub fn init(path: &Path, args: Vec<String>) {
        let _ = Repositories::write(path);
        Init::git_init(path, args);
    }

    /// Esta funcion handlea lo que respecta al comando status
    pub fn status() -> Result<
        (
            UntrackedFiles,
            ChangesNotStagedForCommit,
            ChangesToBeCommited,
        ),
        std::io::Error,
    > {
        let current = CurrentRepository::read()?;
        let files = read(&current)?;
        let staging_area = Index::read_index()?;
        let repository = Repository::read_repository()?;

        Ok(Status::status(&files, &staging_area, &repository))
    }

    /// Esta funcion handlea lo que respecta al comando add
    pub fn add(path: &Path) -> Result<HashMap<String, VCSFile>, std::io::Error> {
        Add::add(path)
    }

    /// Esta funcion handlea lo que respecta al comando reset
    pub fn reset(path: &Path) -> Result<HashMap<String, VCSFile>, std::io::Error> {
        Reset::reset(path)
    }

    /// Esta funcion handlea lo que respecta al comando hash_object
    pub fn hash_object(
        path: &Path,
        option: WriteOption,
        _type: &str,
    ) -> Result<String, std::io::Error> {
        let current = CurrentRepository::read()?;
        let object_path = Init::get_object_path(&current)?;
        HashObject::hash_object(path, object_path, option, _type)
    }

    /// Esta funcion handlea lo que respecta al comando cat-file
    pub fn cat_file(hash: &str) -> Result<String, std::io::Error> {
        let current = CurrentRepository::read()?;
        let object_path = Init::get_object_path(&current)?;
        CatFile::cat_file(hash, object_path)
    }

    /// Esta funcion handlea lo que respecta al comando rm
    pub fn rm(
        path: &Path,
        option: RemoveOption,
    ) -> Result<HashMap<String, VCSFile>, std::io::Error> {
        Rm::rm(path, option)
    }

    /// Esta funcion handlea lo que respecta al comando commit
    pub fn commit(message: String) -> Result<HashMap<String, String>, std::io::Error> {
        Commit::commit(message)
    }

    /// Esta funcion handlea lo que respecta al comando log
    pub fn log() -> Result<String, std::io::Error> {
        Log::log()
    }

    /// Esta funcion handlea lo que respecta al comando branch
    pub fn branch(option: BranchOptions) -> Result<Vec<String>, std::io::Error> {
        let current = CurrentRepository::read()?;
        Branch::branch(&current, option)
    }

    /// Esta funcion handlea lo que respecta al comando checkout
    pub fn checkout(option: CheckoutOptions) -> Result<(), std::io::Error> {
        let current = CurrentRepository::read()?;
        Checkout::checkout(&current, option)
    }

    /// Esta funcion handlea lo que respecta al comando merge
    pub fn merge(branch: &str) -> Result<HashMap<String, Conflict>, std::io::Error> {
        Merge::merge(branch, HashMap::new())
    }

    /// Esta funcion handlea lo que respecta a la resolucin de conflictos
    pub fn resolve_conflicts(
        branch: &str,
        conflicts: HashMap<String, Conflict>,
    ) -> Result<HashMap<String, Conflict>, std::io::Error> {
        Merge::merge(branch, conflicts)
    }

    /// Esta funcion handlea lo que respecta al comando ls files
    pub fn ls_files(option: LsFilesOptions) -> Result<Vec<String>, std::io::Error> {
        let current = CurrentRepository::read()?;
        LsFiles::ls_files(option, &current)
    }

    /// Esta funcion handlea lo que respecta al comando ls_tree
    pub fn ls_tree(branch: &str) -> Result<Vec<String>, std::io::Error> {
        let current = CurrentRepository::read()?;
        LsTree::ls_tree(branch, &current)
    }

    /// Esta funcion handlea lo que respecta al check ignore
    pub fn check_ignore(path: &Path) -> Result<String, std::io::Error> {
        if CheckIgnore::check_ignore(path)? {
            return Ok(RESPONSE_OK_IGNORE.to_string());
        }
        Ok(RESPONSE_NOK_IGNORE.to_string())
    }

    /// Esta funcion handlea lo que respecta al comando tag
    pub fn tag(option: TagOptions) -> Result<Vec<String>, std::io::Error> {
        let current = CurrentRepository::read()?;
        Tag::tag(&current, option)
    }

    /// Esta funcion handlea lo que respecta al comando show-ref
    pub fn show_ref(option: ShowRefOptions) -> Result<HashMap<String, String>, std::io::Error> {
        let current = CurrentRepository::read()?;
        ShowRef::show_ref(&current, option)
    }

    /// Esta funcion handlea lo que respecta al comando clone
    pub fn clone(message: String, path_to_clone: &Path) -> Result<(), std::io::Error> {
        Client::client(message, path_to_clone)
    }

    /// Esta funcion handlea lo que respecta al comando fetch
    pub fn fetch(message: String) -> Result<(), std::io::Error> {
        let input: Vec<&str> = message.split_ascii_whitespace().collect();
        let current = CurrentRepository::read()?;
        let repo_to_fetch = Remote::get_path_of_repo_remote(&current, input[2])?;
        let _ = Client::client(message, Path::new(&repo_to_fetch));
        Ok(())
    }

    /// Esta funcion handlea lo que respecta al comando pull
    pub fn pull(message: String) -> Result<(), std::io::Error> {
        Pull::pull(message)
    }

    /// Esta funcion handlea lo que respecta al comando push
    pub fn push(message: String) -> Result<(), std::io::Error> {
        let input: Vec<&str> = message.split_ascii_whitespace().collect();
        let current = CurrentRepository::read()?;
        let repo_to_push =
            Remote::get_path_of_repo_remote(&current, input[2].trim_end_matches('\n'))?;
        let _ = Client::client(message, Path::new(&repo_to_push));
        Ok(())
    }

    /// Esta funcion handlea lo que respecta al comando remote
    pub fn remote(option: RemoteOption) -> Result<String, std::io::Error> {
        let current = CurrentRepository::read()?;
        Remote::remote(&current, option)
    }

    /// Esta funcion handlea lo que respecta al comando rebase
    pub fn rebase(branch: &str) -> Result<(), std::io::Error> {
        Rebase::rebase(branch)
    }
}
