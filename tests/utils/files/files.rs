#[cfg(test)]
mod tests {
    use std::{fs::File, path::Path};

    use rust_git::utils::files::files::read;
    use tempdir::TempDir;
    use crate::tests_functions::{create_file, set_up};

    #[test]
    fn test_01_contain_file_1() -> Result<(),std::io::Error>{
        let temp_dir = set_up();
        let file = create_file(&temp_dir, "file1.txt");
        let files = read(temp_dir.path())?;

        assert_eq!(true,files.contains_key(&file.display().to_string()));
        Ok(())
    }

    #[test]
    fn test_02_contain_file_2() {
        let temp_dir = set_up();
        let file = create_file(&temp_dir, "file2.txt");
        assert!(
            matches!(read(temp_dir.path()), Ok(files) if files.contains_key(&file.display().to_string()))
        );
    }

    #[test]
    fn test_03_contain_file_3() -> Result<(), std::io::Error> {
        let temp_dir = set_up();
        let folder_path = temp_dir.path().join("folder");
        std::fs::create_dir(&folder_path)?;
    
        let file_path = folder_path.join("file1.txt");
        let _ = File::create(&file_path)?;
    
        let result = read(temp_dir.path())?;
        assert_eq!(result.contains_key(&file_path.display().to_string()), true);
    
        Ok(())
    }

    #[test]
    fn test_04_not_contain_folder() {
       let directory = Path::new("tests/utils/files");
        let path = directory.join("folder");
        assert!(
            matches!(read(directory), Ok(files) if !files.contains_key(&path.display().to_string()))
        );
    }

    #[test]
    fn test_05_contain_file_1() {
        let temp_dir = set_up();
        let path = create_file(&temp_dir, "file1.txt");
        assert!(matches!(read(&path), Ok(files) if files.contains_key(&path.display().to_string())));
    }

    #[test]
    fn test_06_not_contain_file_2() {
        let temp_dir = set_up();
        assert!(
            !matches!(read(temp_dir.path()), Ok(files) if files.contains_key("tests/utils/files/file2.txt"))
        );
    }

    #[test]
    fn test_07_not_contain_file_4(){
        let temp_dir = set_up();
        assert!(
            !matches!(read(temp_dir.path()), Ok(files) if files.contains_key("tests/utils/files/file4.txt"))
        );
    }

    #[test]
    fn test_08_contain_one_files()  -> Result<(), std::io::Error>{
        let temp_dir = set_up();
        let folder_path = temp_dir.path().join("folder");
        std::fs::create_dir(&folder_path)?;
    
        let file_path = folder_path.join("file1.txt");
        let _ = File::create(&file_path)?;
    
        let result = read(temp_dir.path())?;
        assert_eq!(result.len(), 1);
        Ok(())
    }

    #[test]
    fn test_09_contain_four_files() {
        let temp_dir = set_up();
        let _ = create_file(&temp_dir, "file1.txt");
        let _ = create_file(&temp_dir, "file2.txt");
        let _ = create_file(&temp_dir, "file3.txt");
        let _ = create_file(&temp_dir, "file4.txt");
        assert!(matches!(read(temp_dir.path()), Ok(files) if files.len() == 4));
    }

    #[test]
    fn test_10_contain_one_file() {
        let temp_dir = set_up();
        let file = create_file(&temp_dir, "file1.txt");
        assert!(matches!(read(&file), Ok(files) if files.len() == 1));
    }

    #[test]
    fn test_11_contain_file_1() {
        let temp_dir = set_up();
        let file = create_file(&temp_dir, "file1.txt");
        assert!(matches!(read(temp_dir.path()), Ok(files) if files.contains_key(&file.display().to_string())));
    }
}