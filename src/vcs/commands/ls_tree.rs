use std::path::PathBuf;

use crate::vcs::{files::commits_table::CommitsTable, entities::{tree_entity::TreeEntity, commit_entity::CommitEntity, entity::{Entity, convert_to_repository, convert_to_entities}}};




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
        let commits = CommitsTable::read(path.to_path_buf(), branch)?;
        if let Some(last_commit) = commits.last() {
            let commit = CommitEntity::read(path, &last_commit.hash.clone())?;
            let entities = TreeEntity::read(path, commit.tree_hash.clone())?;
            Self::read_entities(entities, information);
        }
        Ok(information.to_vec())
    }

    pub fn read_entities(content: Vec<Entity>, information: &mut Vec<String>) {
        for entity in content {
            match entity {
                Entity::Blob(blob) => {information.push(format!("{} {} {}",blob.content_type, blob.blob_hash, blob.path));},
                Entity::Tree(tree) => {information.push(format!("{} {} {}",tree.content_type, tree.tree_hash, tree.path));
                                                    Self::read_entities(tree.entities, information);
                                                    },
            }
        }
    }
}