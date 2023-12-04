#[cfg(test)]
mod tests {
    use std::{path::Path, thread, time::Duration, fs};

    use rust_git::{vcs::{version_control_system::VersionControlSystem, commands::remote::{Remote, RemoteOption}}, handlers::clone::handler_clone, servers::server::Server};

    use crate::tests_functions::{commit_one_file, compare_directories, commit_one_file_client};

    #[test]
    pub fn test_01_remote_1_file() -> Result<(), std::io::Error> {
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

            let other_server = thread::spawn( move || {
                VersionControlSystem::init( &server_path.join("tests2").join("clone2"), Vec::new());
            });
            let _ = other_server.join();

            let clone = thread::spawn( move || {
                VersionControlSystem::init( client_path, Vec::new());
                let _ = handler_clone("git clone tests/clone".to_string());

                let _ = Remote::remote(client_path, RemoteOption::Add("repo2", "tests2/clone2".to_string().as_str()));

                commit_one_file_client(client_path.to_path_buf(), "test_file_client.txt");
                let _ = VersionControlSystem::push("git push repo2\n".to_string());
            });
            let _ = clone.join();

            thread::sleep(Duration::from_secs_f64(0.5));
            assert!(
                compare_directories(&client_path.join(".rust_git").join("objects"), &server_path.join("tests2").join("clone2").join(".rust_git").join("objects"))?,
                "Los directorios no son iguales"
            );
            fs::remove_dir_all(client_path)?;
            fs::remove_dir_all(server_path)?;
            Ok(())
        }


}