#[cfg(test)]
mod tests {
    use std::path::Path;
    use rust_git::file::VCSFile;
    use rust_git::version_control_system::VersionControlSystem;
    use crate::tests_functions::{create_file, set_up, status_contains};

    #[test]
    pub fn test_01_untracked_files_contains_four_files() {
        let version_control_system = VersionControlSystem::init("tests/utils/files".to_string());
        assert!(matches!(version_control_system.status(), Ok((untracked_files, _, _)) if untracked_files.len() == 4));
    }

    #[test]
    pub fn test_02_untracked_files_contains_one_file_with_status_created() {
        let (temp_dir, vsc) = set_up();
        let path = create_file(&temp_dir, "file1.txt");        
        assert!(matches!(vsc.status(), Ok((untracked_files, _, _)) if status_contains(untracked_files.clone(), "CREATED", &path)));
    }
    
    #[test]
    pub fn test_03_untracked_files_not_contains_files() {
        let (temp_dir, mut vsc) = set_up();
        let path = create_file(&temp_dir, "file1.txt");
        
        vsc.local_repository.insert(path.display().to_string(), "contenido".to_string());
        assert!(matches!(vsc.status(), Ok((untracked_files, _, _)) if untracked_files.len() == 0));
    }

    #[test]
    pub fn test_04_changes_not_staged_for_commit_contains_one_file() {
        let (temp_dir, mut vsc) = set_up();
        let path = create_file(&temp_dir, "file1.txt");
        
        vsc.local_repository.insert(path.display().to_string(), "contenido".to_string());
        
        assert!(matches!(vsc.status(), Ok((_, changes_not_staged_for_commit, _)) if status_contains(changes_not_staged_for_commit.clone(), "MODIFIED", &path)));
        assert!(matches!(vsc.status(), Ok((_, changes_not_staged_for_commit, _)) if changes_not_staged_for_commit.len() == 1));
    }

    #[test]
    pub fn test_05_changes_not_staged_for_commit_contains_one_file() {
        let (_, mut vsc) = set_up();
        let path = Path::new("file1.txt");
        
        vsc.local_repository.insert(path.display().to_string(), "contenido".to_string());
        
        assert!(matches!(vsc.status(), Ok((_, changes_not_staged_for_commit, _)) if status_contains(changes_not_staged_for_commit.clone(), "DELETED", &path.to_path_buf())));
        assert!(matches!(vsc.status(), Ok((_, changes_not_staged_for_commit, _)) if changes_not_staged_for_commit.len() == 1));
    }
    
    
    #[test]
    pub fn test_06_untracked_file_contain_file_1_and_changes_not_staged_for_commit_contain_file_3() {
        let (temp_dir, mut vsc) = set_up();
        
        let file_1 = create_file(&temp_dir, "file1.txt");
        let file_3 = Path::new("file3.txt");
        
        vsc.local_repository.insert(file_3.display().to_string(), "File 3".to_string());
        

        assert!(matches!(vsc.status(), Ok((_, changes_not_staged_for_commit, _)) if status_contains(changes_not_staged_for_commit.clone(), "DELETED", &file_3.to_path_buf())));
        assert!(matches!(vsc.status(), Ok((untracked_file, _, _)) if status_contains(untracked_file.clone(), "CREATED", &file_1)));
    }
    
    #[test]
    pub fn test_07_untracked_file_contain_file_1_and_changes_not_staged_for_commit_contain_file_2_and_file_3() {
        let (temp_dir, mut vsc) = set_up();
        
        let file_1 = create_file(&temp_dir, "file1.txt");
        let file_2 = create_file(&temp_dir, "file2.txt");
        let file_3 = Path::new("file3.txt");
        
        vsc.local_repository.insert(file_2.display().to_string(),"File 2, File 2, File 2".to_string());
        vsc.local_repository.insert(file_3.display().to_string(), "File 3".to_string());
        
        assert!(matches!(vsc.status(), Ok((untracked_file, _, _)) if status_contains(untracked_file.clone(), "CREATED", &file_1)));
        assert!(matches!(vsc.status(), Ok((_, changes_not_staged_for_commit, _)) if status_contains(changes_not_staged_for_commit.clone(), "MODIFIED", &file_2)));
        assert!(matches!(vsc.status(), Ok((_, changes_not_staged_for_commit, _)) if status_contains(changes_not_staged_for_commit.clone(), "DELETED", &file_3.to_path_buf())));

        assert!(matches!(vsc.status(), Ok((untracked_file, _, _)) if untracked_file.len() == 1));
        assert!(matches!(vsc.status(), Ok((_, changes_not_staged_for_commit, _)) if changes_not_staged_for_commit.len() == 2));
    }

