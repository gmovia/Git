#[cfg(test)]
mod tests {
    use std::path::Path;
    use rust_git::version_control_system::VersionControlSystem;
    use crate::tests_functions::{create_file, set_up, status_contains};

    #[test]
    pub fn test_01_status_says_create_6_files() {
        let version_control_system = VersionControlSystem::init("tests/utils/files".to_string());
        assert!(matches!(version_control_system.status(), Ok(result) if result.len()==4));
    }

    #[test]
    pub fn test_02_status_says_create_file_1() {
        let (temp_dir, vsc) = set_up();
        let path = create_file(&temp_dir, "file1.txt");        

        assert!(matches!(vsc.status(), Ok(result) if status_contains(result.clone(), "CREATED", &path)));
    }

    #[test]
    pub fn test_03_status_says_modified_file_1() {
        let (temp_dir, mut vsc) = set_up();
        let path = create_file(&temp_dir, "file1.txt");

        vsc.local_repository.insert(path.display().to_string(), "contenido".to_string());
        
        assert!(matches!(vsc.status(), Ok(result) if status_contains(result.clone(), "MODIFIED", &path)));
    }

    #[test]
    pub fn test_04_status_says_delete_file_3() {
        let (_, mut vsc) = set_up();
        let path = Path::new("file3.txt");

        vsc.local_repository.insert("file3.txt".to_string(), "contenido".to_string());
        assert!(matches!(vsc.status(), Ok(result) if status_contains(result.clone(), "DELETED", &path.to_path_buf())));
    }

    #[test]
    pub fn test_05_status_says_delete_file_3_and_create_file_1() {
        let (temp_dir, mut vsc) = set_up();

        let file_1 = create_file(&temp_dir, "file1.txt");
        let file_3 = Path::new("file3.txt");

        vsc.local_repository.insert(file_3.display().to_string(), "File 3".to_string());

        assert!(matches!(vsc.status(), Ok(result) if status_contains(result.clone(), "DELETED", &file_3.to_path_buf())));
        assert!(matches!(vsc.status(), Ok(result) if status_contains(result.clone(), "CREATED", &file_1)));
    }

    #[test]
    pub fn test_06_status_says_delete_file_3_modify_file_2_and_create_file_1() {
        let (temp_dir, mut vsc) = set_up();

        let file_1 = create_file(&temp_dir, "file1.txt");
        let file_2 = create_file(&temp_dir, "file2.txt");
        let file_3 = Path::new("file3.txt");

        vsc.local_repository.insert(file_2.display().to_string(),"File 2, File 2, File 2".to_string());
        vsc.local_repository.insert(file_3.display().to_string(), "File 3".to_string());

        assert!(matches!(vsc.status(), Ok(result) if status_contains(result.clone(), "CREATED", &file_1)));
        assert!(matches!(vsc.status(), Ok(result) if status_contains(result.clone(), "MODIFIED", &file_2)));
        assert!(matches!(vsc.status(), Ok(result) if status_contains(result.clone(), "DELETED", &file_3.to_path_buf())));
    }
}