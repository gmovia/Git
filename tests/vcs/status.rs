#[cfg(test)]
mod tests {
    use std::{fs, io::Write};
    use crate::tests_functions::{create_file, set_up, status_contains};

    #[test]
    pub fn test_01_untracked_files_contains_four_files() {
        let (temp_dir, vsc) = set_up();
        let _ = create_file(&temp_dir, "file1.txt");        
        let _ = create_file(&temp_dir, "file2.txt");        
        let _ = create_file(&temp_dir, "file3.txt");        
        let _ = create_file(&temp_dir, "file4.txt");        
        assert!(matches!(vsc.status(), Ok((untracked_files, _, _)) if untracked_files.len() == 4));
    }

    #[test]
    pub fn test_02_untracked_files_contains_one_file_with_status_created() {
        let (temp_dir, vsc) = set_up();
        let path = create_file(&temp_dir, "file1.txt");        
        assert!(matches!(vsc.status(), Ok((untracked_files, _, _)) if status_contains(untracked_files.clone(), "CREATED", &path)));
    }
    
    #[test]
    pub fn test_03_untracked_files_not_contains_files() -> Result<(), std::io::Error>{
        let (temp_dir, vcs) = set_up();
        let path = create_file(&temp_dir, "file1.txt");
        vcs.add(&path)?;
        vcs.commit("first_commit".to_string())?;

        let (untracked_files, _, _ ) = vcs.status()?;
        assert_eq!(untracked_files.len(), 0);
        Ok(())
    }

    #[test]
    pub fn test_04_changes_not_staged_for_commit_contains_one_file() -> Result<(), std::io::Error>{
        let (temp_dir, vcs) = set_up();
        let path = create_file(&temp_dir, "file1.txt");
        
        vcs.add(&path)?;
        let mut file = fs::OpenOptions::new().write(true).create(true).append(true).open(&path)?;
        let _ = file.write_all(b"contenido");
        
        vcs.commit("first_commit".to_string())?;

        assert!(matches!(vcs.status(), Ok((_, changes_not_staged_for_commit, _)) if status_contains(changes_not_staged_for_commit.clone(), "MODIFIED", &path)));
        assert!(matches!(vcs.status(), Ok((_, changes_not_staged_for_commit, _)) if changes_not_staged_for_commit.len() == 1));
        Ok(())
    }

    #[test]
    pub fn test_05_changes_not_staged_for_commit_contains_one_file()  -> Result<(), std::io::Error>{
        let (temp_dir, vcs) = set_up();
        let file = create_file(&temp_dir, "file.txt");

        vcs.add(&file)?;
        vcs.commit("first commit".to_string())?;
        
        fs::remove_file(&file)?;
        
        assert!(matches!(vcs.status(), Ok((_, changes_not_staged_for_commit, _)) if status_contains(changes_not_staged_for_commit.clone(), "DELETED", &file.to_path_buf())));
        assert!(matches!(vcs.status(), Ok((_, changes_not_staged_for_commit, _)) if changes_not_staged_for_commit.len() == 1));
        Ok(())
    }
    
    
    #[test]
    pub fn test_06_untracked_file_contain_file_1_and_changes_not_staged_for_commit_contain_file_3() -> Result<(), std::io::Error>{
        let (temp_dir, vcs) = set_up();
        
        let file_1 = create_file(&temp_dir, "file1.txt");
        let file_3 = create_file(&temp_dir, "file3.txt"); 
        //let file_3 = Path::new("file3.txt");
        
        vcs.add(&file_3)?;
        vcs.commit("file_3".to_string())?;
        
        fs::remove_file(&file_3)?;

        assert!(matches!(vcs.status(), Ok((_, changes_not_staged_for_commit, _)) if status_contains(changes_not_staged_for_commit.clone(), "DELETED", &file_3.to_path_buf())));
        assert!(matches!(vcs.status(), Ok((untracked_file, _, _)) if status_contains(untracked_file.clone(), "CREATED", &file_1)));
        Ok(())
    }
    
