#[cfg(test)]
mod tests {

    use std::{fs::{File, self}, collections::HashMap};
    use rust_git::vcs::commands::rm::RemoveOption;

    use crate::tests_functions::{create_file, set_up, equals, create_dir};

    #[test]
    pub fn test_01_if_file_1_was_commited_then_rm_file_1_is_ok() -> Result<(), std::io::Error>{ 
        let (temp_dir, mut vcs) = set_up();
        let path = create_file(&temp_dir, "file1.txt");      

        vcs.add(&path)?;
        vcs.commit("first commit".to_string())?;
        
        let staging_area = vcs.rm(&path, RemoveOption::NoDirectory)?;
        assert_eq!(staging_area.len(), 1);
        Ok(())
    }

    #[test]
    pub fn test_02_if_file_1_was_commited_then_rm_file_1_is_ok() -> Result<(), std::io::Error> {
        let (temp_dir, mut vcs) = set_up();
        let path = create_file(&temp_dir, "file1.txt");      

        vcs.add(&path)?;
        vcs.commit("first_commit".to_string())?;
        
        let staging_area = vcs.rm(&path, RemoveOption::NoDirectory)?;
        assert_eq!(equals(staging_area, &path, "DELETED"), true);
        Ok(())
    }

    #[test]
    pub fn test_03_if_file_1_was_commited_then_rm_file_1_is_ok() -> Result<(), std::io::Error> {
        let (temp_dir, mut vcs) = set_up();

        let file1_path = create_file(&temp_dir, "file1.txt");      
        let file2_path = create_file(&temp_dir, "file2.txt");      

        vcs.add(&file1_path)?;
        vcs.add(&file2_path)?;
        vcs.commit("first commit".to_string())?;

        let _ = vcs.rm(&file1_path, RemoveOption::Directory);
        let staging_area = vcs.rm(&file2_path, RemoveOption::NoDirectory)?;

        assert_eq!(staging_area.len(), 2);
        Ok(())
    }

    #[test]
    pub fn test_04_try_rm_directory_without_r() -> Result<(), std::io::Error>{
        let (temp_dir, mut vcs) = set_up();
        let dir_path = create_dir(&temp_dir, "directory");      

        let file_path = dir_path.join("file.txt");
        let _ = File::create(&file_path);     
        
        vcs.add(&dir_path)?;
        vcs.commit("first_commit".to_string())?;
        let result = vcs.rm(&dir_path, RemoveOption::NoDirectory);
        
        assert!(matches!(result, Err(e) if e.to_string().contains("recursively without -r")));    
        Ok(())
    } 

    #[test]
    pub fn test_05_rm_directory_without_r_staging_area_without_any_change() -> Result<(), std::io::Error>{ 
        let (temp_dir, mut vcs) = set_up();
        let dir_path = create_dir(&temp_dir, "directory"); 

        let file_path = dir_path.join("file.txt");
        let _ = File::create(&file_path);     
        
        vcs.add(&dir_path)?;
        vcs.commit("first_commit".to_string())?;
        
        let _ = vcs.rm(&dir_path, RemoveOption::NoDirectory).unwrap_or(HashMap::new());        
        
        let staging_area = vcs.index.read_index()?;

        assert_eq!(staging_area.len(), 0);    
        Ok(())
    }

    #[test]
    pub fn test_06_rm_directory_with_r() -> Result<(), std::io::Error>{ 
        let (temp_dir, mut vcs) = set_up();
        let dir_path = create_dir(&temp_dir, "carpeta");      
        let file_path = dir_path.join("file.txt");
        let _ = File::create(&file_path);
    
        vcs.add(&dir_path)?;
        vcs.add(&file_path)?;
        vcs.commit("first_commit".to_string())?;
        let staging_area = vcs.rm(&dir_path, RemoveOption::Directory)?;
    
        assert_eq!(staging_area.len(), 1);    
        Ok(())
    }
    
    
    #[test]
    pub fn test_07_rm_remove_file_from_workspace() -> Result<(), std::io::Error>{ 
        let (temp_dir, mut vcs) = set_up();
        let path = create_file(&temp_dir, "file1.txt");      

        vcs.add(&path)?;
        vcs.commit("first_commit".to_string())?;
        
        let _ = vcs.rm(&path, RemoveOption::NoDirectory);
        assert!(!path.exists());
        Ok(())
    }

