#[cfg(test)]
mod tests {

    use std::{fs, io::Write};

    use rust_git::vcs::{commands::{checkout::CheckoutOptions, rm::RemoveOption}, files::repository::Repository, version_control_system::VersionControlSystem};

    use crate::tests_functions::{create_file, set_up};

    #[test]
    pub fn test_01_merge() -> Result<(), std::io::Error> {
        let temp_dir = set_up();
        let file_1 = create_file(&temp_dir, "file1.txt");
        let file_2 = create_file(&temp_dir, "file2.txt");
        let file_3 = create_file(&temp_dir, "file3.txt");
        VersionControlSystem::add(&file_1)?;
        VersionControlSystem::add(&file_2)?;
        VersionControlSystem::add(&file_3)?;
        VersionControlSystem::commit("first commit".to_string())?;

        VersionControlSystem::checkout(CheckoutOptions::CreateAndChangeBranch("new_branch"))?;
        VersionControlSystem::checkout(CheckoutOptions::ChangeBranch("master"))?;

        let conflicts = VersionControlSystem::merge("new_branch")?;
        assert_eq!(conflicts.len(), 0);

        let repository = Repository::read_repository()?;
        assert_eq!(repository.len(), 3);
        Ok(())
    }

    #[test]
    pub fn test_02_merge() -> Result<(), std::io::Error> {
        let temp_dir = set_up();
        let file_1 = create_file(&temp_dir, "file1.txt");
        let file_2 = create_file(&temp_dir, "file2.txt");
        let file_3 = create_file(&temp_dir, "file3.txt");
        VersionControlSystem::add(&file_1)?;
        VersionControlSystem::add(&file_2)?;
        VersionControlSystem::add(&file_3)?;
        VersionControlSystem::commit("first commit".to_string())?;

        VersionControlSystem::checkout(CheckoutOptions::CreateAndChangeBranch("new_branch"))?;
        VersionControlSystem::checkout(CheckoutOptions::ChangeBranch("master"))?;
        
        let file_4 = create_file(&temp_dir, "file4.txt");
        VersionControlSystem::add(&file_4)?;
        VersionControlSystem::commit("second commit".to_string())?;

        let conflicts = VersionControlSystem::merge("new_branch")?;
        assert_eq!(conflicts.len(), 0);

        let repository = Repository::read_repository()?;

        assert_eq!(repository.len(), 4);
        Ok(())    
    }

    #[test]
    pub fn test_03_merge() -> Result<(), std::io::Error> {
        let temp_dir = set_up();
        let file_1 = create_file(&temp_dir, "file1.txt");
        let file_2 = create_file(&temp_dir, "file2.txt");
        VersionControlSystem::add(&file_1)?;
        VersionControlSystem::add(&file_2)?;
        VersionControlSystem::commit("first commit".to_string())?;
        
        VersionControlSystem::checkout(CheckoutOptions::CreateAndChangeBranch("new_branch"))?;
        let file_4 = create_file(&temp_dir, "file4.txt");
        VersionControlSystem::add(&file_4)?;
        VersionControlSystem::commit("second commit".to_string())?;
        
        VersionControlSystem::checkout(CheckoutOptions::ChangeBranch("master"))?;
        
        let file_5 = create_file(&temp_dir, "file5.txt");
        VersionControlSystem::add(&file_5)?;
        VersionControlSystem::commit("third commit".to_string())?;
        
        let conflicts = VersionControlSystem::merge("new_branch")?;
        assert_eq!(conflicts.len(), 0);

        let repository = Repository::read_repository()?;
        assert_eq!(repository.len(), 4);        
        Ok(())
    }

    #[test]
    pub fn test_04_merge() -> Result<(), std::io::Error> {
        let temp_dir = set_up();
        let file_1 = create_file(&temp_dir, "file1.txt");
        let file_2 = create_file(&temp_dir, "file2.txt");
        VersionControlSystem::add(&file_1)?;
        VersionControlSystem::add(&file_2)?;
        VersionControlSystem::commit("first commit".to_string())?;
        
        VersionControlSystem::checkout(CheckoutOptions::CreateAndChangeBranch("new_branch"))?;
        let file_4 = create_file(&temp_dir, "file4.txt");
        VersionControlSystem::add(&file_4)?;
        VersionControlSystem::commit("second commit".to_string())?;
        
        VersionControlSystem::checkout(CheckoutOptions::ChangeBranch("master"))?;
        
        VersionControlSystem::rm(&file_1, RemoveOption::NoDirectory)?;
        VersionControlSystem::commit("third commit".to_string())?;
        
        let conflicts = VersionControlSystem::merge("new_branch")?;
        assert_eq!(conflicts.len(), 0);

        let repository = Repository::read_repository()?;
        assert_eq!(repository.len(), 2);        
        Ok(())
    }

    #[test]
    pub fn test_05_merge() -> Result<(), std::io::Error> {
        let temp_dir = set_up();
        let file_1 = create_file(&temp_dir, "file1.txt");
        let file_2 = create_file(&temp_dir, "file2.txt");
        VersionControlSystem::add(&file_1)?;
        VersionControlSystem::add(&file_2)?;
        VersionControlSystem::commit("first commit".to_string())?;
        
        VersionControlSystem::checkout(CheckoutOptions::CreateAndChangeBranch("new_branch"))?;

        let mut file2 = fs::OpenOptions::new().write(true).create(true).append(true).open(&file_2)?;
        let _ = file2.write_all(b"contenido");
        
        VersionControlSystem::add(&file_2)?;
        VersionControlSystem::commit("second commit".to_string())?;
        
        VersionControlSystem::checkout(CheckoutOptions::ChangeBranch("master"))?;

        let mut file1 = fs::OpenOptions::new().write(true).create(true).append(true).open(&file_1)?;
        let _ = file1.write_all(b"contenido");
        
        VersionControlSystem::add(&file_1)?;
        VersionControlSystem::commit("third commit".to_string())?;
        
        let conflicts = VersionControlSystem::merge("new_branch")?;
        assert_eq!(conflicts.len(), 0);

        let repository = Repository::read_repository()?;
        assert_eq!(repository.len(), 2);        
        Ok(())
    }

