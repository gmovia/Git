use rust_git::{vcs::files::vcs_file::VCSFile, vcs::version_control_system::VersionControlSystem};
use std::{collections::HashMap, path::{Path, PathBuf}, fs::File};
use tempdir::TempDir;

pub fn equals(staging_area: HashMap<String, VCSFile>, path: &Path, state: &str) -> bool {
    if let Some(value) = staging_area.get(&path.display().to_string()) {
        return value.state == state.to_string();
    }
    false
}

pub fn set_up() -> (TempDir, VersionControlSystem){
    let temp_dir = TempDir::new("test_version_control_system").expect("Failed to create temp directory");
    let vsc = VersionControlSystem::init(temp_dir.path().to_string_lossy().to_string());
    (temp_dir, vsc)
}

pub fn create_file(temp_dir: &TempDir, filename: &str) -> PathBuf {
    let file_path = temp_dir.path().join(filename);
    let _ = File::create(&file_path).expect("Failed to create file");
    file_path
}

pub fn status_contains(result: HashMap<String, String>, status: &str, file: &PathBuf) -> bool {
    if let Some(value) = result.get(&file.display().to_string()) {
        return value.as_str() == status;
    }
    false
}