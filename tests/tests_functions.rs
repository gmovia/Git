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
    // let repositories_path = temp_dir.path().join(BDD_PATH);
    // let current_repository_path = temp_dir.path().join(CURRENT_REPOSITORY_PATH);
    // let _ = create_file(&temp_dir, &repositories_path.to_string_lossy());
    // let _ = create_file(&temp_dir, &current_repository_path.to_string_lossy());
    //let mut vcs = VersionControlSystem::new();
    VersionControlSystem::init(temp_dir.path(), Vec::new());
    //let vsc = VersionControlSystem::init(temp_dir.path(), Vec::new());
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

pub fn status_contains(result: HashMap<String, String>, status: &str, file: &PathBuf) -> bool {
    if let Some(value) = result.get(&file.display().to_string()) {
        return value.as_str() == status;
    }
    false
}