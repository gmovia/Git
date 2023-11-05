#[cfg(test)]
mod tests {
    use std::{fs, io::Write};

    use rust_git::vcs::{files::{repository::Repository, index::Index}, version_control_system::VersionControlSystem};

    use crate::tests_functions::{create_file, set_up, equals};

    #[test]
    pub fn test_01_reset() -> Result<(), std::io::Error>{ // git add file1.txt
        let temp_dir = set_up();
        let path = create_file(&temp_dir, "file1.txt");        

        let staging_area = VersionControlSystem::add(&path)?;
        assert_eq!(staging_area.len(), 1);

        let staging_area = VersionControlSystem::reset(&path)?;
        assert_eq!(staging_area.len(), 0);
        Ok(())
    }
}