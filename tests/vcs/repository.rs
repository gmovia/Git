
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use rust_git::vcs::entities::blob_entity::BlobEntity;
    use rust_git::vcs::entities::entity::{Entity, convert_to_repository, convert_to_entities};
    use rust_git::vcs::entities::tree_entity::TreeEntity;
    use rust_git::vcs::files::current_repository::CurrentRepository;

    fn count_blobs(entities: &Vec<Entity>) -> i32{
        let mut index = 0;
        for entity in entities{
            match entity{
                Entity::Blob(_) => { index += 1; },
                Entity::Tree(tree) => { index += count_blobs(&tree.entities); }
            }
        }
        index
    }
    
    fn count_tree(entities: &Vec<Entity>) -> i32{
        let mut index = 0;
        for entity in entities{
            match entity{
                Entity::Blob(_) => {},
                Entity::Tree(tree) => { 
                    index += 1;
                    index += count_tree(&tree.entities); 
                }
            }
        }
        index
    }

    #[test]
    pub fn test_01_convert_entity_to_repository() -> Result<(), std::io::Error>{
        let blob_1 = BlobEntity{content_type: "blob".to_string(), path: "file1.txt".to_string(), blob_hash: "0x1279".to_string()};
        let blob_2 = BlobEntity{content_type: "blob".to_string(), path: "file2.txt".to_string(), blob_hash: "0x1288".to_string()};

        let mut entities: Vec<Entity> = Vec::new();
        
        entities.push(Entity::Blob(blob_1));
        entities.push(Entity::Blob(blob_2));

        let repository = convert_to_repository(&entities, CurrentRepository::read()?);

        assert_eq!(repository.len(), 2);
        Ok(())
    }


    #[test]
    pub fn test_02_convert_entity_to_repository() -> Result<(), std::io::Error>{
        let blob_1 = BlobEntity{content_type: "blob".to_string(), path: "file1.txt".to_string(), blob_hash: "0x1279".to_string()};
        let blob_2 = BlobEntity{content_type: "blob".to_string(), path: "file2.txt".to_string(), blob_hash: "0x1288".to_string()};

        let mut tree_entities: Vec<Entity> = Vec::new();
        tree_entities.push(Entity::Blob(blob_1));
        tree_entities.push(Entity::Blob(blob_2));

        let tree = TreeEntity{content_type: "tree".to_string(), path: "carpeta".to_string(), tree_hash: "".to_string(), entities: tree_entities};
        
        let mut entities: Vec<Entity> = Vec::new();
        entities.push(Entity::Tree(tree));

        let repository = convert_to_repository(&entities, CurrentRepository::read()?);

        assert_eq!(repository.len(), 2);
        Ok(())
    }

    #[test]
    pub fn test_03_convert_entity_to_repository() -> Result<(), std::io::Error>{
        let blob_1 = BlobEntity{content_type: "blob".to_string(), path: "file1.txt".to_string(), blob_hash: "0x1279".to_string()};
        let blob_2 = BlobEntity{content_type: "blob".to_string(), path: "file2.txt".to_string(), blob_hash: "0x1288".to_string()};
        let blob_3 = BlobEntity{content_type: "blob".to_string(), path: "file3.txt".to_string(), blob_hash: "0x1290".to_string()};

        let mut tree_entities: Vec<Entity> = Vec::new();
        tree_entities.push(Entity::Blob(blob_1));
        tree_entities.push(Entity::Blob(blob_2));

        let tree = TreeEntity{content_type: "tree".to_string(), path: "carpeta".to_string(), tree_hash: "".to_string(), entities: tree_entities};
        
        let mut entities: Vec<Entity> = Vec::new();
        entities.push(Entity::Tree(tree));
        entities.push(Entity::Blob(blob_3));

        let repository = convert_to_repository(&entities, CurrentRepository::read()?);

        assert_eq!(repository.len(), 3);
        Ok(())
    }

    #[test]
    pub fn test_04_convert_entity_to_repository() -> Result<(), std::io::Error>{
        let blob_1 = BlobEntity{content_type: "blob".to_string(), path: "file1.txt".to_string(), blob_hash: "0x1279".to_string()};
        let blob_2 = BlobEntity{content_type: "blob".to_string(), path: "file2.txt".to_string(), blob_hash: "0x1288".to_string()};
        let blob_3 = BlobEntity{content_type: "blob".to_string(), path: "file3.txt".to_string(), blob_hash: "0x1290".to_string()};
        let blob_4 = BlobEntity{content_type: "blob".to_string(), path: "file4.txt".to_string(), blob_hash: "0x1290".to_string()};

        let mut tree_entities: Vec<Entity> = Vec::new();
        tree_entities.push(Entity::Blob(blob_1));
        tree_entities.push(Entity::Blob(blob_2));

        let tree = TreeEntity{content_type: "tree".to_string(), path: "carpeta".to_string(), tree_hash: "".to_string(), entities: tree_entities};
        
        let mut entities: Vec<Entity> = Vec::new();
        entities.push(Entity::Tree(tree));
        entities.push(Entity::Blob(blob_3));

        let mut tree_entities: Vec<Entity> = Vec::new();
        tree_entities.push(Entity::Blob(blob_4));

        let tree = TreeEntity{content_type: "tree".to_string(), path: "carpeta2".to_string(), tree_hash: "".to_string(), entities: tree_entities};

        entities.push(Entity::Tree(tree));

        let repository = convert_to_repository(&entities, CurrentRepository::read()?);

        assert_eq!(repository.len(), 4);
        Ok(())
    }

    #[test]
    pub fn test_05_convert_repository_to_entities() -> Result<(), std::io::Error>{
        let mut repository: HashMap<String, String> = HashMap::new();

        repository.insert("file1.txt".to_string(), "0x001".to_string());
        repository.insert("file2.txt".to_string(), "0x002".to_string());

        let entities = convert_to_entities(&mut repository, "");
        assert_eq!(entities.len(), 2);

        assert_eq!(count_blobs(&entities), 2);
        assert_eq!(count_tree(&entities), 0);

        Ok(())
    }

    #[test]
    pub fn test_06_convert_repository_to_entities() -> Result<(), std::io::Error>{
        let mut repository: HashMap<String, String> = HashMap::new();

        repository.insert("file1.txt".to_string(), "0x001".to_string());
        repository.insert("file2.txt".to_string(), "0x002".to_string());
        repository.insert("carpeta/file3.txt".to_string(), "0x003".to_string());

        let entities = convert_to_entities(&mut repository, "");
        assert_eq!(entities.len(), 3);

        assert_eq!(count_blobs(&entities), 3);
        assert_eq!(count_tree(&entities), 1);

        Ok(())
    }

    #[test]
    pub fn test_07_convert_repository_to_entities() -> Result<(), std::io::Error>{
        let mut repository: HashMap<String, String> = HashMap::new();

        repository.insert("file1.txt".to_string(), "0x001".to_string());
        repository.insert("file2.txt".to_string(), "0x002".to_string());
        repository.insert("carpeta/folder/file3.txt".to_string(), "0x003".to_string());
        repository.insert("carpeta/file4.txt".to_string(), "0x003".to_string());

        let entities = convert_to_entities(&mut repository, "");

        assert_eq!(entities.len(), 3);
        assert_eq!(count_blobs(&entities), 4);
        assert_eq!(count_tree(&entities), 2);

        Ok(())
    }
}
