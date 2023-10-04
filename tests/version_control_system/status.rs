#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        path::{Path, PathBuf},
    };

    use rust_git::version_control_system::VersionControlSystem;

    pub fn status_contains(result: HashMap<String, String>, status: &str, file: &PathBuf) -> bool {
        if let Some(value) = result.get(&file.display().to_string()) {
            return value.as_str() == status;
        }
        false
    }

    #[test]
    pub fn test_01_status_says_create_6_files() {
        let version_control_system = VersionControlSystem::init("tests/utils/files".to_string());

        assert!(matches!(version_control_system.status(), Ok(result) if result.len()==4));
    }

    #[test]
    pub fn test_02_status_says_create_file_1() {
        let version_control_system = VersionControlSystem::init("tests/utils/files".to_string());

        let file_1 = Path::new("tests/utils/files").join("file1.txt");

        assert!(
            matches!(version_control_system.status(), Ok(result) if status_contains(result.clone(), "CREATED", &file_1))
        );
    }

    #[test]
    pub fn test_03_status_says_modified_file_1() {
        let mut version_control_system =
            VersionControlSystem::init("tests/utils/files".to_string());

        let file_1 = Path::new("tests/utils/files").join("file1.txt");

        version_control_system
            .local_repository
            .insert(file_1.display().to_string(), "".to_string());
        assert!(
            matches!(version_control_system.status(), Ok(result) if status_contains(result.clone(), "MODIFIED", &file_1))
        );
    }

    #[test]
    pub fn test_04_status_says_delete_file_3() {
        let mut version_control_system =
            VersionControlSystem::init("tests/utils/files".to_string());

        let file_3 = Path::new("tests/utils/files").join("/folder/file3.txt");

        version_control_system
            .local_repository
            .insert(file_3.display().to_string(), "File 3".to_string());

        assert!(
            matches!(version_control_system.status(), Ok(result) if status_contains(result.clone(), "DELETED", &file_3))
        );
    }

    #[test]
    pub fn test_05_status_says_delete_file_3_and_create_file_1() {
        let mut version_control_system =
            VersionControlSystem::init("tests/utils/files".to_string());

        let file_3 = Path::new("tests/utils/files").join("/folder/file3.txt");
        let file_1 = Path::new("tests/utils/files").join("file1.txt");

        version_control_system
            .local_repository
            .insert(file_3.display().to_string(), "File 3".to_string());

        assert!(
            matches!(version_control_system.status(), Ok(result) if status_contains(result.clone(), "DELETED", &file_3))
        );
        assert!(
            matches!(version_control_system.status(), Ok(result) if status_contains(result.clone(), "CREATED", &file_1))
        );
    }

    #[test]
    pub fn test_06_status_says_delete_file_3_modify_file_2_and_create_file_1() {
        let mut version_control_system =
            VersionControlSystem::init("tests/utils/files".to_string());

        let file_3 = Path::new("tests/utils/files").join("/folder/file3.txt");
        let file_1 = Path::new("tests/utils/files").join("file1.txt");
        let file_2 = Path::new("tests/utils/files").join("file2.txt");

        version_control_system
            .local_repository
            .insert(file_3.display().to_string(), "File 3".to_string());
        version_control_system.local_repository.insert(
            file_2.display().to_string(),
            "File 2, File 2, File 2".to_string(),
        );

        assert!(
            matches!(version_control_system.status(), Ok(result) if status_contains(result.clone(), "DELETED", &file_3))
        );
        assert!(
            matches!(version_control_system.status(), Ok(result) if status_contains(result.clone(), "CREATED", &file_1))
        );
        assert!(
            matches!(version_control_system.status(), Ok(result) if status_contains(result.clone(), "MODIFIED", &file_2))
        );
    }
}
