#[cfg(test)]
mod tests {
    use std::{fs, path::Path, thread};

    use rust_git::{
        handlers::{clone::handler_clone, pull::handler_pull},
        servers::server::Server,
        vcs::version_control_system::VersionControlSystem,
    };

    use crate::tests_functions::{commit_one_file, get_number_of_elements};

    #[test]
    pub fn test_01_test_pull_with_2_file_2_commit() -> Result<(), std::io::Error> {
        let client_path = Path::new("tests/clone");
        let server_path = Path::new("tests/test_folder");
        let folder_to_clone = thread::spawn(move || {
            let folder = &server_path.join("tests").join("clone");
            VersionControlSystem::init(folder, Vec::new());
            commit_one_file(server_path.to_path_buf(), "test_file_1.txt");
        });
        let _ = folder_to_clone.join();
        let _ = thread::spawn(|| {
            let _ = Server::server("tests/test_folder".to_string());
        });
        let clone = thread::spawn(move || {
            VersionControlSystem::init(client_path, Vec::new());
            let _ = handler_clone("git clone tests/clone".to_string());
        });
        let _ = clone.join();
        let folder_to_fetch = thread::spawn(move || {
            let folder = &server_path.join("tests").join("clone");
            VersionControlSystem::init(folder, Vec::new());
            commit_one_file(server_path.to_path_buf(), "test_file_2.txt");
        });
        let _ = folder_to_fetch.join();

        let _ = VersionControlSystem::init(client_path, Vec::new());
        let _ = handler_pull("git pull origin".to_string());
        let server = get_number_of_elements(server_path.join("tests").join("clone"));
        let client = get_number_of_elements(client_path.to_path_buf());
        assert_eq!(server, client);
        fs::remove_dir_all(client_path)?;
        fs::remove_dir_all(server_path)?;
        Ok(())
    }
}
