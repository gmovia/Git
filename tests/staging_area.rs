#[cfg(test)]
mod tests {
    use std::path::Path;
    use rust_git::staging_area::StagingArea;

    #[test]
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
        assert_eq!(staging_area.area.len(), 4);
    }

    #[test]
    fn test_04_add_folder() {
        let mut staging_area = StagingArea::new();
        let path = Path::new("tests/utils/files");

        let _ = staging_area.add(path);
        assert!(!staging_area.area.contains_key("tests/utils/files"));
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

    #[test]
    fn test_06_add_folder() {
        let mut staging_area = StagingArea::new();
        let path = Path::new("tests/utils/noexiste");

        let _ = staging_area.add(path);
        assert!(staging_area
            .area
            .len() == 0);
    }
}
