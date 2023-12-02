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
        let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

        let log = process_lines(&lines, config)?;

        Ok(log)
    }
}
fn process_lines(lines: &[String], config: (String, String)) -> Result<String, std::io::Error> {
    let mut log = String::new();
    for line in lines.iter().rev() {
        process_line(line, &mut log, config.clone())?;
    }
    Ok(log)
}

fn process_line(line: &str, log: &mut String, config: (String, String)) -> Result<(), std::io::Error> {
    let parts: Vec<&str> = line.splitn(6, '-').collect();
    let (_ , tree_hash, message, date_str) = (parts[0], parts[2], parts[3], parts[4].trim_matches(|c| c == '[' || c == ']'));
    let date_time = date_str.split('.').next();

    if let Some(date_time) = date_time {
        let date_time = date_time.trim_end();

        let timestamp = date_time.trim().parse::<i64>().map_err(|e| {
            println!("Error parsing timestamp: {}", e);
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid timestamp")
        })?;
        
        if let Some(parsed_date_time) = chrono::NaiveDateTime::from_timestamp_opt(timestamp, 0) {
            let formatted_date = parsed_date_time.format("%a %b %d %T %Y");
        
            log.push_str(&format!(" {} {} ","\n commit:",tree_hash));
            log.push_str(&format!("\n Author: {} <{}>",config.0,config.1));
            log.push_str(&format!("\n {} {} ","Date:",formatted_date));
            log.push_str(&format!("\n {} {} ","   message: ",message));
            log.push_str("\n ~  ~  ~  ~  ~  ~  ~  ~  ~  ~  ~  ~  ~  ~  ~  ~  ~  ~  ~  ~  ~");
        } else {
            println!("Error creating NaiveDateTime from timestamp {}", timestamp);
        }
    } else {
        println!("Error extracting date and time from line: {}", line);
    }

    Ok(())
}
