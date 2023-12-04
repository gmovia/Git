#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::tests_functions::{create_file, set_up};
    use rust_git::{
        constants::constant::BLOB_CODE,
        vcs::{commands::hash_object::WriteOption, version_control_system::VersionControlSystem},
    };

    #[test]
    pub fn test_01_calculate_hash_object_of_file_1() -> Result<(), std::io::Error> {
        let _temp_dir = set_up();
        let path = create_file(&_temp_dir, "file1.txt");
        assert_eq!(
            VersionControlSystem::hash_object(&path, WriteOption::NoWrite, BLOB_CODE)?,
            "e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"
        );
        Ok(())
    }

    #[test]
    pub fn test_02_calculate_hash_object_error() -> Result<(), std::io::Error> {
        let _temp_dir = set_up();
        let path = Path::new("is_not_exist.txt");
        let result = VersionControlSystem::hash_object(&path, WriteOption::NoWrite, BLOB_CODE);
        assert!(matches!(result, Err(e) if e.to_string().contains("No such file or directory")));
        Ok(())
    }

    #[test]
    pub fn test_03_calculate_hash_object_error() -> Result<(), std::io::Error> {
        let _temp_dir = set_up();
        let result =
            VersionControlSystem::hash_object(&_temp_dir.path(), WriteOption::NoWrite, BLOB_CODE);
        assert!(matches!(result, Err(e) if e.to_string().contains("The path is an directory")));
        Ok(())
    }
}
