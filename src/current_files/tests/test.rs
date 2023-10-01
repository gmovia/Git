
#[cfg(test)]
mod tests {
    use crate::current_files::current_files::get_files;
    use std::path::Path;

    #[test]
    fn test_01_contain_file_1() {
        let directory = Path::new("src/current_files/tests");
        let file = directory.join("file1.txt");
    
        match get_files(directory.to_str().unwrap().to_string()) {
            Ok(files) => assert_eq!(files.contains_key(&file.to_str().unwrap().to_string()), true),
            Err(err) => panic!("Error: {:?}", err),
        }
    }

    #[test]
    fn test_02_contain_file_2() {
        let directory = Path::new("src/current_files/tests");
        let file = directory.join("file2.txt");
    
        match get_files(directory.to_str().unwrap().to_string()) {
            Ok(files) => assert_eq!(files.contains_key(&file.to_str().unwrap().to_string()), true),
            Err(err) => panic!("Error: {:?}", err),
        }
    }

    #[test]
    fn test_03_contain_file_3() {
        let directory = Path::new("src/current_files/tests");
        let file = directory.join("folder").join("file3.txt");
    
        match get_files(directory.to_str().unwrap().to_string()) {
            Ok(files) => assert_eq!(files.contains_key(&file.to_str().unwrap().to_string()), true),
            Err(err) => panic!("Error: {:?}", err),
        }
    }

    #[test]
    fn test_04_contain_folder() {
        let directory = Path::new("src/current_files/tests");
        let file = directory.join("folder");
    
        match get_files(directory.to_str().unwrap().to_string()) {
            Ok(files) => assert_eq!(files.contains_key(&file.to_str().unwrap().to_string()), true),
            Err(err) => panic!("Error: {:?}", err),
        }
    }

    #[test]
    fn test_05_contain_file_1() {
        let file = Path::new("src/current_files/tests/file1.txt");
    
        match get_files(file.to_str().unwrap().to_string()) {
            Ok(files) => assert_eq!(files.contains_key(&file.to_str().unwrap().to_string()), true),
            Err(err) => panic!("Error: {:?}", err),
        }
    }

    #[test]
    fn test_06_not_contain_file_2() {
        let directory = Path::new("src/current_files/tests/folder");
        let file = directory.join("file2.txt");
    
        match get_files(directory.to_str().unwrap().to_string()) {
            Ok(files) => assert_eq!(files.contains_key(&file.to_str().unwrap().to_string()), false),
                
            Err(err) => panic!("Error: {:?}", err),
        }
    }

    #[test]
    fn test_07_not_contain_file_4() {
        let directory = Path::new("src/current_files/tests/folder");
        let file = directory.join("file4.txt");
    
        match get_files(directory.to_str().unwrap().to_string()) {
            Ok(files) => assert_eq!(files.contains_key(&file.to_str().unwrap().to_string()), false),
            Err(err) => panic!("Error: {:?}", err),
        }
    }

    #[test]
    fn test_08_contain_two_files() {
        let directory = Path::new("src/current_files/tests/folder");
    
        match get_files(directory.to_str().unwrap().to_string()) {
            Ok(files) => assert_eq!(files.len(), 2),
            Err(err) => panic!("Error: {:?}", err),
        }
    }

    #[test]
    fn test_09_contain_four_files() {
        let directory = Path::new("src/current_files/tests");
    
        match get_files(directory.to_str().unwrap().to_string()) {
            Ok(files) => assert_eq!(files.len(), 6),
            Err(err) => panic!("Error: {:?}", err),
        }
    }

    #[test]
    fn test_10_contain_one_file() {
        let directory = Path::new("src/current_files/tests/file1.txt");
    
        match get_files(directory.to_str().unwrap().to_string()) {
            Ok(files) => assert_eq!(files.len(), 1),
            Err(err) => panic!("Error: {:?}", err),
        }
    }

    #[test]
    fn test_11_contain_file_1() {
        let file = Path::new("src/current_files/tests/file1.txt");
    
        match get_files(file.to_str().unwrap().to_string()) {
            Ok(files) => assert_eq!(files.contains_key(&file.to_str().unwrap().to_string()), true),
            Err(err) => panic!("Error: {:?}", err),
        }
    }

}
