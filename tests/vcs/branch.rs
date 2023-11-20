

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use rust_git::vcs::{commands::branch::{BranchOptions, Branch}, version_control_system::VersionControlSystem, files::current_repository::CurrentRepository};

    use crate::tests_functions::set_up;

    pub fn count_files(path: &Path) -> Result<usize, std::io::Error>{
        let mut file_count = 0;

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if entry.is_ok() {
                    file_count += 1;
                }
            }
        }
        Ok(file_count)
    }

    #[test]
    pub fn test_01_create_a_branch_then_len_dir_logs_is_2() -> Result<(),std::io::Error>{
        let _temp_dir = set_up();

        VersionControlSystem::branch(BranchOptions::NewBranch("new_branch"))?;
        let current  = CurrentRepository::read()?;

        let branch_path = current.join(".rust_git").join("refs").join("heads");

        assert_eq!(count_files(&branch_path)?,2);
        Ok(())
    }



    #[test]
    pub fn test_02_create_2_branches_and_get_it() -> Result<(),std::io::Error>{
        let __temp_dir = set_up();

        VersionControlSystem::branch(BranchOptions::NewBranch("new_branch"))?;
        VersionControlSystem::branch(BranchOptions::NewBranch("another_branch"))?;

        let current  = CurrentRepository::read()?;
        let branches = Branch::get_branches(&current)?;

        assert_eq!(branches.contains(&"master".to_string()),true);
        assert_eq!(branches.contains(&"new_branch".to_string()),true);
        assert_eq!(branches.contains(&"another_branch".to_string()),true);
        assert_eq!(branches.len(),3);
        Ok(())
    }

    #[test]
    pub fn test_03_delete_a_branch_then_len_is_2() -> Result<(),std::io::Error>{
        let _temp_dir = set_up();

        VersionControlSystem::branch(BranchOptions::NewBranch("new_branch"))?;
        VersionControlSystem::branch(BranchOptions::NewBranch("another_brach"))?;
        VersionControlSystem::branch(BranchOptions::NewBranch("a_third_brach"))?;

        VersionControlSystem::branch(BranchOptions::DeleteBranch("new_branch"))?;
        
        let current  = CurrentRepository::read()?;
        let branch_path = current.join(".rust_git").join("refs").join("heads");

        assert_eq!(count_files(&branch_path)?, 3);

        Ok(())
    }


#[test]
    pub fn test_04_try_to_delete_master_branch() -> Result<(),std::io::Error>{
        let _temp_dir = set_up();

        let result = VersionControlSystem::branch(BranchOptions::DeleteBranch("master"));

        assert!(result.is_err());

        Ok(())
    }
}