    #[test]
    pub fn test_06_merge() -> Result<(), std::io::Error> {
        let temp_dir = set_up();
        let file_1 = create_file(&temp_dir, "file1.txt");
        VersionControlSystem::add(&file_1)?;
        VersionControlSystem::commit("first commit".to_string())?;
        
        VersionControlSystem::checkout(CheckoutOptions::CreateAndChangeBranch("new_branch"))?;

        let file_2 = create_file(&temp_dir, "file2.txt");
        VersionControlSystem::add(&file_2)?;
        VersionControlSystem::commit("second commit".to_string())?;
        
        VersionControlSystem::checkout(CheckoutOptions::ChangeBranch("master"))?;
    

        let conflicts = VersionControlSystem::merge("new_branch")?;
        assert_eq!(conflicts.len(), 0);
    
        let repository = Repository::read_repository()?;
        assert_eq!(repository.len(), 2);        
        Ok(())
    }    

    #[test]
    pub fn test_07_merge() -> Result<(), std::io::Error> {
        let temp_dir = set_up();
        let file_1 = create_file(&temp_dir, "file1.txt");
        VersionControlSystem::add(&file_1)?;
        VersionControlSystem::commit("first commit".to_string())?;
        
        VersionControlSystem::checkout(CheckoutOptions::CreateAndChangeBranch("new_branch"))?;

        let file_2 = create_file(&temp_dir, "file2.txt");
        VersionControlSystem::add(&file_2)?;
        VersionControlSystem::commit("second commit".to_string())?;
            
        let conflicts = VersionControlSystem::merge("master")?;
        assert_eq!(conflicts.len(), 0);

        let repository = Repository::read_repository()?;
        assert_eq!(repository.len(), 2);        
        Ok(())
    } 

    #[test]
    pub fn test_08_merge() -> Result<(), std::io::Error> {
        let temp_dir = set_up();
        let file_1 = create_file(&temp_dir, "file1.txt");
        VersionControlSystem::add(&file_1)?;
        VersionControlSystem::commit("first commit".to_string())?;
        
        VersionControlSystem::checkout(CheckoutOptions::CreateAndChangeBranch("new_branch"))?;

        let file_2 = create_file(&temp_dir, "file2.txt");
        VersionControlSystem::add(&file_2)?;
        VersionControlSystem::commit("second commit".to_string())?;
            
        let conflicts = VersionControlSystem::merge("master")?;
        assert_eq!(conflicts.len(), 0);

        let repository = Repository::read_repository()?;
        assert_eq!(repository.len(), 2);        
        Ok(())
    } 

    #[test]
    pub fn test_09_conflict() -> Result<(), std::io::Error> {
        let temp_dir = set_up();
        let file_1 = create_file(&temp_dir, "file1.txt");

        VersionControlSystem::add(&file_1)?;
        VersionControlSystem::commit("first commit".to_string())?;
        
        VersionControlSystem::checkout(CheckoutOptions::CreateAndChangeBranch("new_branch"))?;

        let mut file1 = fs::OpenOptions::new().write(true).create(true).append(true).open(&file_1)?;
        let _ = file1.write_all(b"content");
        VersionControlSystem::add(&file_1)?;
        VersionControlSystem::commit("second commit".to_string())?;
        
        VersionControlSystem::checkout(CheckoutOptions::ChangeBranch("master"))?;

        let mut file1 = fs::OpenOptions::new().write(true).create(true).append(true).open(&file_1)?;
        let _ = file1.write_all(b"other content");
        VersionControlSystem::add(&file_1)?;
        VersionControlSystem::commit("third commit".to_string())?;

        let conflicts = VersionControlSystem::merge("new_branch")?;
        assert_eq!(conflicts.len(), 1);

        Ok(())
    }

    #[test]
    pub fn test_10_resolve_conflict() -> Result<(), std::io::Error> {
        let temp_dir = set_up();
        let file_1 = create_file(&temp_dir, "file1.txt");

        VersionControlSystem::add(&file_1)?;
        VersionControlSystem::commit("first commit".to_string())?;
        
        VersionControlSystem::checkout(CheckoutOptions::CreateAndChangeBranch("new_branch"))?;

        let mut file1 = fs::OpenOptions::new().write(true).create(true).append(true).open(&file_1)?;
        let _ = file1.write_all(b"content");
        VersionControlSystem::add(&file_1)?;
        VersionControlSystem::commit("second commit".to_string())?;
        
        VersionControlSystem::checkout(CheckoutOptions::ChangeBranch("master"))?;

        let mut file1 = fs::OpenOptions::new().write(true).create(true).append(true).open(&file_1)?;
        let _ = file1.write_all(b"other content");
        VersionControlSystem::add(&file_1)?;
        VersionControlSystem::commit("third commit".to_string())?;

        let conflicts = VersionControlSystem::merge("new_branch")?;
        assert_eq!(conflicts.len(), 1);
        Ok(())
    }
}