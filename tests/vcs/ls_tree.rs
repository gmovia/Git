#[cfg(test)]
mod tests {

    use std::fs::{self, File};

    use rust_git::vcs::{
        commands::{branch::BranchOptions, checkout::CheckoutOptions},
        version_control_system::VersionControlSystem,
    };

    use crate::tests_functions::{create_dir, create_file, set_up};

    #[test]
    pub fn test_01_commit_2_files_then_ls_tree_len_is_2() -> Result<(), std::io::Error> {
        let temp_dir = set_up();

        let file_1 = create_file(&temp_dir, "file1.txt");
        let file_2 = create_file(&temp_dir, "file2.txt");

        VersionControlSystem::add(&file_1)?;
        VersionControlSystem::add(&file_2)?;

        VersionControlSystem::commit("first_commit".to_string())?;

        let current_branch = VersionControlSystem::branch(BranchOptions::GetCurrentBranch)?;
        let information = VersionControlSystem::ls_tree(&current_branch[0])?;
        assert_eq!(information.len(), 2);
        Ok(())
    }

    #[test]
    pub fn test_02_add_2_files_but_not_commited_then_ls_tree_len_is_0() -> Result<(), std::io::Error>
    {
        let temp_dir = set_up();

        let file_1 = create_file(&temp_dir, "file1.txt");
        let file_2 = create_file(&temp_dir, "file2.txt");

        VersionControlSystem::add(&file_1)?;
        VersionControlSystem::add(&file_2)?;

        let current_branch = VersionControlSystem::branch(BranchOptions::GetCurrentBranch)?;
        let information = VersionControlSystem::ls_tree(&current_branch[0])?;
        assert_eq!(information.len(), 0);
        Ok(())
    }

    #[test]
    pub fn test_03_commit_2_files_in_new_branch_then_ls_tree_len_is_2() -> Result<(), std::io::Error>
    {
        let temp_dir = set_up();

        let _ =
            VersionControlSystem::checkout(CheckoutOptions::CreateAndChangeBranch("new_branch"))?;

        let file_1 = create_file(&temp_dir, "file1.txt");
        let file_2 = create_file(&temp_dir, "file2.txt");

        VersionControlSystem::add(&file_1)?;
        VersionControlSystem::add(&file_2)?;

        VersionControlSystem::commit("first_commit".to_string())?;

        let current_branch = VersionControlSystem::branch(BranchOptions::GetCurrentBranch)?;
        let information_current = VersionControlSystem::ls_tree(&current_branch[0])?;

        let information_master = VersionControlSystem::ls_tree("master")?;

        assert_eq!(information_current.len(), 2);
        assert_eq!(information_master.len(), 0);
        Ok(())
    }

    #[test]
    pub fn test_04_commit_2_files_and_2_dirs_with_1_file_each_then_ls_tree_len_is_6(
    ) -> Result<(), std::io::Error> {
        let temp_dir = set_up();

        let file_1 = create_file(&temp_dir, "file1.txt");
        let file_2 = create_file(&temp_dir, "file2.txt");
        let dir_path = create_dir(&temp_dir, "carpeta");
        let file_3 = dir_path.join("file3.txt");
        let _ = File::create(&file_3);
        let dir_path2 = dir_path.join("otra_carpeta");
        fs::create_dir_all(&dir_path2).expect("Failed to create directory");
        let file_4 = dir_path2.join("file4.txt");
        let _ = File::create(&file_4);

        VersionControlSystem::add(&file_1)?;
        VersionControlSystem::add(&file_2)?;
        VersionControlSystem::add(&file_3)?;
        VersionControlSystem::add(&file_4)?;

        VersionControlSystem::commit("first_commit".to_string())?;

        let current_branch = VersionControlSystem::branch(BranchOptions::GetCurrentBranch)?;
        let information = VersionControlSystem::ls_tree(&current_branch[0])?;
        assert_eq!(information.len(), 6);
        Ok(())
    }
}
