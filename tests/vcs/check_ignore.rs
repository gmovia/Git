#[cfg(test)]
mod tests {

    use std::{fs::OpenOptions, io::Write};

    use crate::tests_functions::{create_dir, create_file, set_up};
    use rust_git::{
        constants::constant::{RESPONSE_NOK_IGNORE, RESPONSE_OK_IGNORE},
        vcs::version_control_system::VersionControlSystem,
    };

    #[test]
    pub fn test_01_git_ignore() -> Result<(), std::io::Error> {
        let temp_dir = set_up();
        let file_1 = create_file(&temp_dir, "file1.txt");

        let git_path = temp_dir.path().join(".gitignore");
        let mut git_file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&git_path)?;

        git_file.write_all("file1.txt".as_bytes())?;

        let ok = VersionControlSystem::check_ignore(&file_1)?;
        assert_eq!(ok, RESPONSE_OK_IGNORE);

        Ok(())
    }

    #[test]
    pub fn test_02_git_ignore() -> Result<(), std::io::Error> {
        let temp_dir = set_up();
        let file_1 = create_file(&temp_dir, "file2.txt");

        let git_path = temp_dir.path().join(".gitignore");
        let mut git_file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&git_path)?;

        git_file.write_all("file1.txt".as_bytes())?;

        let ok = VersionControlSystem::check_ignore(&file_1)?;
        assert_eq!(ok, RESPONSE_NOK_IGNORE);

        Ok(())
    }

    #[test]
    pub fn test_03_git_ignore() -> Result<(), std::io::Error> {
        let temp_dir = set_up();
        let _ = create_dir(&temp_dir, "folder");

        let file_1 = create_file(&temp_dir, "folder/file1.txt");

        let git_path = temp_dir.path().join(".gitignore");
        let mut git_file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&git_path)?;

        git_file.write_all("file1.txt".as_bytes())?;

        let ok = VersionControlSystem::check_ignore(&file_1)?;
        assert_eq!(ok, RESPONSE_NOK_IGNORE);

        Ok(())
    }

    #[test]
    pub fn test_04_git_ignore() -> Result<(), std::io::Error> {
        let temp_dir = set_up();
        let _ = create_dir(&temp_dir, "folder");

        let file_1 = create_file(&temp_dir, "folder/file1.txt");

        let git_path = temp_dir.path().join(".gitignore");
        let mut git_file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&git_path)?;

        git_file.write_all("folder/file1.txt".as_bytes())?;

        let ok = VersionControlSystem::check_ignore(&file_1)?;
        assert_eq!(ok, RESPONSE_OK_IGNORE);

        Ok(())
    }

    #[test]
    pub fn test_05_git_ignore() -> Result<(), std::io::Error> {
        let temp_dir = set_up();
        let _ = create_dir(&temp_dir, "folder");

        let file_1 = create_file(&temp_dir, "folder/file1.txt");
        let file_2 = create_file(&temp_dir, "file2.txt");
        let file_3 = create_file(&temp_dir, "file3.txt");

        let git_path = temp_dir.path().join(".gitignore");
        let mut git_file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&git_path)?;

        git_file.write_all("folder/\n".as_bytes())?;
        git_file.write_all("file2.txt".as_bytes())?;

        let ok = VersionControlSystem::check_ignore(&file_1)?;
        assert_eq!(ok, RESPONSE_OK_IGNORE);

        let ok = VersionControlSystem::check_ignore(&file_2)?;
        assert_eq!(ok, RESPONSE_OK_IGNORE);

        let ok = VersionControlSystem::check_ignore(&file_3)?;
        assert_eq!(ok, RESPONSE_NOK_IGNORE);

        Ok(())
    }
}
