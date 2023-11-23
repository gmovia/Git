use std::{path::{Path, PathBuf}, fs::{self, File, create_dir_all}, io::{Read, self, Write}};

#[cfg(test)]
mod tests {
    use std::{thread, path::Path, fs};

    use rust_git::{servers::server::Server, handlers::{clone::handler_clone, fetch::handler_fetch, pull::handler_pull}, vcs::{version_control_system::VersionControlSystem, commands::branch::BranchOptions}};

    use crate::vcs::pull::{commit_one_file, get_number_of_elements};

    #[test]
    pub fn test_01_test_pull_with_2_file_2_commit()-> Result<(), std::io::Error>{ 
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
        });
        let _ = clone.join();
        let folder_to_fetch = thread::spawn( move || {
            let folder = &server_path.join("tests").join("clone");
            VersionControlSystem::init( folder, Vec::new());
            commit_one_file(server_path.to_path_buf(), "test_file_2.txt");
        });
        let _ = folder_to_fetch.join();
        
        let _ = VersionControlSystem::init(client_path, Vec::new());
        let _ = handler_pull();
        let server = get_number_of_elements(server_path.join("tests").join("clone"));
        let client = get_number_of_elements(client_path.to_path_buf());
        assert_eq!(server,client);
        fs::remove_dir_all(client_path)?;
        fs::remove_dir_all(server_path)?;
        Ok(())
    }

}


pub fn commit_one_file(server_path: PathBuf, file_name: &str) {
    let folder = &server_path.join("tests").join("clone");
    let _ = create_dir_all(folder);
    let test_file_path = folder.join(file_name);
    if let Ok(mut archivo) = File::create(test_file_path) {
        let _ = archivo.write_all(format!("Archivo para hacer prueba de clone: {}", file_name ).as_bytes());
        let _ = rust_git::vcs::version_control_system::VersionControlSystem::add(&server_path);
        let _ = rust_git::vcs::version_control_system::VersionControlSystem::commit(format!("{} commit", file_name));
    }    
}

pub fn commit_one_folder(server_path: PathBuf, folders: &str, file_name: &str) {
    let folder = &server_path.join("tests").join("clone").join(folders);
    let _ = create_dir_all(folder);
    let test_file_path = folder.join(file_name);
    if let Ok(mut archivo) = File::create(test_file_path) {
        let _ = archivo.write_all(format!("Archivo para hacer prueba de clone: {}", file_name ).as_bytes());
        let _ = rust_git::vcs::version_control_system::VersionControlSystem::add(&server_path);
        let _ = rust_git::vcs::version_control_system::VersionControlSystem::commit(format!("{} commit", file_name));
    }    
}

fn get_number_of_elements(path: PathBuf) -> usize {
    let mut archivos = 0;
    let mut directorios = 0;

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        archivos += 1;
                    } else if metadata.is_dir() {
                        directorios += 1;
                    }
                }
            }
        }
    }
    archivos + directorios
}