    #[test]
    pub fn test_08_rm_remove_only_file_that_was_commited_in_repository() -> Result<(), std::io::Error>{ 
        let (temp_dir, mut vcs) = set_up();
        let dir_path = create_dir(&temp_dir, "carpeta");   

        let file_path = dir_path.join("file.txt");
        let file_path2 = dir_path.join("file2.txt");

        let _ = File::create(&file_path);
        let _ = File::create(&file_path2);

        vcs.add(&file_path)?;
        vcs.commit("first_commit".to_string())?;
        let staging_area = vcs.rm(&dir_path, RemoveOption::Directory)?;
    
        assert_eq!(staging_area.len(), 1);
        Ok(())
    }

    #[test]
    pub fn test_09_rm_remove_two_files_that_were_commited_in_repository() -> Result<(), std::io::Error>{ 
        let (temp_dir, mut vcs) = set_up();
        let dir_path = create_dir(&temp_dir, "carpeta");      
        
        let file_path = dir_path.join("file.txt");
        let file_path2 = dir_path.join("file2.txt");
        
        let _ = File::create(&file_path);
        let _ = File::create(&file_path2);

        vcs.add(&file_path)?;
        vcs.add(&file_path2)?;
        vcs.commit("first_commit".to_string())?;
        let staging_area = vcs.rm(&dir_path, RemoveOption::Directory)?;
    
        assert_eq!(staging_area.len(), 2);
        Ok(())
    }

    #[test]
    pub fn test_10_rm_remove_one_file_commited_with_other_directory_with_files_that_not_are_commited() -> Result<(), std::io::Error>{ 
        let (temp_dir, mut vcs) = set_up();

        let dir_path = create_dir(&temp_dir, "carpeta");      
        let file_path = dir_path.join("file.txt");

        let dir_path2 = dir_path.join("subcarpeta");
        fs::create_dir(&dir_path2)?;
        let file_path2 = dir_path2.join("file2.txt");
        let file_path3 = dir_path2.join("file3.txt");
        
        let _ = File::create(&file_path);
        let _ = File::create(&file_path2);
        let _ = File::create(&file_path3);

        vcs.add(&file_path)?;
        vcs.commit("first_commit".to_string())?;
        
        let staging_area = vcs.rm(&dir_path, RemoveOption::Directory)?;
    
        assert_eq!(staging_area.len(), 1);
        Ok(())
    }

    #[test]
    pub fn test_11_rm_remove_one_file_commited_with_other_directory_with_files_that_not_are_commited_and_exists_yet() -> Result<(), std::io::Error>{ 
        let (temp_dir, mut vcs) = set_up();

        let dir_path = create_dir(&temp_dir, "carpeta");      
        let file_path = dir_path.join("file.txt");

        let dir_path2 = dir_path.join("subcarpeta");
        fs::create_dir(&dir_path2)?;
        let file_path2 = dir_path2.join("file2.txt");
        let file_path3 = dir_path2.join("file3.txt");
        
        let _ = File::create(&file_path);
        let _ = File::create(&file_path2);
        let _ = File::create(&file_path3);

        vcs.add(&file_path)?;
        vcs.commit("first_commit".to_string())?;
        
        let _ = vcs.rm(&dir_path, RemoveOption::Directory)?;
    
        assert!(dir_path2.is_dir());
        assert!(file_path2.is_file());
        assert!(file_path3.is_file());
        Ok(())
    }

    #[test]
    pub fn test_12_rm_remove_one_file_commited_with_subdir_that_one_file_was_commited() -> Result<(), std::io::Error>{ 
        let (temp_dir, mut vcs) = set_up();

        let dir_path = create_dir(&temp_dir, "carpeta");      
        let file_path = dir_path.join("file.txt");

        let dir_path2 = dir_path.join("subcarpeta");
        fs::create_dir(&dir_path2)?;
        let file_path2 = dir_path2.join("file2.txt");
        let file_path3 = dir_path2.join("file3.txt");
        
        let _ = File::create(&file_path);
        let _ = File::create(&file_path2);
        let _ = File::create(&file_path3);


        vcs.add(&file_path)?;
        vcs.add(&file_path3)?;
        vcs.commit("first_commit".to_string())?;

        let staging_area = vcs.rm(&dir_path, RemoveOption::Directory)?;
    
        assert_eq!(staging_area.len(), 2);
        assert!(file_path2.is_file());
        assert!(!file_path3.is_file());
        Ok(())
    }
}