    #[test]
    pub fn test_07_untracked_file_contain_file_1_and_changes_not_staged_for_commit_contain_file_2_and_file_3() -> Result<(), std::io::Error>{
        let (temp_dir, vcs) = set_up();
        
        let file_1 = create_file(&temp_dir, "file1.txt");
        let file_2 = create_file(&temp_dir, "file2.txt");
        let file_3 = create_file(&temp_dir, "file3.txt");
        //let file_3 = Path::new("file3.txt");
        
        vcs.add(&file_2)?;
        let mut file = fs::OpenOptions::new().write(true).create(true).append(true).open(&file_2)?;
        let _ = file.write_all(b"File 2, File 2, File 2");
        vcs.add(&file_3)?;

        vcs.commit("first_commit".to_string())?;
        fs::remove_file(&file_3)?;
        
        assert!(matches!(vcs.status(), Ok((untracked_file, _, _)) if status_contains(untracked_file.clone(), "CREATED", &file_1)));
        assert!(matches!(vcs.status(), Ok((_, changes_not_staged_for_commit, _)) if status_contains(changes_not_staged_for_commit.clone(), "MODIFIED", &file_2)));
        assert!(matches!(vcs.status(), Ok((_, changes_not_staged_for_commit, _)) if status_contains(changes_not_staged_for_commit.clone(), "DELETED", &file_3.to_path_buf())));

        assert!(matches!(vcs.status(), Ok((untracked_file, _, _)) if untracked_file.len() == 1));
        assert!(matches!(vcs.status(), Ok((_, changes_not_staged_for_commit, _)) if changes_not_staged_for_commit.len() == 2));
        Ok(())
    }

    #[test]
    pub fn test_08_staging_area_contain_file_1() {
        let (temp_dir, vsc) = set_up();
        let path = create_file(&temp_dir, "file1.txt");    

        let _ = vsc.add(&path);
        assert!(matches!(vsc.status(), Ok((_, _, changes_to_be_commited)) if status_contains(changes_to_be_commited.clone(), "CREATED", &path)));
        assert!(matches!(vsc.status(), Ok((untracked_file, _, _)) if untracked_file.len() == 0));
    }


    #[test]
    pub fn test_09_staging_area_contain_file_1_and_changes_not_staged_for_commit_contain_file1() -> Result<(), std::io::Error>{
        let (temp_dir, vcs) = set_up();
        let path = create_file(&temp_dir, "file1.txt");    
        let mut file = fs::OpenOptions::new().write(true).create(true).append(true).open(&path)?;

        vcs.add(&path)?;
        vcs.commit("first_commit".to_string())?;
        let _ = file.write_all(b" ");
        vcs.add(&path)?;

        let _ = file.write_all(b"modified");
        
        let (untracked_files, changes_not_staged_for_commit, changes_to_be_commited) = vcs.status()?;

        assert_eq!(status_contains(changes_to_be_commited.clone(), "MODIFIED", &path), true);
        assert_eq!(untracked_files.len(), 0);
        assert_eq!(changes_not_staged_for_commit.len(), 1);
        Ok(())
    }
    
    
    #[test]
    pub fn test_10_changes_not_staged_for_commit_contain_file_1() -> Result<(), std::io::Error> {
        let (temp_dir, vcs) = set_up(); // SI SACAS TEMP_DIR ROMPE!
        let path = create_file(&temp_dir, "file1.txt");   
        
        vcs.add(&path)?;
        vcs.commit("first_commit".to_string())?;

        let mut file = fs::OpenOptions::new().write(true).create(true).append(true).open(&path)?;
        let _ = file.write_all(b"File 3");
        
        let (untracked_files, changes_not_staged_for_commit, _) = vcs.status()?;
        assert_eq!(untracked_files.len(), 0);
        assert_eq!(changes_not_staged_for_commit.len(), 1);
        Ok(())
    }
    
    
    #[test]
    pub fn test_11_changes_to_be_commited_contain_file_1() -> Result<(), std::io::Error>{
        let (temp_dir, vcs) = set_up();
        let path = create_file(&temp_dir, "file1.txt"); 
        
        vcs.add(&path)?;
        vcs.commit("first_commit".to_string())?;

        let mut file = fs::OpenOptions::new().write(true).create(true).append(true).open(&path)?;
        let _ = file.write_all(b"File 3");

        vcs.add(&path)?;

        let (_, changes_not_staged_for_commit, changes_to_be_commited) = vcs.status()?;
        assert_eq!(changes_to_be_commited.len(), 1);
        assert_eq!(changes_not_staged_for_commit.len(), 0);
        Ok(())
    }

    #[test]
    pub fn test_12_three_sets_are_empty() -> Result<(), std::io::Error>{
        let (temp_dir, vcs) = set_up();
        let path = create_file(&temp_dir, "file1.txt");    
        vcs.repository.read_repository()?.insert(path.display().to_string(), "".to_string());

        vcs.add(&path)?;
        vcs.commit("first_commit".to_string())?;

        vcs.add(&path)?;
        let (untracked_file, changes_not_staged_for_commit, changes_to_be_commited) = vcs.status()?;

        assert_eq!(untracked_file.len(), 0);
        assert_eq!(changes_to_be_commited.len(), 0);
        assert_eq!(changes_not_staged_for_commit.len(), 0);
        Ok(())
    }

}