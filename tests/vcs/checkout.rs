#[cfg(test)]
mod tests {
    use std::{fs::File, io::{self, BufRead}, path::Path};

    use rust_git::vcs::commands::{branch::BranchOptions, checkout::CheckoutOptions};

    use crate::tests_functions::{set_up, create_file};

    pub fn count_lines(path: &str) -> Result<isize,std::io::Error>{
        let file = File::open(path)?;

        let reader = io::BufReader::new(file);
        let mut line_count = 0;
        for _ in reader.lines() {
            line_count += 1;
        }
        Ok(line_count)
    }

    #[test]
    pub fn test_01_create_a_branch_then_change_to_it_and_add_a_file_in_branch_changed() -> Result<(),std::io::Error>{
        let (temp_dir, mut vcs) = set_up();
        let path = create_file(&temp_dir, "file1.txt");

        vcs.branch(BranchOptions::NewBranch("new_branch"))?;
        vcs.checkout(CheckoutOptions::ChangeBranch("new_branch"))?;

        let vcs_path = Path::new(&vcs.path);
        let master_branch_path = vcs_path.join(".rust_git").join("logs").join("master");
        let changed_branch_path = vcs_path.join(".rust_git").join("logs").join("new_branch");

        vcs.add(&path)?;
        vcs.commit("commit in new branch".to_string())?;

        assert_eq!(count_lines(&changed_branch_path.display().to_string())?,1);
        assert_eq!(count_lines(&master_branch_path.display().to_string())?,0);
        Ok(())
    }

    #[test]
    pub fn test_02_add_a_file_in_master_then_create_and_change_branch_then_add_a_new_file_in_new_branch() -> Result<(),std::io::Error>{
        let (temp_dir, mut vcs) = set_up();
        let path_file1 = create_file(&temp_dir, "file1.txt");
        let path_file2 = create_file(&temp_dir, "file2.txt");

        vcs.add(&path_file2)?;
        vcs.commit("commit in master".to_string())?;

        vcs.checkout(CheckoutOptions::CreateAndChangeBranch("new_branch"))?;

        let vcs_path = Path::new(&vcs.path);
        let master_branch_path = vcs_path.join(".rust_git").join("logs").join("master");
        let new_branch_path = vcs_path.join(".rust_git").join("logs").join("new_branch");

        vcs.add(&path_file1)?;
        vcs.commit("commit in new_branch".to_string())?;

        assert_eq!(count_lines(&new_branch_path.display().to_string())?,2);
        assert_eq!(count_lines(&master_branch_path.display().to_string())?,1);
        Ok(())
    }



}