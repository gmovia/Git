use std::{path::{PathBuf, Path}, fs::{OpenOptions, self}, io::Write};
use crate::{vcs::commands::{hash_object::{HashObject, WriteOption}, init::Init, cat_file::CatFile}, constants::constants::{END_OF_LINE, BLOB_CODE, TREE_CODE}, utils::random::random::Random};

use super::{blob_entity::BlobEntity, entity::Entity};

#[derive(Debug, Clone)]

pub struct TreeEntity{
    pub content_type: String,
    pub path: String,
    pub tree_hash: String,
    pub entities: Vec<Entity>
}

impl TreeEntity{
    
    pub fn write(repo_path: &PathBuf, entities: &Vec<Entity>) -> Result<String, std::io::Error>{
        let tree_path = Path::new(&repo_path).join(Random::random());
        let mut tree_file = OpenOptions::new().write(true).create(true).append(true).open(&tree_path)?; 
        for entity in entities {
            match entity{
                Entity::Blob(blob) => {
                    let entry = format!("100644 {} {} {}\n", blob.content_type, blob.blob_hash, blob.path);
                    tree_file.write_all(entry.as_bytes())?;
                },
                Entity::Tree(tree) => {
                    let tree_hash = TreeEntity::write(repo_path, &tree.entities)?;
                    let entry = format!("40000 {} {} {}\n", tree.content_type, tree_hash, tree.path,);
                    tree_file.write_all(entry.as_bytes())?;
                }
            }
        }
        let tree_hash = HashObject::hash_object(&tree_path, Init::get_object_path(&repo_path)?, WriteOption::Write, TREE_CODE)?;
        let _ = fs::remove_file(tree_path);
        Ok(tree_hash)
    }
    
    pub fn read(repo_path: &PathBuf, tree_hash: String) -> Result<Vec<Entity>, std::io::Error>{
        let mut entities: Vec<Entity> = Vec::new();
        let content = CatFile::cat_file(&tree_hash, Init::get_object_path(&repo_path)?)?;
        let lines: Vec<&str> = content.split("\n").collect();

        for line in lines{
            if line != END_OF_LINE{
                let parts: Vec<&str> = line.split_whitespace().collect();

                if parts[1] == BLOB_CODE{
                    let blob = BlobEntity{content_type: BLOB_CODE.to_string(), path: parts[3].to_string(), blob_hash: parts[2].to_string()};
                    entities.push(Entity::Blob(blob));
                }

                if parts[1] == TREE_CODE{
                    let tree = TreeEntity{content_type: TREE_CODE.to_string(), path: parts[3].to_string(), tree_hash: parts[2].to_string(), entities: Self::read(repo_path, parts[2].to_string())?};
                    entities.push(Entity::Tree(tree));
                }
            }
        }
        Ok(entities)
    }
}
