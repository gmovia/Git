#[cfg(test)]
mod tests {
    use crate::tests_functions::{create_file, set_up, equals};

    #[test]
    pub fn test_01_add_file_1_to_staging_area() -> Result<(), std::io::Error>{ // git add file1.txt
        let (temp_dir, mut vcs) = set_up();
        let path = create_file(&temp_dir, "file1.txt");        

        let staging_area = vcs.add(&path)?;
        assert_eq!(staging_area.len(), 1);
        Ok(())
    }

    #[test]
    pub fn test_02_add_file_1_and_file_2_to_staging_area() -> Result<(), std::io::Error>{ // git add file1.txt y despues git add file2.txt
        let (temp_dir, mut vcs) = set_up();
        let file1_path = create_file(&temp_dir, "file1.txt");   
        let file2_path = create_file(&temp_dir, "file2.txt");        
     
        let _ = vcs.add(&file1_path);
        let staging_area = vcs.add(&file2_path)?;

        assert_eq!(staging_area.len(), 2);
        Ok(())
    }

    #[test]
    pub fn test_03_add_file_1_the_status_is_created() -> Result<(), std::io::Error>{ // git add file1.txt
        let (temp_dir, mut vcs) = set_up();
        let path = create_file(&temp_dir, "file1.txt");      

        let staging_area = vcs.add(&path)?;
        assert_eq!(equals(staging_area.clone(), &path, "CREATED"),true);
        Ok(())
    }


    #[test]
    pub fn test_04_add_file_1_the_status_is_modified() -> Result<(), std::io::Error>{ // git add file1.txt
        let (temp_dir, mut vcs) = set_up();
        let path = create_file(&temp_dir, "file1.txt");      

        vcs.local_repository.insert(path.display().to_string(), "content".to_string());  

        let staging_area = vcs.add(&path)?;
        assert_eq!(equals(staging_area.clone(), &path, "MODIFIED"),true);
        Ok(())
    }


    #[test]
    pub fn test_05_if_file_1_is_in_staging_area_and_add_file_1_updated_staging_area() -> Result<(), std::io::Error>{ // git add file1.txt y despues git add file1.txt
        let (temp_dir, mut vcs) = set_up();
        let path = create_file(&temp_dir, "file1.txt");      

        vcs.local_repository.insert(path.display().to_string(), "content".to_string());  

        let _ = vcs.add(&path);
        let staging_area = vcs.add(&path)?;

        assert_eq!(staging_area.len(), 1);
        Ok(())
    }

    #[test]
    pub fn test_06_if_file_1_is_in_staging_area_and_add_file_1_updated_staging_area() -> Result<(), std::io::Error>{ // git add file1.txt
        let (temp_dir, mut vcs) = set_up();
        let path = create_file(&temp_dir, "file1.txt");      

        let _ = vcs.add(&path);
        let staging_area = vcs.add(&path)?;

        assert_eq!(equals(staging_area.clone(), &path, "CREATED"),true);
        Ok(())
    }

    #[test]
    pub fn test_07_add_all_files_to_staging_area() -> Result<(), std::io::Error>{ // git add .
        let (temp_dir, mut vcs) = set_up();
        let _ = create_file(&temp_dir, "file1.txt");
        let _ = create_file(&temp_dir, "file2.txt");        
        let _ = create_file(&temp_dir, "file3.txt");     

        let staging_area = vcs.add(std::path::Path::new(&vcs.path.clone()))?;
        assert_eq!(staging_area.len(), 3);
        Ok(())
    }

     #[test] 
    pub fn test_08_read_index_and_write_it_to_staging_area() -> Result<(),std::io::Error>{
        let (temp_dir, mut vcs) = set_up();
        let _ = create_file(&temp_dir, "file1.txt");
        let _ = create_file(&temp_dir, "file2.txt");   

        let staging_area = vcs.add(std::path::Path::new(&vcs.path.clone()))?;
        let staging = vcs.index.read_index_write_staging()?;
        for (key, value) in &staging{
            assert_eq!(true,staging_area.contains_key(key));
            assert_eq!(Some(value), staging_area.get(key));
        }
        Ok(())
    }
}