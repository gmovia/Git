#[cfg(test)]
mod tests {
    use std::path::Path;
    use rust_git::staging_area::StagingArea;

    #[test]
    #[ignore]
    fn test_01_add_file() {
        let mut staging_area = StagingArea::new();
        let path = Path::new("tests/utils/files/file1.txt");

        let _ = staging_area.add(path);
        assert_eq!(staging_area.area.len(), 1);
    }

    #[test]
    fn test_02_add_file() {
        let mut staging_area = StagingArea::new();
        let path = Path::new("tests/utils/files/file1.txt");

        let _ = staging_area.add(path);
        assert!(staging_area
            .area
            .contains_key("tests/utils/files/file1.txt"));
    }

    #[test]
    fn test_03_add_folder() {
        let mut staging_area = StagingArea::new();
        let path = Path::new("tests/utils/files");

        let _ = staging_area.add(path);
        assert_eq!(staging_area.area.len(), 6);
    }

    #[test]
    fn test_04_add_folder() {
        let mut staging_area = StagingArea::new();
        let path = Path::new("tests/utils/files");

        let _ = staging_area.add(path);
        assert!(staging_area.area.contains_key("tests/utils/files"));
    }

    #[test]
    fn test_05_add_folder() {
        let mut staging_area = StagingArea::new();
        let path = Path::new("tests/utils/files");

        let _ = staging_area.add(path);
        assert!(staging_area
            .area
            .contains_key("tests/utils/files/file1.txt"));
    }
    /*

    #[test]
    #[ignore]
    fn test_06_remove_file() {
        let staging_area = StagingArea::new();
        staging_area.add(file_path);

        staging_area.remove(file_path)
        assert_eq!(staging_area.area.len(), 0);
    }

    #[test]
    #[ignore]
    fn test_07_remove_file() {
        let staging_area = StagingArea::new();
        staging_area.add(file_path);

        staging_area.remove(file_path);
        assert!(!staging_area.include(FILE_PATH));
    }

    #[test]
    #[ignore]
    fn test_08_remove_folder() {
        let staging_area = StagingArea::new();
        staging_area.add(folder_path);

        staging_area.remove(folder_path)
        assert!(staging_area.area.len(), 0);
    }

    #[test]
    #[ignore]
    fn test_09_remove_folder() {
        let staging_area = StagingArea::new();
        staging_area.add(folder_path);

        staging_area.remove(folder_path)
        assert!(!staging_area.include(FOLDER_PATH));
    }

    #[test]
    #[ignore]
    fn test_10_remove_folder() {
        let staging_area = StagingArea::new();
        staging_area.add(folder_path);

        staging_area.remove(folder_path)
        assert!(!staging_area.include(FILE_PATH));
    }
    */
}
