#[cfg(test)]
mod tests {
    use rust_git::vcs::version_control_system::VersionControlSystem;
    use crate::tests_functions::{create_file, set_up};

    #[test]
    pub fn test_01_calculate_hash_object_of_file_1() -> Result<(), std::io::Error> {
        let (temp_dir, _) = set_up();
        let path = create_file(&temp_dir, "file1.txt");
        
        assert_eq!(VersionControlSystem::hash_object(&path)?, "e69de29bb2d1d6434b8b29ae775ad8c2e48c5391");
        Ok(())
    }
    
}