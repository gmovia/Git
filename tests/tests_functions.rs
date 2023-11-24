use rust_git::{vcs::files::vcs_file::VCSFile, vcs::version_control_system::VersionControlSystem};
use std::{collections::HashMap, path::{Path, PathBuf}, fs::{File, self}};
use tempdir::TempDir;

pub fn equals(staging_area: HashMap<String, VCSFile>, path: &Path, state: &str) -> bool {
    if let Some(value) = staging_area.get(&path.display().to_string()) {
        return value.state == state.to_string();
    }
    false
}

pub fn set_up() -> TempDir{
    let temp_dir = TempDir::new("test_version_control_system").expect("Failed to create temp directory");
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