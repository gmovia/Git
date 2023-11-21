use std::{path::{Path, PathBuf}, collections::HashSet, fs::{self, File}, io::{Read, self}};

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration, path::Path, fs};

    use rust_git::{servers::server::Server, handlers::clone::handler_clone, vcs::version_control_system::VersionControlSystem};

    use crate::vcs::clone::compare_directories;

    #[test]
    pub fn test_01_run_server_for_command_clone()-> Result<(), std::io::Error>{ 
        let client_path = Path::new("tests/clone");
        let server_path = Path::new("tests/test_folder");
        let _ = thread::spawn( || {
            let _ = Server::server("tests/test_folder".to_string());
        });
        let _ = VersionControlSystem::init(client_path, Vec::new());
        let _ = handler_clone("git clone tests/clone".to_string());
        assert!(
            compare_directories(&client_path.join(".rust_git").join("objects"), &server_path.join("tests").join("clone").join(".rust_git").join("objects"))?,
            "Los directorios no son iguales"
        );
        fs::remove_dir_all(client_path)?;
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