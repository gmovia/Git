#[cfg(test)]
mod tests {
    use std::{fs::{File, self}, io::Write};

    use rust_git::vcs::{version_control_system::VersionControlSystem, commands::ls_files::LsFilesOptions};

    use crate::tests_functions::{create_file, set_up, create_dir};


    #[test]
    pub fn test_01_adding_3_files_then_ls_files_returns_3() -> Result<(), std::io::Error>{

        let temp_dir = set_up();
        
        let file_1 = create_file(&temp_dir, "file1.txt");
        let file_2 = create_file(&temp_dir, "file2.txt");
        let dir_path = create_dir(&temp_dir, "carpeta");      
        let file_3 = dir_path.join("file3.txt");
        let _ = File::create(&file_3);
        
        VersionControlSystem::add(&file_1)?;
        VersionControlSystem::add(&file_2)?;
        VersionControlSystem::add(&file_3)?;

        let files = VersionControlSystem::ls_files(LsFilesOptions::EverythingInVCS)?;
        assert_eq!(files.len(), 3);
        Ok(())
    }

    #[test]
    pub fn test_02_adding_3_files_and_commit_1_then_ls_files_returns_3() -> Result<(), std::io::Error>{

        let temp_dir = set_up();
        
        let file_1 = create_file(&temp_dir, "file1.txt");
        let file_2 = create_file(&temp_dir, "file2.txt");
        let dir_path = create_dir(&temp_dir, "carpeta");      
        let file_3 = dir_path.join("file3.txt");
        let _ = File::create(&file_3);
        
        VersionControlSystem::add(&file_1)?;
        VersionControlSystem::commit("first_commit".to_string())?;
        VersionControlSystem::add(&file_2)?;
        VersionControlSystem::add(&file_3)?;

        let files = VersionControlSystem::ls_files(LsFilesOptions::EverythingInVCS)?;
        assert_eq!(files.len(), 3);
        Ok(())
    }

    #[test]
    pub fn test_03_adding_3_commit_2_delete_1_then_ls_files_returns_3() -> Result<(), std::io::Error>{

        let temp_dir = set_up();

        let _file_0 = create_file(&temp_dir, "file.txt");
        let file_1 = create_file(&temp_dir, "file1.txt");
        let file_2 = create_file(&temp_dir, "file2.txt");
        let dir_path = create_dir(&temp_dir, "carpeta");      
        let file_3 = dir_path.join("file3.txt");
        let _ = File::create(&file_3);
        
        VersionControlSystem::add(&file_1)?;
        VersionControlSystem::add(&file_2)?;
        VersionControlSystem::commit("first_commit".to_string())?;
        VersionControlSystem::add(&file_3)?;

        fs::remove_file(&file_2)?;

        let files = VersionControlSystem::ls_files(LsFilesOptions::EverythingInVCS)?;
        assert_eq!(files.len(), 3);
        Ok(())
    }

    #[test]
    pub fn test_04_create_5_files_adding_3_then_ls_files_others_returns_2() -> Result<(), std::io::Error>{

        let temp_dir = set_up();

        let _file_0 = create_file(&temp_dir, "file.txt");
        let _file_1 = create_file(&temp_dir, "file1.txt");
        let file_2 = create_file(&temp_dir, "file2.txt");
        let dir_path = create_dir(&temp_dir, "carpeta");      
        let file_3 = dir_path.join("file3.txt");
        let file_4 = dir_path.join("file4.txt");
        let _ = File::create(&file_3);
        let _ = File::create(&file_4);
        
        VersionControlSystem::add(&file_2)?;
        VersionControlSystem::add(&file_3)?;
        VersionControlSystem::add(&file_4)?;

        let files = VersionControlSystem::ls_files(LsFilesOptions::OnlyUntracked)?;
        assert_eq!(files.len(), 2);
        Ok(())
    }

    #[test]
    pub fn test_05_create_5_files_adding_3_then_ls_files_cached_returns_3() -> Result<(), std::io::Error>{

        let temp_dir = set_up();

        let _file_0 = create_file(&temp_dir, "file.txt");
        let _file_1 = create_file(&temp_dir, "file1.txt");
        let file_2 = create_file(&temp_dir, "file2.txt");
        let dir_path = create_dir(&temp_dir, "carpeta");      
        let file_3 = dir_path.join("file3.txt");
        let file_4 = dir_path.join("file4.txt");
        let _ = File::create(&file_3);
        let _ = File::create(&file_4);
        
        VersionControlSystem::add(&file_2)?;
        VersionControlSystem::add(&file_3)?;
        VersionControlSystem::add(&file_4)?;

        let files = VersionControlSystem::ls_files(LsFilesOptions::OnlyStaging)?;
        assert_eq!(files.len(), 3);
        Ok(())
    }

    #[test]
    pub fn test_06_create_7_files_adding_4_commit_3_delete_2_adding_1_deleted_then_ls_files_deleted_returns_1() -> Result<(), std::io::Error>{

        let temp_dir = set_up();

        let _file_0 = create_file(&temp_dir, "file.txt");
        let _file_1 = create_file(&temp_dir, "file1.txt");
        let file_2 = create_file(&temp_dir, "file2.txt");
        let dir_path = create_dir(&temp_dir, "carpeta");      
        let file_3 = dir_path.join("file3.txt");
        let file_4 = dir_path.join("file4.txt");
        let _ = File::create(&file_3);
        let _ = File::create(&file_4);
        let _file_5 = create_file(&temp_dir, "file5.txt");
        let file_6 = create_file(&temp_dir, "file6.txt");
        
        VersionControlSystem::add(&file_2)?;
        VersionControlSystem::add(&file_3)?;
        VersionControlSystem::add(&file_4)?;
        VersionControlSystem::commit("first_commit".to_string())?;
        VersionControlSystem::add(&file_6)?;

        fs::remove_file(&file_2)?;
        VersionControlSystem::add(&file_2)?;
        fs::remove_file(&file_3)?;

        let files = VersionControlSystem::ls_files(LsFilesOptions::OnlyDeleted)?;
        assert_eq!(files.len(), 1);
        Ok(())
    }

    #[test]
    pub fn test_07_create_7_files_adding_4_commit_3_modifie_1_delete_1_then_ls_files_modified_returns_2() -> Result<(), std::io::Error>{

        let temp_dir = set_up();

        let _file_0 = create_file(&temp_dir, "file.txt");
        let _file_1 = create_file(&temp_dir, "file1.txt");
        let file_2 = create_file(&temp_dir, "file2.txt");
        let dir_path = create_dir(&temp_dir, "carpeta");      
        let file_3 = dir_path.join("file3.txt");
        let file_4 = dir_path.join("file4.txt");
        let _ = File::create(&file_3);
        let _ = File::create(&file_4);
        let _file_5 = create_file(&temp_dir, "file5.txt");
        let file_6 = create_file(&temp_dir, "file6.txt");
        
        VersionControlSystem::add(&file_2)?;
        VersionControlSystem::add(&file_3)?;
        VersionControlSystem::add(&file_4)?;
        VersionControlSystem::commit("first_commit".to_string())?;
        VersionControlSystem::add(&file_6)?;

        fs::remove_file(&file_2)?;
        let mut file = fs::OpenOptions::new().write(true).create(true).append(true).open(&file_3)?;
        let _ = file.write_all(b"File 3, File 3, File 3");

        let files = VersionControlSystem::ls_files(LsFilesOptions::OnlyModified)?;
        assert_eq!(files.len(), 2);
        Ok(())
    }

}