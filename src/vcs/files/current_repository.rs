use crate::constants::constant::CURRENT_REPOSITORY_PATH;
use std::{
    fs::OpenOptions,
    io::{self, BufRead},
    path::{Path, PathBuf},
};

pub struct CurrentRepository;

impl CurrentRepository {
    /// Obtiene el repositorio actual
    pub fn read() -> Result<PathBuf, std::io::Error> {
        let current_path = Path::new(CURRENT_REPOSITORY_PATH);
        let repo_file = OpenOptions::new().read(true).open(current_path)?;
        let reader = io::BufReader::new(repo_file);
        if let Some(current) = reader.lines().map_while(Result::ok).last() {
            return Ok(Path::new(&current).to_path_buf());
        }
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "El archivo no existe",
        ))
    }
}
