use super::{blob_entity::BlobEntity, tree_entity::TreeEntity};
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};
#[derive(Debug, Clone)]

pub enum Entity {
    Blob(BlobEntity),
    Tree(TreeEntity),
}

pub fn convert_to_repository(entities: &Vec<Entity>, path: PathBuf) -> HashMap<String, String> {
    let mut local_repository: HashMap<String, String> = HashMap::new();
    for entity in entities {
        match entity {
            Entity::Blob(blob) => {
                let file_path = path.join(blob.path.clone()).display().to_string();
                local_repository.insert(file_path, blob.blob_hash.clone());
            }
            Entity::Tree(tree) => {
                let folder_path = path.join(tree.path.clone());
                local_repository.extend(convert_to_repository(&tree.entities, folder_path));
            }
        }
    }
    local_repository
}

pub fn convert_to_entities(hash_map: &HashMap<String, String>, prefix: &str) -> Vec<Entity> {
    let mut entities = Vec::new();
    let mut processed_paths = HashSet::new();

    for (key, value) in hash_map.iter() {
        if processed_paths.contains(key) {
            continue;
        }

        if let Some(rest) = key.strip_prefix(prefix) {
            let path_parts: Vec<&str> = rest.split('/').collect();

            if path_parts.len() == 1 {
                let entity = BlobEntity {
                    content_type: "blob".to_string(),
                    path: key.to_string(),
                    blob_hash: value.to_string(),
                };
                entities.push(Entity::Blob(entity));
                processed_paths.insert(key.clone());
            } else {
                let common_prefix = format!("{}{}/", prefix, path_parts[0]);
                let subtree: HashMap<String, String> = hash_map
                    .iter()
                    .filter(|(k, _)| k.starts_with(&common_prefix))
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect();

                let tree_entities = convert_to_entities(&subtree, &common_prefix);

                let tree = TreeEntity {
                    content_type: "tree".to_string(),
                    path: format!("{}{}", prefix, path_parts[0]),
                    entities: tree_entities,
                    tree_hash: "".to_string(),
                };

                entities.push(Entity::Tree(tree));
                processed_paths.extend(subtree.keys().cloned());
            }
        }
    }

    entities
}
