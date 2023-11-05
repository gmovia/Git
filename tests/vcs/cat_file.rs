#[cfg(test)]
mod tests {

    use std::fs;

    use rust_git::vcs::{commands::hash_object::WriteOption, version_control_system::VersionControlSystem};
    use crate::tests_functions::{create_file, set_up};

    #[test]
    pub fn test_01_read_hash_of_file_1_with_cat_file() -> Result<(), std::io::Error> {
        let temp_dir = set_up();
        let path = create_file(&temp_dir, "file1.txt");  
        let hash = VersionControlSystem::hash_object(&path, WriteOption::Write)?;

        assert_eq!(VersionControlSystem::cat_file(&hash, ".rust_git")?,fs::read_to_string(path)?);
        Ok(())
    }
    #[test]
    pub fn test_02_tried_to_read_content_with_cat_file_of_an_unexist_hash_object() -> Result<(), std::io::Error> {
        let temp_dir = set_up();
        assert!(matches!(VersionControlSystem::cat_file("asdasd", ".rust_git"), Err(e) if e.to_string().contains("No such file or directory")));    
        Ok(())
    }

}