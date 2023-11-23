#[cfg(test)]
mod tests {
    use std::{thread, path::Path, fs};

    use rust_git::{servers::server::Server, handlers::clone::handler_clone, vcs::{version_control_system::VersionControlSystem, commands::branch::BranchOptions}};

    use crate::tests_functions::{commit_one_file, compare_directories, get_number_of_elements, commit_one_folder};

    #[test]
    pub fn test_01_test_clone_with_1_file_1_commit()-> Result<(), std::io::Error>{ 
        let client_path = Path::new("tests/clone");
        let server_path = Path::new("tests/test_folder");
        let folder_to_clone = thread::spawn( move || {
            let folder = &server_path.join("tests").join("clone");
            VersionControlSystem::init( folder, Vec::new());
            commit_one_file(server_path.to_path_buf(), "test_file_1.txt");
        });
        let _ = folder_to_clone.join();
        let _ = thread::spawn( || {
            let _ = Server::server("tests/test_folder".to_string());
        });
        let _ = VersionControlSystem::init(client_path, Vec::new());
        let _ = handler_clone("git clone tests/clone".to_string());
        assert!(
            compare_directories(&client_path.join(".rust_git").join("objects"), &server_path.join("tests").join("clone").join(".rust_git").join("objects"))?,
            "Los directorios no son iguales"
        );
        let server = get_number_of_elements(server_path.join("tests").join("clone"));
        let client = get_number_of_elements(client_path.to_path_buf());
        assert_eq!(server,client);
        fs::remove_dir_all(client_path)?;
        fs::remove_dir_all(server_path)?;
        Ok(())
    }

    #[test]
    pub fn test_02_test_clone_with_2_file_2_commit()-> Result<(), std::io::Error>{ 
        let client_path = Path::new("tests/clone");
        let server_path = Path::new("tests/test_folder");
        let folder_to_clone = thread::spawn( move || {
            let folder = &server_path.join("tests").join("clone");
            VersionControlSystem::init( folder, Vec::new());
            commit_one_file(server_path.to_path_buf(), "test_file_1.txt");
            commit_one_file(server_path.to_path_buf(), "test_file_2.txt");
        });
        let _ = folder_to_clone.join();
        let _ = thread::spawn( || {
            let _ = Server::server("tests/test_folder".to_string());
        });
        let _ = VersionControlSystem::init(client_path, Vec::new());
        let _ = handler_clone("git clone tests/clone".to_string());
        assert!(
            compare_directories(&client_path.join(".rust_git").join("objects"), &server_path.join("tests").join("clone").join(".rust_git").join("objects"))?,
            "Los directorios no son iguales"
        );
        let server = get_number_of_elements(server_path.join("tests").join("clone"));
        let client = get_number_of_elements(client_path.to_path_buf());
        assert_eq!(server,client);
        fs::remove_dir_all(client_path)?;
        fs::remove_dir_all(server_path)?;
        Ok(())
    }


    #[test]
    pub fn test_03_test_clone_with_2_file_1_folder_3_commit()-> Result<(), std::io::Error>{ 
        let client_path = Path::new("tests/clone");
        let server_path = Path::new("tests/test_folder");
        let folder_to_clone = thread::spawn( move || {
            let folder = &server_path.join("tests").join("clone");
            VersionControlSystem::init( folder, Vec::new());
            commit_one_file(server_path.to_path_buf(), "test_file_1.txt");
            commit_one_file(server_path.to_path_buf(), "test_file_2.txt");
            commit_one_folder(server_path.to_path_buf(),"folder", "test_file_3.txt");
        });
        let _ = folder_to_clone.join();
        let _ = thread::spawn( || {
            let _ = Server::server("tests/test_folder".to_string());
        });
        let _ = VersionControlSystem::init(client_path, Vec::new());
        let _ = handler_clone("git clone tests/clone".to_string());
        assert!(
            compare_directories(&client_path.join(".rust_git").join("objects"), &server_path.join("tests").join("clone").join(".rust_git").join("objects"))?,
            "Los directorios no son iguales"
        );
        let server = get_number_of_elements(server_path.join("tests").join("clone"));
        let client = get_number_of_elements(client_path.to_path_buf());
        assert_eq!(server,client);
        fs::remove_dir_all(client_path)?;
        fs::remove_dir_all(server_path)?;
        Ok(())
    }

    #[test]
    pub fn test_04_test_clone_with_2_files_2_branches_2_commit()-> Result<(), std::io::Error>{ 
        let client_path = Path::new("tests/clone");
        let server_path = Path::new("tests/test_folder");
        let folder_to_clone = thread::spawn( move || {
            let folder = &server_path.join("tests").join("clone");
            VersionControlSystem::init( folder, Vec::new());
            commit_one_file(server_path.to_path_buf(), "test_file_1.txt");
            let _ = VersionControlSystem::branch(BranchOptions::NewBranch("test"));
            commit_one_file(server_path.to_path_buf(), "test_file_2.txt");
        });
        let _ = folder_to_clone.join();
        let _ = thread::spawn( || {
            let _ = Server::server("tests/test_folder".to_string());
        });
        let _ = VersionControlSystem::init(client_path, Vec::new());
        let _ = handler_clone("git clone tests/clone".to_string());
        assert!(
            compare_directories(&client_path.join(".rust_git").join("objects"), &server_path.join("tests").join("clone").join(".rust_git").join("objects"))?,
            "Los directorios no son iguales"
        );
        let server = get_number_of_elements(server_path.join("tests").join("clone"));
        let client = get_number_of_elements(client_path.to_path_buf());
        assert_eq!(server,client);
        fs::remove_dir_all(client_path)?;
        fs::remove_dir_all(server_path)?;
        Ok(())
    }


}
