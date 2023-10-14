#[cfg(test)]
mod tests {

    use std::{fs::File, collections::HashMap};

    use crate::tests_functions::{create_file, set_up, equals, create_dir};

    #[test]
    pub fn test_01_if_file_1_was_commited_then_rm_file_1_is_ok() -> Result<(), std::io::Error>{ 
        let (temp_dir, mut vcs) = set_up();
        let path = create_file(&temp_dir, "file1.txt");      

        vcs.local_repository.insert(path.to_string_lossy().to_string(), "content".to_string());
        
        let staging_area = vcs.rm(&path, vec!["rm".to_string()])?;
        assert_eq!(staging_area.len(), 1);
        Ok(())
    }

    #[test]
    pub fn test_02_if_file_1_was_commited_then_rm_file_1_is_ok() -> Result<(), std::io::Error> {
        let (temp_dir, mut vcs) = set_up();
        let path = create_file(&temp_dir, "file1.txt");      

        vcs.local_repository.insert(path.to_string_lossy().to_string(), "content".to_string());
        
        let staging_area = vcs.rm(&path,vec!["rm".to_string()])?;
        assert_eq!(equals(staging_area, &path, "DELETED"), true);
        Ok(())
    }

    #[test]
    pub fn test_03_if_file_1_was_commited_then_rm_file_1_is_ok() -> Result<(), std::io::Error> {
        let (temp_dir, mut vcs) = set_up();

        let file1_path = create_file(&temp_dir, "file1.txt");      
        let file2_path = create_file(&temp_dir, "file2.txt");      

        vcs.local_repository.insert(file1_path.to_string_lossy().to_string(), "content".to_string());
        vcs.local_repository.insert(file2_path.to_string_lossy().to_string(), "content".to_string());


        let _ = vcs.rm(&file1_path,vec!["rm".to_string()]);
        let staging_area = vcs.rm(&file2_path, vec!["rm".to_string()])?;

        assert_eq!(staging_area.len(), 2);
        Ok(())
    }

    #[test]
    pub fn test_04_try_rm_directory_without_r() -> Result<(), std::io::Error>{
        let (temp_dir, mut vcs) = set_up();
        let dir_path = create_dir(&temp_dir, "directory/");      

        vcs.local_repository.insert(dir_path.to_string_lossy().to_string(), "content".to_string());
        let result = vcs.rm(&dir_path, vec!["rm".to_string()]);
        
        assert!(matches!(result, Err(e) if e.to_string().contains("recursively without -r")));    
        Ok(())
    }

    #[test]
    pub fn test_05_rm_directory_without_r_staging_area_without_any_change() -> Result<(), std::io::Error>{ 
        let (temp_dir, mut vcs) = set_up();
        let dir_path = create_dir(&temp_dir, "directory/");      
        
        vcs.local_repository.insert(dir_path.to_string_lossy().to_string(), "content".to_string());
        let _ = vcs.rm(&dir_path, vec!["rm".to_string()]).unwrap_or(HashMap::new());        
        
        let staging_area = vcs.index.read_index_write_staging()?;

        assert_eq!(staging_area.len(), 0);    
        Ok(())
    }

    #[test]
    pub fn test_06_rm_directory_with_r() -> Result<(), std::io::Error>{ 
        let (temp_dir, mut vcs) = set_up();
        let dir_path = create_dir(&temp_dir, "carpeta/");      
        let file_path = dir_path.join("file.txt");
        let _ = File::create(&file_path);
    
        vcs.local_repository.insert(file_path.to_string_lossy().to_string(), "content".to_string());
        let staging_area = vcs.rm(&dir_path, vec!["rm -r".to_string()])?;
    
        assert_eq!(staging_area.len(), 1);    
        Ok(())
    }
    
    
    #[test]
    pub fn test_07_rm_remove_file_from_workspace() -> Result<(), std::io::Error>{ 
        let (temp_dir, mut vcs) = set_up();
        let path = create_file(&temp_dir, "file1.txt");      

        vcs.local_repository.insert(path.to_string_lossy().to_string(), "content".to_string());
        
        let _ = vcs.rm(&path, vec!["rm".to_string()]);
        assert!(!path.exists());
        Ok(())

    }

    #[test]
    pub fn test_08_rm_remove_only_file_that_was_commited_in_repository() -> Result<(), std::io::Error>{ 
        let (temp_dir, mut vcs) = set_up();
        let dir_path = create_dir(&temp_dir, "carpeta/");   

        let file_path = dir_path.join("file.txt");
        let file_path2 = dir_path.join("file2.txt");

        let _ = File::create(&file_path);
        let _ = File::create(&file_path2);

        vcs.local_repository.insert(file_path.to_string_lossy().to_string(), "content".to_string());
        let staging_area = vcs.rm(&dir_path, vec!["rm -r".to_string()])?;
    
        assert_eq!(staging_area.len(), 1);
        Ok(())

    }

    #[test]
    pub fn test_09_rm_remove_two_files_that_were_commited_in_repository() -> Result<(), std::io::Error>{ 
        let (temp_dir, mut vcs) = set_up();
        let dir_path = create_dir(&temp_dir, "carpeta/");      
        
        let file_path = dir_path.join("file.txt");
        let file_path2 = dir_path.join("file2.txt");
        
        let _ = File::create(&file_path);
        let _ = File::create(&file_path2);

        vcs.local_repository.insert(file_path.to_string_lossy().to_string(), "content".to_string());
        vcs.local_repository.insert(file_path2.to_string_lossy().to_string(), "content".to_string());
        
        let staging_area = vcs.rm(&dir_path, vec!["rm -r".to_string()])?;
    
        assert_eq!(staging_area.len(), 2);
        Ok(())
    }

    }