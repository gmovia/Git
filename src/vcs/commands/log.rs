use std::{
    io::{BufReader, BufRead},
    fs::File,
};
use crate::vcs::version_control_system::VersionControlSystem;

use super::init::Init;

pub struct Log;

impl Log {

    pub fn log(vcs: &VersionControlSystem) -> Result<String, std::io::Error> {
        let commits_file = File::open(Init::get_commits_path(&vcs.path)?)?;
        let reader = BufReader::new(commits_file);
        let mut lines: Vec<String> = Vec::new();
        
        for line in reader.lines() {
            lines.push(line?);
        }

        let mut log = String::new();
        for line in lines.iter().rev() {
            let parts: Vec<&str> = line.splitn(4, '-').collect();
            let (_ , hash, message, date_str) = (parts[0], parts[1], parts[2], parts[3].trim_matches(|c| c == '[' || c == ']'));
            let date_time = date_str.split('.').next().unwrap();

            if let Ok(parsed_date_time) = chrono::NaiveDateTime::parse_from_str(date_time, "%Y-%m-%d %H:%M:%S") {
                let formatted_date = parsed_date_time.format("%a %b %d %T %Y");

                log.push_str(&format!(" {} {} ","\n commit:",hash));
                log.push_str(&"\n Author: ldiazc <ldiazc@fi.uba.ar>");
                log.push_str(&format!("\n {} {} ","Date:",formatted_date));
                log.push_str(&format!("\n {} {} ","   message: ",message));
                log.push_str("\n ~  ~  ~  ~  ~  ~  ~  ~  ~  ~  ~  ~  ~  ~  ~  ~  ~  ~  ~  ~  ~  ~  ~  ~");
                println!("\n commit: {} ", hash);
                println!("Author: ldiazc <ldiazc@fi.uba.ar>");
                println!("Date: {}", formatted_date);
                println!("\n    {}", message);
            }else {
                println!("Error analyzing date {}", date_time);
            }
        }
        Ok(log)
    }
}
