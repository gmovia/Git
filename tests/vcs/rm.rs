// #[cfg(test)]
// mod tests {

//     use std::fs::File;

//     use crate::tests_functions::{create_file, set_up, equals, create_dir};

//     #[test]
//     pub fn test_01_if_file_1_was_commited_then_rm_file_1_is_ok() {
//         let (temp_dir, mut vsc) = set_up();
//         let path = create_file(&temp_dir, "file1.txt");      

//         vsc.local_repository.insert(path.to_string_lossy().to_string(), "content".to_string());
        
//         let _ = vsc.rm(&path);
//         assert_eq!(vsc.staging_area.len(), 1);
//     }

//     #[test]
//     pub fn test_02_if_file_1_was_commited_then_rm_file_1_is_ok() {
//         let (temp_dir, mut vsc) = set_up();
//         let path = create_file(&temp_dir, "file1.txt");      

//         vsc.local_repository.insert(path.to_string_lossy().to_string(), "content".to_string());
        
//         let _ = vsc.rm(&path);
//         assert_eq!(equals(vsc.staging_area, &path, "DELETED"), true);
//     }

//     #[test]
//     pub fn test_03_if_file_1_was_commited_then_rm_file_1_is_ok() {
//         let (temp_dir, mut vsc) = set_up();

//         let file1_path = create_file(&temp_dir, "file1.txt");      
//         let file2_path = create_file(&temp_dir, "file2.txt");      

//         vsc.local_repository.insert(file1_path.to_string_lossy().to_string(), "content".to_string());
//         vsc.local_repository.insert(file2_path.to_string_lossy().to_string(), "content".to_string());


//         let _ = vsc.rm(&file1_path);
//         let _ = vsc.rm(&file2_path);

//         assert_eq!(vsc.staging_area.len(), 2);
//     }


//     #[test]
//     pub fn test_04_rm_directory_without_r() {
//         let (temp_dir, mut vsc) = set_up();
//         let dir_path = create_dir(&temp_dir, "carpeta/");      
//         // Intenta eliminar un directorio sin la opción -r
        
//         vsc.local_repository.insert(dir_path.to_string_lossy().to_string(), "content".to_string());
//         let result = vsc.rm(&dir_path);
        
//         assert!(matches!(result, Err(e) if e.to_string().contains("recursively without -r")));    
//     }

//     #[test]
//     pub fn test_05_rm_directory_without_r() {
//         let (temp_dir, mut vsc) = set_up();
//         let dir_path = create_dir(&temp_dir, "carpeta/");      
        
//         // Intenta eliminar un directorio sin la opción -r
//         vsc.local_repository.insert(dir_path.to_string_lossy().to_string(), "content".to_string());
//         let _ = vsc.rm(&dir_path);
        
//         assert_eq!(vsc.staging_area.len(), 0);    
//     }
//     #[test]
//     pub fn test_06_rm_directory_with_r() {
//         let (temp_dir, mut vsc) = set_up();
//         let dir_path = create_dir(&temp_dir, "carpeta/");      
//         let file_path = dir_path.join("file.txt");
//         let _ = File::create(&file_path);
    
//         vsc.local_repository.insert(file_path.to_string_lossy().to_string(), "content".to_string());
//         let _ = vsc.rm_r(&dir_path);
    
//         assert_eq!(vsc.staging_area.len(), 1);    
//     }
    
    
//     #[test]
//     pub fn test_07_rm_remove_file_from_workspace() {
//         let (temp_dir, mut vsc) = set_up();
//         let path = create_file(&temp_dir, "file1.txt");      

//         vsc.local_repository.insert(path.to_string_lossy().to_string(), "content".to_string());
        
//         let _ = vsc.rm(&path);
//         assert!(!path.exists());
//     }

//     #[test]
//     pub fn test_08_rm_remove_only_file_that_was_commited_in_repository() {
//         let (temp_dir, mut vsc) = set_up();
//         let dir_path = create_dir(&temp_dir, "carpeta/");      
//         let file_path = dir_path.join("file.txt");
//         let file_path2 = dir_path.join("file2.txt");
//         let _ = File::create(&file_path);
//         let _ = File::create(&file_path2);

//         vsc.local_repository.insert(file_path.to_string_lossy().to_string(), "content".to_string());
//         let _ = vsc.rm_r(&dir_path);
    
//         assert_eq!(vsc.staging_area.len(), 1);    
//     }

//     #[test]
//     pub fn test_09_rm_remove_two_files_that_were_commited_in_repository() {
//         let (temp_dir, mut vsc) = set_up();
//         let dir_path = create_dir(&temp_dir, "carpeta/");      
//         let file_path = dir_path.join("file.txt");
//         let file_path2 = dir_path.join("file2.txt");
//         let _ = File::create(&file_path);
//         let _ = File::create(&file_path2);

//         vsc.local_repository.insert(file_path.to_string_lossy().to_string(), "content".to_string());
//         vsc.local_repository.insert(file_path2.to_string_lossy().to_string(), "content".to_string());
        
//         let _ = vsc.rm_r(&dir_path);
    
//         assert_eq!(vsc.staging_area.len(), 2);    
//     }


// }