    // AGREGAR TESTS QUE AHORA INCLUYAN AL AREA DE STAGING

    #[test]
    pub fn test_08_staging_area_contain_file_1() {
        let (temp_dir, mut vsc) = set_up();
        let path = create_file(&temp_dir, "file1.txt");    

        vsc.staging_area.insert(path.display().to_string(), VCSFile::new(path.display().to_string(), "A".to_string(), "CREATED".to_string()));
        assert!(matches!(vsc.status(), Ok((_, _, changes_to_be_commited)) if status_contains(changes_to_be_commited.clone(), "CREATED", &path)));
        assert!(matches!(vsc.status(), Ok((untracked_file, _, _)) if untracked_file.len() == 0));
    }

    #[test]
    pub fn test_09_staging_area_contain_file_1_and_changes_not_staged_for_commit_contain_file1() {
        let (temp_dir, mut vsc) = set_up();
        let path = create_file(&temp_dir, "file1.txt");    

        vsc.staging_area.insert(path.display().to_string(), VCSFile::new(path.display().to_string(), "A".to_string(), "MODIFIED".to_string()));
        vsc.local_repository.insert(path.display().to_string(), "File 3".to_string());

        assert!(matches!(vsc.status(), Ok((_, _, changes_to_be_commited)) if status_contains(changes_to_be_commited.clone(), "MODIFIED", &path)));
        assert!(matches!(vsc.status(), Ok((untracked_file, _, _)) if untracked_file.len() == 0));
        assert!(matches!(vsc.status(), Ok((_, changes_not_staged_for_commit, _)) if changes_not_staged_for_commit.len() == 1));
    }

    #[test]
    pub fn test_10_changes_to_be_commited_includes_file_1() {
        let (temp_dir, mut vsc) = set_up();
        let path = create_file(&temp_dir, "file1.txt");    

        vsc.staging_area.insert(path.display().to_string(), VCSFile::new(path.display().to_string(), "".to_string(), "MODIFIED".to_string()));
        vsc.local_repository.insert(path.display().to_string(), "File 3".to_string());

        assert!(matches!(vsc.status(), Ok((_, _, changes_to_be_commited)) if status_contains(changes_to_be_commited.clone(), "MODIFIED", &path)));
        assert!(matches!(vsc.status(), Ok((untracked_file, _, _)) if untracked_file.len() == 0));
        assert!(matches!(vsc.status(), Ok((_, changes_not_staged_for_commit, _)) if changes_not_staged_for_commit.len() == 0));
    }

    #[test]
    pub fn test_11_changes_not_staged_for_commit_contain_file_1() {
        let (_, mut vsc) = set_up();
        let path = Path::new("file1.txt");   

        vsc.local_repository.insert(path.display().to_string(), "File 3".to_string());

        assert!(matches!(vsc.status(), Ok((untracked_file, _, _)) if untracked_file.len() == 0));
        assert!(matches!(vsc.status(), Ok((_, changes_not_staged_for_commit, _)) if changes_not_staged_for_commit.len() == 1));
    }

    #[test]
    pub fn test_12_changes_to_be_commited_contain_file_1() {
        let (_, mut vsc) = set_up();
        let path = Path::new("file1.txt");   

        vsc.staging_area.insert(path.display().to_string(), VCSFile::new(path.display().to_string(), "".to_string(), "DELETED".to_string()));
        vsc.local_repository.insert(path.display().to_string(), "File 3".to_string());

        assert!(matches!(vsc.status(), Ok((_, _, changes_to_be_commited)) if changes_to_be_commited.len() == 1));
        assert!(matches!(vsc.status(), Ok((_, changes_not_staged_for_commit, _)) if changes_not_staged_for_commit.len() == 0));
    }

    #[test]
    pub fn test_13_three_sets_are_empty() {
        let (temp_dir, mut vsc) = set_up();
        let path = create_file(&temp_dir, "file1.txt");    

        vsc.local_repository.insert(path.display().to_string(), "".to_string());

        assert!(matches!(vsc.status(), Ok((untracked_file, _, _)) if untracked_file.len() == 0));
        assert!(matches!(vsc.status(), Ok((_, _, changes_to_be_commited)) if changes_to_be_commited.len() == 0));
        assert!(matches!(vsc.status(), Ok((_, changes_not_staged_for_commit, _)) if changes_not_staged_for_commit.len() == 0));
    }
}