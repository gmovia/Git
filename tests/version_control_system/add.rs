#[cfg(test)]
mod tests {
    use rust_git::version_control_system::VersionControlSystem;
    use std::path::Path;    

    #[test]
    pub fn test_01_add_file_1_to_staging_area() {
        let mut version_control_system =
            VersionControlSystem::init("tests/utils/files".to_string());
        let path = Path::new("tests/utils/files/file1.txt");

        let _ = version_control_system.add(path);
        assert_eq!(version_control_system.staging_area.area.len(), 1);
    }

    #[test]
    pub fn test_02_add_files_to_staging_area() {
        let mut version_control_system =
            VersionControlSystem::init("tests/utils/files".to_string());
        let path = Path::new("tests/utils/files");

        let _ = version_control_system.add(path);
        assert_eq!(version_control_system.staging_area.area.len(), 4);
    }

    // Ahora si, ver como contemplar el caso del "." o el caso de que mandes mas de un archivo
}
