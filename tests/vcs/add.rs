#[cfg(test)]
mod tests {
    use std::{fs, io::Write};

    use rust_git::vcs::{files::{repository::Repository, index::Index, current_repository::CurrentRepository}, version_control_system::VersionControlSystem};

    use crate::tests_functions::{create_file, set_up, equals};

    #[test]
    pub fn test_01_add_file_1_to_staging_area() -> Result<(), std::io::Error>{
        let temp_dir = set_up();
        let path = create_file(&temp_dir, "file1.txt");        

        let staging_area = VersionControlSystem::add(&path)?;
        assert_eq!(staging_area.len(), 1);
        Ok(())
    }

    #[test]
    pub fn test_02_add_file_1_and_file_2_to_staging_area() -> Result<(), std::io::Error>{
        let temp_dir = set_up();
        let file1_path = create_file(&temp_dir, "file1.txt");   
        let file2_path = create_file(&temp_dir, "file2.txt");        
     
        VersionControlSystem::add(&file1_path)?;
        let staging_area = VersionControlSystem::add(&file2_path)?;
        assert_eq!(staging_area.len(), 2);
        Ok(())
    }

    #[test]
    pub fn test_03_add_file_1_the_status_is_created() -> Result<(), std::io::Error>{
        let temp_dir = set_up();
        let path = create_file(&temp_dir, "file1.txt");      

        let staging_area = VersionControlSystem::add(&path)?;
        assert_eq!(equals(staging_area.clone(), &path, "CREATED"),true);
        Ok(())
    }


    #[test]
    pub fn test_04_add_file_1_the_status_is_modified() -> Result<(), std::io::Error>{
        let temp_dir = set_up();
        let path = create_file(&temp_dir, "file1.txt");      

        let _ = VersionControlSystem::add(&path)?;
        let _ = VersionControlSystem::commit("first commit".to_string());

        let mut file = fs::OpenOptions::new().create(true).append(true).open(&path)?;
        let _ = file.write_all(b"hola");

        let staging_area = VersionControlSystem::add(&path)?;

        assert_eq!(equals(staging_area.clone(), &path, "MODIFIED"),true);
        Ok(())
    }


    #[test]
    pub fn test_05_if_file_1_is_in_staging_area_and_add_file_1_updated_staging_area() -> Result<(), std::io::Error>{
        let temp_dir = set_up();
        let path = create_file(&temp_dir, "file1.txt");      

        Repository::read_repository()?.insert(path.display().to_string(), "content".to_string());  

        let _ = VersionControlSystem::add(&path);
        let staging_area = VersionControlSystem::add(&path)?;

        assert_eq!(staging_area.len(), 1);
        Ok(())
    }

    #[test]
    pub fn test_06_if_file_1_is_in_staging_area_and_add_file_1_updated_staging_area() -> Result<(), std::io::Error>{
        let temp_dir = set_up();
        let path = create_file(&temp_dir, "file1.txt");      

        let _ = VersionControlSystem::add(&path);
        let staging_area = VersionControlSystem::add(&path)?;

        assert_eq!(equals(staging_area.clone(), &path, "CREATED"),true);
        Ok(())
    }

    #[test]
    pub fn test_07_add_all_files_to_staging_area() -> Result<(), std::io::Error>{
        let temp_dir = set_up();
        let _ = create_file(&temp_dir, "file1.txt");
        let _ = create_file(&temp_dir, "file2.txt");        
        let _ = create_file(&temp_dir, "file3.txt");     
        let current  = CurrentRepository::read()?;
        let staging_area = VersionControlSystem::add(&current)?;
        assert_eq!(staging_area.len(), 3);
        Ok(())
    }

     #[test] 
    pub fn test_08_read_index_and_write_it_to_staging_area() -> Result<(),std::io::Error>{
        let temp_dir = set_up();
        let _ = create_file(&temp_dir, "file1.txt");
        let _ = create_file(&temp_dir, "file2.txt");   

        let current  = CurrentRepository::read()?;
        let staging_area = VersionControlSystem::add(&current)?;
        let staging = Index::read_index()?;
        for (key, value) in &staging{
            assert_eq!(true,staging_area.contains_key(key));
            assert_eq!(Some(value), staging_area.get(key));
        }
        Ok(())
    }

    #[test]
    pub fn test_09_trying_to_commit_a_file_before_adding_it() -> Result<(),std::io::Error>{
        let temp_dir = set_up();
        let _ = create_file(&temp_dir, "file1.txt");

        let result = VersionControlSystem::commit("first_commit".to_string());
        assert!(result.is_err());
        Ok(())
    }
}