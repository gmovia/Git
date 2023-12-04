use crate::{
    constants::constant::{BLOB_CODE, END_OF_LINE, TREE_CODE},
    utils::randoms::random::Random,
    vcs::commands::{
        cat_file::CatFile,
        hash_object::{HashObject, WriteOption},
        init::Init,
    },
};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

use super::{blob_entity::BlobEntity, entity::Entity};

#[derive(Debug, Clone)]

pub struct TreeEntity {
    pub content_type: String,
    pub path: String,
    pub tree_hash: String,
    pub entities: Vec<Entity>,
}

impl TreeEntity {
    /// Recibe un conjunto de entidades (blobs y trees) y crea su correspondiente arbol
    pub fn write(repo_path: &Path, entities: &Vec<Entity>) -> Result<String, std::io::Error> {
        let tree_path = Path::new(&repo_path).join(Random::random());
        let mut tree_file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&tree_path)?;
        for entity in entities {
            match entity {
                Entity::Blob(blob) => {
                    if let Some(filename) = Path::new(&blob.path).file_name() {
                        let entry = format!(
                            "100644 {} {}    {}\n",
                            blob.content_type,
                            blob.blob_hash,
                            filename.to_string_lossy()
                        );
                        tree_file.write_all(entry.as_bytes())?;
                    }
                }
                Entity::Tree(tree) => {
                    let tree_hash = TreeEntity::write(repo_path, &tree.entities)?;
                    if let Some(filename) = Path::new(&tree.path).file_name() {
                        let entry = format!(
                            "40000 {} {}    {}\n",
                            tree.content_type,
                            tree_hash,
                            filename.to_string_lossy()
                        );
                        tree_file.write_all(entry.as_bytes())?;
                    }
                }
            }
        }
        let tree_hash = HashObject::hash_object(
            &tree_path,
            Init::get_object_path(repo_path)?,
            WriteOption::Write,
            TREE_CODE,
        )?;
        let _ = fs::remove_file(tree_path);
        Ok(tree_hash)
    }

    /// Recibe un hash y devuelve la entidad tree asociada
    pub fn read(repo_path: &Path, tree_hash: String) -> Result<Vec<Entity>, std::io::Error> {
        let mut entities: Vec<Entity> = Vec::new();
        let content = CatFile::cat_file(&tree_hash, Init::get_object_path(repo_path)?)?;
        let lines: Vec<&str> = content.split('\n').collect();

        for line in lines {
            if line != END_OF_LINE {
                let parts: Vec<&str> = line.split_whitespace().collect();

                if parts[1] == BLOB_CODE {
                    let blob = BlobEntity {
                        content_type: BLOB_CODE.to_string(),
                        path: parts[3].to_string(),
                        blob_hash: parts[2].to_string(),
                    };
                    entities.push(Entity::Blob(blob));
                }

                if parts[1] == TREE_CODE {
                    let tree = TreeEntity {
                        content_type: TREE_CODE.to_string(),
                        path: parts[3].to_string(),
                        tree_hash: parts[2].to_string(),
                        entities: Self::read(repo_path, parts[2].to_string())?,
                    };
                    entities.push(Entity::Tree(tree));
                }
            }
        }
        Ok(entities)
    }
}
