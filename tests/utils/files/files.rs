#[cfg(test)]
mod tests {
    use rust_git::utils::files::files::read;
    use std::path::Path;

    #[test]
    fn test_01_contain_file_1() {
        let directory = Path::new("tests/utils/files");
        let path = directory.join("file1.txt");
        assert!(
            matches!(read(directory), Ok(files) if files.contains_key(&path.display().to_string()))
        );
    }

    #[test]
    fn test_02_contain_file_2() {
        let directory = Path::new("tests/utils/files");
        let path = directory.join("file2.txt");
        assert!(
            matches!(read(directory), Ok(files) if files.contains_key(&path.display().to_string()))
        );
    }

    #[test]
    fn test_03_contain_file_3() {
        let directory = Path::new("tests/utils/files");
        let path = directory.join("folder").join("file3.txt");
        assert!(
            matches!(read(directory), Ok(files) if files.contains_key(&path.display().to_string()))
        );
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
        let file = Path::new("tests/utils/files/file1.txt");
        assert!(matches!(read(file), Ok(files) if files.contains_key(&file.display().to_string())));
    }

    #[test]
    fn test_06_not_contain_file_2() {
        let directory = Path::new("tests/utils/files/folder");
        assert!(
            !matches!(read(directory), Ok(files) if files.contains_key("tests/utils/files/file2.txt"))
        );
    }

    #[test]
    fn test_07_not_contain_file_4() {
        let directory = Path::new("tests/utils/files/folder");
        assert!(
            !matches!(read(directory), Ok(files) if files.contains_key("tests/utils/files/folder/file4.txt"))
        );
    }

    #[test]
    fn test_08_contain_one_files() {
        let directory = Path::new("tests/utils/files/folder");
        assert!(matches!(read(directory), Ok(files) if files.len() == 1));
    }

    #[test]
    fn test_09_contain_four_files() {
        let directory = Path::new("tests/utils/files");
        assert!(matches!(read(directory), Ok(files) if files.len() == 4));
    }

    #[test]
    fn test_10_contain_one_file() {
        let directory = Path::new("tests/utils/files/file1.txt");
        assert!(matches!(read(directory), Ok(files) if files.len() == 1));
    }

    #[test]
    fn test_11_contain_file_1() {
        let file = Path::new("tests/utils/files/file1.txt");
        assert!(matches!(read(file), Ok(files) if files.contains_key(&file.display().to_string())));
    }
}
