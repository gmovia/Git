#[cfg(test)]
mod tests {

    use std::fs;

    use crate::tests_functions::{create_file, set_up};
    use rust_git::{
        constants::constant::BLOB_CODE,
        vcs::{commands::hash_object::WriteOption, version_control_system::VersionControlSystem},
    };

    #[test]
    pub fn test_01_read_hash_of_file_1_with_cat_file() -> Result<(), std::io::Error> {
        let _temp_dir = set_up();
        let path = create_file(&_temp_dir, "file1.txt");
        let hash = VersionControlSystem::hash_object(&path, WriteOption::Write, BLOB_CODE)?;

        assert_eq!(
            VersionControlSystem::cat_file(&hash)?,
            fs::read_to_string(path)?
        );
        Ok(())
    }
    #[test]
    pub fn test_02_tried_to_read_content_with_cat_file_of_an_unexist_hash_object(
    ) -> Result<(), std::io::Error> {
        let _temp_dir = set_up();
        assert!(
            matches!(VersionControlSystem::cat_file("asdasd"), Err(e) if e.to_string().contains("No such file or directory"))
        );
        Ok(())
    }
}
