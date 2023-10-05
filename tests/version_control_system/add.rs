#[cfg(test)]
mod tests {
    use crate::tests_functions::{create_file, set_up, equals};

    #[test]
    pub fn test_01_add_file_1_to_staging_area() { // git add file1.txt
        let (temp_dir, mut vsc) = set_up();
        let path = create_file(&temp_dir, "file1.txt");        

        let staging_area = vsc.add(&path);
        assert!(matches!(staging_area, Ok(staging_area) if staging_area.len() == 1))
    }

    #[test]
    pub fn test_02_add_file_1_and_file_2_to_staging_area() { // git add file1.txt y despues git add file2.txt
        let (temp_dir, mut vsc) = set_up();
        let file1_path = create_file(&temp_dir, "file1.txt");   
        let file2_path = create_file(&temp_dir, "file2.txt");        
     
        let _ = vsc.add(&file1_path);
        let staging_area = vsc.add(&file2_path);

        assert!(matches!(staging_area, Ok(staging_area) if staging_area.len() == 2))
    }

    #[test]
    pub fn test_03_add_file_1_the_status_is_created() { // git add file1.txt
        let (temp_dir, mut vsc) = set_up();
        let path = create_file(&temp_dir, "file1.txt");      

        let staging_area = vsc.add(&path);
        assert!(matches!(staging_area, Ok(staging_area) if equals(staging_area.clone(), &path, "CREATED")));
    }


    #[test]
    pub fn test_04_add_file_1_the_status_is_modified() { // git add file1.txt
        let (temp_dir, mut vsc) = set_up();
        let path = create_file(&temp_dir, "file1.txt");      

        vsc.local_repository.insert(path.display().to_string(), "content".to_string());  

        let staging_area = vsc.add(&path);
        assert!(matches!(staging_area, Ok(staging_area) if equals(staging_area.clone(), &path, "MODIFIED")));
    }


    #[test]
    pub fn test_05_if_file_1_is_in_staging_area_and_add_file_1_updated_staging_area() { // git add file1.txt y despues git add file1.txt
        let (temp_dir, mut vsc) = set_up();
        let path = create_file(&temp_dir, "file1.txt");      

        vsc.local_repository.insert(path.display().to_string(), "content".to_string());  

        let _ = vsc.add(&path);
        let staging_area = vsc.add(&path);

        assert!(matches!(staging_area, Ok(staging_area) if staging_area.len() == 1));
    }

    #[test]
    pub fn test_06_if_file_1_is_in_staging_area_and_add_file_1_updated_staging_area() { // git add file1.txt
        let (temp_dir, mut vsc) = set_up();
        let path = create_file(&temp_dir, "file1.txt");      

        let _ = vsc.add(&path);
        let staging_area = vsc.add(&path);

        assert!(matches!(staging_area, Ok(staging_area) if equals(staging_area.clone(), &path, "CREATED")));
    }

    #[test]
    pub fn test_07_add_all_files_to_staging_area() { // git add .
        let (temp_dir, mut vsc) = set_up();
        let _ = create_file(&temp_dir, "file1.txt");
        let _ = create_file(&temp_dir, "file2.txt");        
        let _ = create_file(&temp_dir, "file3.txt");     

        let staging_area = vsc.add(std::path::Path::new(&vsc.path.clone()));
        assert!(matches!(staging_area, Ok(staging_area) if staging_area.len() == 3))
    }
}
