use std::{fs::{OpenOptions, self}, io::Write};

use super::current_repository::CurrentRepository;


#[derive(Debug, Clone)]
pub struct Log;

impl Log {

    pub fn write_log(log: &String) -> Result<(), std::io::Error>{
        let current = CurrentRepository::read()?;
        let log_path = current.join(".rust_git").join("info").join("log");
        let mut log_file = OpenOptions::new().write(true).append(true).open(log_path)?;

        log_file.write_all(log.as_bytes())?;

        Ok(())
    }

    pub fn read_log() -> Result<String, std::io::Error>{
        let current = CurrentRepository::read()?;
        let log_path = current.join(".rust_git").join("info").join("log");

        let logs = fs::read_to_string(log_path)?;

        if logs.is_empty() {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Log's file is empty"));
        }

        Ok(logs)
    }

}