use rust_git::{vcs::files::vcs_file::VCSFile, vcs::{version_control_system::VersionControlSystem, files::config::Config}};
use std::{collections::HashMap, path::{Path, PathBuf}, fs::{File, self, create_dir_all}, io::{Read, self, Write}};
use tempdir::TempDir;

pub fn equals(staging_area: HashMap<String, VCSFile>, path: &Path, state: &str) -> bool {
    if let Some(value) = staging_area.get(&path.display().to_string()) {
        return value.state == state.to_string();
    }
    false
}

pub fn set_up() -> TempDir{
    let temp_dir = TempDir::new("test_version_control_system").expect("Failed to create temp directory");
    let _ = Config::write_config(("temp_dir_name".to_string(),"temp_dir_email".to_string()));
    VersionControlSystem::init(temp_dir.path(), Vec::new());
    temp_dir
}

pub fn create_file(temp_dir: &TempDir, filename: &str) -> PathBuf {
    let file_path = temp_dir.path().join(filename);
    let _ = File::create(&file_path).expect("Failed to create file");
    file_path
}

pub fn create_dir(temp_dir: &TempDir, dirname: &str) -> PathBuf {
    let dir_path = temp_dir.path().join(dirname);
    fs::create_dir_all(&dir_path).expect("Failed to create directory");
    dir_path
}

pub fn status_contains(result: HashMap<String, String>, status: &str, file: &Path) -> bool {
    if let Some(value) = result.get(&file.display().to_string()) {
        return value.as_str() == status;
    }
    false
}

pub fn compare_files(file1: &Path, file2: &Path) -> io::Result<bool> {
    let mut buf1 = Vec::new();
    let mut buf2 = Vec::new();

    File::open(file1)?.read_to_end(&mut buf1)?;
    File::open(file2)?.read_to_end(&mut buf2)?;
    println!("ENTRY 1: {}, ENTRY 2: {}", String::from_utf8_lossy(&buf1), String::from_utf8_lossy(&buf2));
    Ok(buf1 == buf2)
}

pub fn compare_directories(dir1: &Path, dir2: &Path) -> io::Result<bool> {
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

pub fn commit_one_file_client(client_path: PathBuf, file_name: &str) {
    let _ = create_dir_all(&client_path);
    let test_file_path = client_path.join(file_name);
    if let Ok(mut archivo) = File::create(test_file_path) {
        let _ = archivo.write_all(format!("Archivo para hacer prueba de clone: {}", file_name ).as_bytes());
        let _ = rust_git::vcs::version_control_system::VersionControlSystem::add(&client_path);
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


pub fn get_number_of_elements(path: PathBuf) -> usize {
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

pub fn count_files(path: &Path) -> Result<u32, std::io::Error> {
    let mut total = 0;

    if path.is_dir() {
        for entrada in fs::read_dir(path)? {
            let entrada = entrada?;
            let ruta = entrada.path();

            if ruta.is_file() {
                total += 1;
            } else if ruta.is_dir() {
                total += count_files(&ruta)?;
            }
        }
    } else if path.is_file() {
        total = 1;
    }

    Ok(total)
}