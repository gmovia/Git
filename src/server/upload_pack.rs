use std::fs;
use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use std::net::TcpStream;
pub fn handler_upload_pack(message: String, reader: &mut TcpStream, path: &PathBuf) -> Result<String, std::io::Error> {
    let logs_path = path.join(".rust_git").join("logs");
    let log_entries = get_log_entries(&logs_path)?;
    Ok(log_entries)
}

fn get_log_entries(logs_path: &Path) -> Result<String, std::io::Error>{
    let mut log_entries = String::new();

    let entries = fs::read_dir(logs_path)?;
    for entry in entries {
        let log_file = entry?;
        let file = fs::File::open(log_file.path())?;
        
        let mut reader = BufReader::new(file);
        let mut last_line = String::new();

        for line in reader.by_ref().lines() {
            if let Ok(line) = line {
                last_line = line;
            }
        }
        if let Some(hash) = parse_log_line(&last_line) {
            let filename = log_file.file_name().to_string_lossy().to_string();
            log_entries.push_str(&format!("{} refs/heads/{}\n", hash, filename));
        }
    }

    Ok(log_entries)
}


fn parse_log_line(line: &str) -> Option<String> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    match parts.get(0) {
        Some(part) => {
            let part = part.replace("--m", "");
            let hash_parts: Vec<&str> = part.splitn(2, '-').collect(); 

            match hash_parts.as_slice() {
                [_, hash] if hash.len() == 40 => {
                    let hash = hash_parts[0..2].join("");
                    Some(hash)
                }
                _ => None,
            }
        }
        None => None,
    }
}


