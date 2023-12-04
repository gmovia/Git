#[cfg(test)]
mod tests {

    use rust_git::vcs::version_control_system::VersionControlSystem;

    use crate::tests_functions::{create_file, set_up};

    #[test]
    pub fn test_01_reset() -> Result<(), std::io::Error> {
        let temp_dir = set_up();
        let path = create_file(&temp_dir, "file1.txt");

        let staging_area = VersionControlSystem::add(&path)?;
        assert_eq!(staging_area.len(), 1);

        let staging_area = VersionControlSystem::reset(&path)?;
        assert_eq!(staging_area.len(), 0);
        Ok(())
    }
}
