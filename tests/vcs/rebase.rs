#[cfg(test)]
mod tests {

    use std::{fs, io::Write};

    use rust_git::vcs::{commands::{checkout::CheckoutOptions, rm::RemoveOption}, files::repository::Repository, version_control_system::VersionControlSystem};

    use crate::tests_functions::{create_file, set_up};

    #[test]
    pub fn test_01_rebase() -> Result<(), std::io::Error> {
        let temp_dir = set_up();
        let file_1 = create_file(&temp_dir, "file1.txt");
        let file_2 = create_file(&temp_dir, "file2.txt");

        VersionControlSystem::add(&file_1)?;
        VersionControlSystem::add(&file_2)?;
        VersionControlSystem::commit("first commit".to_string())?;

        VersionControlSystem::checkout(CheckoutOptions::CreateAndChangeBranch("new_branch"))?;

        let file_3 = create_file(&temp_dir, "file3.txt");
        VersionControlSystem::add(&file_3)?;
        VersionControlSystem::commit("second commit".to_string())?;
        
        VersionControlSystem::checkout(CheckoutOptions::ChangeBranch("master"))?;
        
        let file_4 = create_file(&temp_dir, "file4.txt");
        VersionControlSystem::add(&file_4)?;
        VersionControlSystem::commit("third commit".to_string())?;

        VersionControlSystem::rebase("new_branch")?;
        
        let repository = Repository::read_repository()?;
        assert_eq!(repository.len(), 4);

        Ok(())
    }
}