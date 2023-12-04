use std::path::Path;
use crate::{vcs::{files::current_commit::CurrentCommit, entities::{tree_entity::TreeEntity, commit_entity::CommitEntity, entity::Entity}}, constants::constant::COMMIT_INIT_HASH};


pub struct LsTree;

impl LsTree {

    pub fn ls_tree(branch: &str, path: &Path) -> Result<Vec<String>, std::io::Error>{
        let mut information = Vec::new();
        Self::get_information_branch(branch, path, &mut information)
    } 

    pub fn get_information_branch(branch: &str, path: &Path, information: &mut Vec<String>) -> Result<Vec<String>, std::io::Error>{
        let commit_hash = CurrentCommit::read_for_branch(path, branch)?;
        if commit_hash != COMMIT_INIT_HASH{
            let commit = CommitEntity::read(path, &commit_hash)?;
            let entities = TreeEntity::read(path, commit.tree_hash.clone())?;
            Self::read_entities(entities, information, 0);
        }
        Ok(information.to_vec())
    }

    pub fn read_entities(content: Vec<Entity>, information: &mut Vec<String>, depth: usize) {
        for entity in content {
            match entity {
                Entity::Blob(blob) => {
                    information.push(format!(
                        "{} - {} {} {}",
                        "    ".repeat(depth),
                        blob.content_type,
                        blob.blob_hash,
                        blob.path
                    ));
                }
                Entity::Tree(tree) => {
                    information.push(format!(
                        "{} - {} {} {}",
                        "    ".repeat(depth),
                        tree.content_type,
                        tree.tree_hash,
                        tree.path
                    ));
                    Self::read_entities(tree.entities, information, depth + 3);
                }
            }
        }
    }
    
}