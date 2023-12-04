use std::{
    fs::{self},
    path::Path,
};

use crate::vcs::files::current_repository::CurrentRepository;

pub struct CheckIgnore;

impl CheckIgnore {
    /// Recibe una ruta y señala si se encuentra ignorado
    pub fn check_ignore(path: &Path) -> Result<bool, std::io::Error> {
        let current_path = CurrentRepository::read()?;
        let ignore_path = current_path.join(".gitignore");

        let content = fs::read_to_string(ignore_path)?;
        let lines: Vec<&str> = content.lines().collect();

        if let Ok(path) = path.strip_prefix(current_path) {
            if Self::rule_full_path(path, &lines)? {
                return Ok(true);
            }

            if Self::rule_dir(path, &lines)? {
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Ver si matcha con una ruta especifica
    pub fn rule_full_path(path: &Path, ignore_paths: &Vec<&str>) -> Result<bool, std::io::Error> {
        let path = path.to_str();
        for ignore_path in ignore_paths {
            if path == Some(*ignore_path) {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Ver si matchea con un dir especifico
    pub fn rule_dir(path: &Path, ignore_paths: &Vec<&str>) -> Result<bool, std::io::Error> {
        for ignore_path in ignore_paths {
            let ignore_path = Path::new(&ignore_path);
            if path.starts_with(ignore_path) {
                return Ok(true);
            }
        }
        Ok(false)
    }
}
