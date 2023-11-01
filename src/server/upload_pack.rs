use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::net::TcpStream;
use std::io;

pub fn handler_upload_pack(message: String, reader: &mut TcpStream, path: &PathBuf) -> Result<String, std::io::Error> {
    let logs_path = path.join(".rust_git").join("logs");

    if logs_path.is_dir() {
        let log_entries = get_log_entries(&logs_path)?;

        println!("{}", log_entries);

    }

    Ok("listo".to_string())
}

fn get_log_entries(logs_path: &Path) -> io::Result<String> {
    let mut log_entries = String::new();

    let entries = fs::read_dir(logs_path)?;
    for entry in entries {
        let log_file = entry?;
        let file = fs::File::open(log_file.path())?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            if let Some(hash) = parse_log_line(&line) {
                let filename = log_file.file_name().to_string_lossy().to_string();
                log_entries.push_str(&format!("{} refs/heads/{}\n", hash, filename));
            }
        }
    }

    Ok(log_entries)
}

fn parse_log_line(line: &str) -> Option<String> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 1 {
        let hash_parts: Vec<&str> = parts[0].split('-').collect();
        if hash_parts.len() == 2 && hash_parts[1].len() == 40 {
            Some(hash_parts[1].to_string())
        } else {
            None
        }
    } else {
        None
    }
}

