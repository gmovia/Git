#[cfg(test)]
mod tests {
    use rust_git::utils::files::files::read;
    use std::path::Path;

    const TESTS_PATH: &str = "tests/utils/files";
    const FOLDER_PATH: &str = "tests/utils/files/folder";
    const FILE_1_PATH: &str = "tests/utils/files/file1.txt";
    const FILE_2_PATH: &str = "tests/utils/files/file2.txt";
    const FILE_3_PATH: &str = "tests/utils/files/folder/file3.txt";
    const INEXIST_FILE_PATH: &str = "tests/utils/files/folder/file4.txt";
    const TEST_ERROR: &str = "ERROR: An error ocurred while running the test";

    #[test]
    fn test_01_contain_file_1() {
        let directory = Path::new(TESTS_PATH);

        match read(directory) {
            Ok(files) => assert_eq!(files.contains_key(FILE_1_PATH), true),
            Err(_) => panic!("{}", TEST_ERROR),
        }
    }

    #[test]
    fn test_02_contain_file_2() {
        let directory = Path::new(TESTS_PATH);

        match read(directory) {
            Ok(files) => assert_eq!(files.contains_key(FILE_2_PATH), true),
            Err(_) => panic!("{}", TEST_ERROR),
        }
    }

    #[test]
    fn test_03_contain_file_3() {
        let directory = Path::new(TESTS_PATH);

        match read(directory) {
            Ok(files) => assert_eq!(files.contains_key(FILE_3_PATH), true),
            Err(_) => panic!("{}", TEST_ERROR),
        }
    }

    #[test]
    fn test_04_contain_folder() {
        let directory = Path::new(TESTS_PATH);

        match read(directory) {
            Ok(files) => assert_eq!(files.contains_key(FOLDER_PATH), true),
            Err(_) => panic!("{}", TEST_ERROR),
        }
    }

    #[test]
    fn test_05_contain_file_1() {
        let file = Path::new(FILE_1_PATH);

        match read(file) {
            Ok(files) => assert_eq!(files.contains_key(FILE_1_PATH), true),
            Err(_) => panic!("{}", TEST_ERROR),
        }
    }

    #[test]
    fn test_06_not_contain_file_2() {
        let directory = Path::new(FOLDER_PATH);

        match read(directory) {
            Ok(files) => assert_eq!(files.contains_key(FILE_2_PATH), false),
            Err(_) => panic!("{}", TEST_ERROR),
        }
    }

    #[test]
    fn test_07_not_contain_file_4() {
        let directory = Path::new(FOLDER_PATH);

        match read(directory) {
            Ok(files) => assert_eq!(files.contains_key(INEXIST_FILE_PATH), false),
            Err(_) => panic!("{}", TEST_ERROR),
        }
    }

    #[test]
    fn test_08_contain_two_files() {
        let directory = Path::new(FOLDER_PATH);

        match read(directory) {
            Ok(files) => assert_eq!(files.len(), 2),
            Err(_) => panic!("{}", TEST_ERROR),
        }
    }

    #[test]
    fn test_09_contain_four_files() {
        let directory = Path::new(TESTS_PATH);

        match read(directory) {
            Ok(files) => assert_eq!(files.len(), 6),
            Err(_) => panic!("{}", TEST_ERROR),
        }
    }

    #[test]
    fn test_10_contain_one_file() {
        let directory = Path::new(FILE_1_PATH);

        match read(directory) {
            Ok(files) => assert_eq!(files.len(), 1),
            Err(_) => panic!("{}", TEST_ERROR),
        }
    }

    #[test]
    fn test_11_contain_file_1() {
        let file = Path::new(FILE_1_PATH);

        match read(file) {
            Ok(files) => assert_eq!(files.contains_key(FILE_1_PATH), true),
            Err(_) => panic!("{}", TEST_ERROR),
        }
    }
}
