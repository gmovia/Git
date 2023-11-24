#[cfg(test)]
mod tests {

    use crate::tests_functions::count_files;
    use rust_git::vcs::{version_control_system::VersionControlSystem, commands::tag::TagOptions};
    use crate::tests_functions::{create_file, set_up};


    #[test]
    pub fn test_01_tag() -> Result<(), std::io::Error>{
        let temp_dir = set_up();
        
        let file_1 = create_file(&temp_dir, "file1.txt");
        VersionControlSystem::add(&file_1)?;
        VersionControlSystem::commit("first_commit".to_string())?;

        VersionControlSystem::tag(TagOptions::CreateLight("v1"))?;
        
        let tags_path = temp_dir.path().join(".rust_git").join("refs").join("tags");

        assert_eq!(count_files(&tags_path)?, 1);
        Ok(())
    }

    #[test]
    pub fn test_02_tag() -> Result<(), std::io::Error>{
        let temp_dir = set_up();
        
        let file_1 = create_file(&temp_dir, "file1.txt");
        VersionControlSystem::add(&file_1)?;
        VersionControlSystem::commit("first_commit".to_string())?;

        VersionControlSystem::tag(TagOptions::CreateLight("v1"))?;

        let file_2 = create_file(&temp_dir, "file2.txt");
        VersionControlSystem::add(&file_2)?;
        VersionControlSystem::commit("second".to_string())?;

        VersionControlSystem::tag(TagOptions::CreateLight("v2"))?;
        
        let tags_path = temp_dir.path().join(".rust_git").join("refs").join("tags");

        assert_eq!(count_files(&tags_path)?, 2);
        Ok(())
    }

    #[test]
    pub fn test_03_tag() -> Result<(), std::io::Error>{
        let temp_dir = set_up();
        
        let file_1 = create_file(&temp_dir, "file1.txt");
        VersionControlSystem::add(&file_1)?;
        VersionControlSystem::commit("first_commit".to_string())?;

        VersionControlSystem::tag(TagOptions::CreateLight("v1"))?;

        let file_2 = create_file(&temp_dir, "file2.txt");
        VersionControlSystem::add(&file_2)?;
        VersionControlSystem::commit("second".to_string())?;

        VersionControlSystem::tag(TagOptions::CreateLight("v2"))?;
        
        let tags = VersionControlSystem::tag(TagOptions::Get)?;
        assert_eq!(tags.len(), 2);
        Ok(())
    }

    #[test]
    pub fn test_04_tag() -> Result<(), std::io::Error>{
        let temp_dir = set_up();
        
        let file_1 = create_file(&temp_dir, "file1.txt");
        VersionControlSystem::add(&file_1)?;
        VersionControlSystem::commit("first_commit".to_string())?;

        VersionControlSystem::tag(TagOptions::CreateLight("v1"))?;

        let file_2 = create_file(&temp_dir, "file2.txt");
        VersionControlSystem::add(&file_2)?;
        VersionControlSystem::commit("second".to_string())?;

        VersionControlSystem::tag(TagOptions::CreateLight("v2"))?;
        VersionControlSystem::tag(TagOptions::Delete("v1"))?;

        let tags = VersionControlSystem::tag(TagOptions::Get)?;
        assert_eq!(tags.len(), 1);
        Ok(())
    }
}