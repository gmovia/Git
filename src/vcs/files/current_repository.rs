use std::{path::{PathBuf, Path}, fs::OpenOptions, io::{self, BufRead}};
use crate::constants::constants::CURRENT_REPOSITORY_PATH;

pub struct CurrentRepository;

impl CurrentRepository{
    pub fn read() -> Result<PathBuf, std::io::Error>{
        let current_path = Path::new(CURRENT_REPOSITORY_PATH);
        let repo_file = OpenOptions::new().read(true).open(&current_path)?;
        let reader = io::BufReader::new(repo_file);
        if let Some(current) = reader.lines().filter_map(Result::ok).last() {
            return Ok(Path::new(&current).to_path_buf());
        }
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "El archivo no existe",))
    }
}