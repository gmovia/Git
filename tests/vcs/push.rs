#[cfg(test)]
mod tests {
    use std::{path::Path, thread, fs};

    use rust_git::{vcs::version_control_system::VersionControlSystem, handlers::{clone::handler_clone, tag::handler_tag}, servers::server::Server};

    use crate::tests_functions::{commit_one_file, compare_directories};

    #[test]
    pub fn test_01_push_1_file() -> Result<(), std::io::Error> {
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


            let clone = thread::spawn( move || {
                VersionControlSystem::init( client_path, Vec::new());
                let _ = handler_clone("git clone tests/clone".to_string());
                commit_one_file(client_path.to_path_buf(), "test_file.txt");
                //let _ = VersionControlSystem::push("git push origin".to_string());
            });
            let _ = clone.join();

            println!("CLIENT PATH: {:?}", client_path);
            println!("SERVER PATH: {:?}", server_path);
            assert!(
                compare_directories(&client_path.join(".rust_git").join("objects"), &server_path.join("tests").join("clone").join(".rust_git").join("objects"))?,
                "Los directorios no son iguales"
            );
            //fs::remove_dir_all(client_path)?;
            //fs::remove_dir_all(server_path)?;
            Ok(())
        }


        #[test]
        pub fn test_02_push_1_tag() -> Result<(), std::io::Error> {
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
                let clone = thread::spawn( move || {
                    VersionControlSystem::init( client_path, Vec::new());
                    let _ = handler_clone("git clone tests/clone".to_string());
                    commit_one_file(client_path.to_path_buf(), "test_file.txt");
                    handler_tag("git tag -a version2 hola".to_string());
                    let _ = VersionControlSystem::push("git push origin".to_string());
                });
                let _ = clone.join();
    
                let _ = VersionControlSystem::init(server_path, Vec::new());
                println!("CLIENT PATH: {:?}", client_path);
                println!("SERVER PATH: {:?}", server_path);
                assert!(
                    compare_directories(&client_path.join(".rust_git").join("objects"), &server_path.join("tests").join("clone").join(".rust_git").join("objects"))?,
                    "Los directorios no son iguales"
                );
                fs::remove_dir_all(client_path)?;
                fs::remove_dir_all(server_path)?;
                Ok(())
            }

}