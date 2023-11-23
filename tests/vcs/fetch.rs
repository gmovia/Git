use std::{path::{Path, PathBuf}, fs::{self, File, create_dir_all}, io::{Read, self, Write}};

#[cfg(test)]
mod tests {
    use std::{thread, path::Path, fs};

    use rust_git::{servers::server::Server, handlers::{clone::handler_clone, fetch::handler_fetch}, vcs::{version_control_system::VersionControlSystem, commands::branch::BranchOptions}};

    use crate::vcs::fetch::{compare_directories, commit_one_file, commit_one_folder, get_number_of_elements};

    #[test]
    pub fn test_01_test_fetch_with_2_file_2_commit()-> Result<(), std::io::Error>{ 
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
        let _ = handler_fetch("git fetch".to_string());
        assert!(
            compare_directories(&client_path.join(".rust_git").join("objects"), &server_path.join("tests").join("clone").join(".rust_git").join("objects"))?,
            "Los directorios no son iguales"
        );
        let server = get_number_of_elements(server_path.join("tests").join("clone"));
        let client = get_number_of_elements(client_path.to_path_buf());
        assert_ne!(server,client);
        fs::remove_dir_all(client_path)?;
        fs::remove_dir_all(server_path)?;
        Ok(())
    }

}

fn compare_files(file1: &Path, file2: &Path) -> io::Result<bool> {
    let mut buf1 = Vec::new();
    let mut buf2 = Vec::new();

    File::open(file1)?.read_to_end(&mut buf1)?;
    File::open(file2)?.read_to_end(&mut buf2)?;

    Ok(buf1 == buf2)
}

fn compare_directories(dir1: &Path, dir2: &Path) -> io::Result<bool> {
    let entries1: Vec<PathBuf> = fs::read_dir(dir1)?.map(|entry| entry.unwrap().path()).collect();
    let entries2: Vec<PathBuf> = fs::read_dir(dir2)?.map(|entry| entry.unwrap().path()).collect();

    if entries1.len() != entries2.len() {
        return Ok(false);
    }

    for entry1 in entries1 {
        let entry2 = dir2.join(entry1.file_name().unwrap());

        if entry1.is_dir() {
            if !compare_directories(&entry1, &entry2)? {
                return Ok(false);
            }
        } else {
            if !compare_files(&entry1, &entry2)? {
                return Ok(false);
            }
        }
    }

    Ok(true)
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