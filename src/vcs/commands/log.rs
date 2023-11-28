use std::{io::{BufReader, BufRead}, fs::File};
use crate::vcs::files::{current_repository::CurrentRepository, config::Config};
use super::init::Init;

pub struct Log;

impl Log {

    pub fn log() -> Result<String, std::io::Error> {
        let current = CurrentRepository::read()?;
        let commits_file = File::open(Init::get_current_log(&current)?)?;
        let reader = BufReader::new(commits_file);
        let config = Config::read_config()?;
        let mut lines: Vec<String> = Vec::new();
        
        for line in reader.lines() {
            lines.push(line?);
        }

        let mut log = String::new();
        for line in lines.iter().rev() {
            let parts: Vec<&str> = line.splitn(5, '-').collect();
            let (_ , tree_hash, message, date_str) = (parts[0], parts[2], parts[3], parts[4].trim_matches(|c| c == '[' || c == ']'));
            let date_time = date_str.split('.').next().unwrap();

            if let Ok(parsed_date_time) = chrono::NaiveDateTime::parse_from_str(date_time, "%Y-%m-%d %H:%M:%S") {
                let formatted_date = parsed_date_time.format("%a %b %d %T %Y");

                log.push_str(&format!(" {} {} ","\n commit:",tree_hash));
                log.push_str(&format!("\n Author: {} <{}>",config.0,config.1));
                log.push_str(&format!("\n {} {} ","Date:",formatted_date));
                log.push_str(&format!("\n {} {} ","   message: ",message));
                log.push_str("\n ~  ~  ~  ~  ~  ~  ~  ~  ~  ~  ~  ~  ~  ~  ~  ~  ~  ~  ~  ~  ~  ~  ~  ~");
            }else {
                println!("Error analyzing date {}", date_time);
            }
        }
        Ok(log)
    }
}