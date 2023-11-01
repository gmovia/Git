#[cfg(test)]
mod tests {

    use std::{fs, io::Write};

    use rust_git::vcs::commands::{checkout::CheckoutOptions, rm::RemoveOption};

    use crate::tests_functions::{create_file, set_up};

    #[test]
    pub fn test_01_merge() -> Result<(), std::io::Error> {
        let (temp_dir, vcs) = set_up();
        let file_1 = create_file(&temp_dir, "file1.txt");
        let file_2 = create_file(&temp_dir, "file2.txt");
        let file_3 = create_file(&temp_dir, "file3.txt");
        vcs.add(&file_1)?;
        vcs.add(&file_2)?;
        vcs.add(&file_3)?;
        vcs.commit("first commit".to_string())?;

        vcs.checkout(CheckoutOptions::CreateAndChangeBranch("new_branch"))?;
        vcs.checkout(CheckoutOptions::ChangeBranch("master"))?;

        let _ = vcs.merge("new_branch")?;

        let repository = vcs.repository.read_repository()?;
        assert_eq!(repository.len(), 3);
        Ok(())
    }

    #[test]
    pub fn test_02_merge() -> Result<(), std::io::Error> {
        let (temp_dir, vcs) = set_up();
        let file_1 = create_file(&temp_dir, "file1.txt");
        let file_2 = create_file(&temp_dir, "file2.txt");
        let file_3 = create_file(&temp_dir, "file3.txt");
        vcs.add(&file_1)?;
        vcs.add(&file_2)?;
        vcs.add(&file_3)?;
        vcs.commit("first commit".to_string())?;

        vcs.checkout(CheckoutOptions::CreateAndChangeBranch("new_branch"))?; // f1 f2 f3
        vcs.checkout(CheckoutOptions::ChangeBranch("master"))?;
        
        let file_4 = create_file(&temp_dir, "file4.txt"); // f1 f2 f3 f4
        vcs.add(&file_4)?;
        vcs.commit("second commit".to_string())?;

        let _ = vcs.merge("new_branch")?;

        let repository = vcs.repository.read_repository()?;
        println!("{:?}", repository);
        assert_eq!(repository.len(), 4);
        Ok(())    
    }

    #[test]
    pub fn test_03_merge() -> Result<(), std::io::Error> {
        let (temp_dir, vcs) = set_up();
        let file_1 = create_file(&temp_dir, "file1.txt");
        let file_2 = create_file(&temp_dir, "file2.txt");
        vcs.add(&file_1)?;
        vcs.add(&file_2)?;
        vcs.commit("first commit".to_string())?;
        
        vcs.checkout(CheckoutOptions::CreateAndChangeBranch("new_branch"))?;
        let file_4 = create_file(&temp_dir, "file4.txt");
        vcs.add(&file_4)?;
        vcs.commit("second commit".to_string())?;
        
        vcs.checkout(CheckoutOptions::ChangeBranch("master"))?;
        
        let file_5 = create_file(&temp_dir, "file5.txt");
        vcs.add(&file_5)?;
        vcs.commit("third commit".to_string())?;
        
        let _ = vcs.merge("new_branch")?;

        let repository = vcs.repository.read_repository()?;
        assert_eq!(repository.len(), 4);        
        Ok(())
    }

    #[test]
    pub fn test_04_merge() -> Result<(), std::io::Error> {
        let (temp_dir, vcs) = set_up();
        let file_1 = create_file(&temp_dir, "file1.txt");
        let file_2 = create_file(&temp_dir, "file2.txt");
        vcs.add(&file_1)?;
        vcs.add(&file_2)?;
        vcs.commit("first commit".to_string())?;
        
        vcs.checkout(CheckoutOptions::CreateAndChangeBranch("new_branch"))?;
        let file_4 = create_file(&temp_dir, "file4.txt");
        vcs.add(&file_4)?;
        vcs.commit("second commit".to_string())?;
        
        vcs.checkout(CheckoutOptions::ChangeBranch("master"))?;
        
        vcs.rm(&file_1, RemoveOption::NoDirectory)?;
        vcs.commit("third commit".to_string())?;
        
        let _ = vcs.merge("new_branch")?;

        let repository = vcs.repository.read_repository()?;
        assert_eq!(repository.len(), 2);        
        Ok(())
    }

    #[test]
    pub fn test_05_merge() -> Result<(), std::io::Error> {
        let (temp_dir, vcs) = set_up();
        let file_1 = create_file(&temp_dir, "file1.txt");
        let file_2 = create_file(&temp_dir, "file2.txt");
        vcs.add(&file_1)?;
        vcs.add(&file_2)?;
        vcs.commit("first commit".to_string())?;
        
        vcs.checkout(CheckoutOptions::CreateAndChangeBranch("new_branch"))?;

        let mut file2 = fs::OpenOptions::new().write(true).create(true).append(true).open(&file_2)?;
        let _ = file2.write_all(b"contenido");
        
        vcs.add(&file_2)?;
        vcs.commit("second commit".to_string())?;
        
        vcs.checkout(CheckoutOptions::ChangeBranch("master"))?;

        let mut file1 = fs::OpenOptions::new().write(true).create(true).append(true).open(&file_1)?;
        let _ = file1.write_all(b"contenido");
        
        vcs.add(&file_1)?;
        vcs.commit("third commit".to_string())?;
        
        let _ = vcs.merge("new_branch")?;
        
        let repository = vcs.repository.read_repository()?;
        assert_eq!(repository.len(), 2);        
        Ok(())
    }

}