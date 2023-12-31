use crate::{
    constants::constant::TREE_CODE,
    utils::randoms::random::Random,
    vcs::{
        commands::{
            hash_object::{HashObject, WriteOption},
            init::Init,
        },
        entities::{
            blob_entity::BlobEntity, commit_entity::CommitEntity, entity::Entity,
            ref_delta_entity::RefDeltaEntity, tree_entity::TreeEntity,
        },
    },
};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

pub struct Proxy;

impl Proxy {
    /// Recibe una entidad commit y lo escribe
    pub fn write_commit(repo_path: &Path, commit: &CommitEntity) -> Result<String, std::io::Error> {
        CommitEntity::write(repo_path, commit)
    }

    /// Recibe el hash de un commit y obtiene su respectiva entidad
    pub fn read_commit(
        repo_path: &Path,
        commit_hash: String,
    ) -> Result<CommitEntity, std::io::Error> {
        CommitEntity::read(repo_path, &commit_hash)
    }

    /// Recibe el contenido de un tree y lo escribe
    pub fn write_tree(repo_path: &Path, content: &str) -> Result<String, std::io::Error> {
        let entity_strings: Vec<&str> = content.split('\n').filter(|&s| !s.is_empty()).collect();
        let tree_path = Path::new(&repo_path).join(Random::random());
        let mut tree_file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&tree_path)?;

        for entries in entity_strings {
            let parts: Vec<&str> = entries.split('-').collect();
            if parts[0] == "40000" {
                let content = format!("{} tree {} {}\n", parts[0], parts[2], parts[1]);
                tree_file.write_all(content.as_bytes())?;
            } else {
                let content = format!("{} blob {} {}\n", parts[0], parts[2], parts[1]);
                tree_file.write_all(content.as_bytes())?;
            }
        }

        let hash_tree = HashObject::hash_object(
            &tree_path,
            Init::get_object_path(repo_path)?,
            WriteOption::Write,
            TREE_CODE,
        )?;
        let _ = fs::remove_file(tree_path);

        Ok(hash_tree)
    }

    /// Recibe el hash de un tree y obtiene su respectiva entidad
    pub fn read_tree(repo_path: &Path, tree_hash: String) -> Result<Vec<Entity>, std::io::Error> {
        TreeEntity::read(repo_path, tree_hash)
    }

    /// Recibe el contenido de un blob y lo escribe
    pub fn write_blob(repo_path: &Path, content: &String) -> Result<String, std::io::Error> {
        BlobEntity::write(repo_path, content)
    }

    /// Recibe el hash de un blob y obtiene su respectiva entidad
    pub fn read_blob(repo_path: &Path, blob_hash: String) -> Result<String, std::io::Error> {
        BlobEntity::read(repo_path, blob_hash)
    }

    /// Recibe el contenido de un delta y un conjunto de blobs y lo escribe
    pub fn write_ref_delta(
        repo_path: &Path,
        content: RefDeltaEntity,
        blobs: &mut Vec<(u8, Vec<u8>)>,
    ) -> Result<Vec<(String, CommitEntity)>, std::io::Error> {
        if let Ok(entity) = RefDeltaEntity::write(repo_path, content, blobs) {
            Ok(entity)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Error parsing delta object",
            ))
        }
    }
}
