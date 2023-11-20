use std::path::PathBuf;

use crate::{vcs::{files::current_commit::CurrentCommit, entities::{tree_entity::TreeEntity, commit_entity::CommitEntity, entity::Entity}}, constants::constants::COMMIT_INIT_HASH};




pub struct LsTree;

// pub enum LsTreeOptions {
//     TreeBranch,
//     TreeBranchDirectory,
// }

impl LsTree {

    pub fn ls_tree(branch: &str, path: &PathBuf) -> Result<Vec<String>, std::io::Error>{
        let mut information = Vec::new();
        Ok(Self::get_information_branch(branch, path, &mut information)?)
        // match option {
        //     LsTreeOptions::TreeBranch => {Ok(Self::get_information_branch(branch, path, &mut information)?)},
        //     LsTreeOptions::TreeBranchDirectory => todo!(),
        // }
    } 

    /// RECIBE UNA BRANCH Y DEVUELVE INFORMACION SOBRE SU ARBOL TREE
    pub fn get_information_branch(branch: &str, path: &PathBuf, information: &mut Vec<String>) -> Result<Vec<String>, std::io::Error>{
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