use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

use crate::constants::constant::CONFIG_PATH;

pub struct Config;

impl Config {
    pub fn write_config(config: (String, String)) -> Result<(), std::io::Error> {
        let config_path = Path::new(CONFIG_PATH);
        let mut config_file = OpenOptions::new().write(true).open(config_path)?;

        config_file
            .write_all(format!("user.name= {}\nuser.email= {}", config.0, config.1).as_bytes())?;

        Ok(())
    }

    pub fn read_config() -> Result<(String, String), std::io::Error> {
        let config_path = Path::new(CONFIG_PATH);

        let config = fs::read_to_string(config_path)?;

        if config.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Config file is empty",
            ));
        }
        let parts: Vec<&str> = config.split('\n').collect();

        let email: Vec<&str> = parts[0].split_whitespace().collect();
        let name: Vec<&str> = parts[1].split_whitespace().collect();

        let configuration = (email[1].to_string(), name[1].to_string());

        Ok(configuration)
    }
}
