use std::{net::TcpStream, path::Path, io::{Write, self, BufRead}, fs};

use crate::{ packfiles::packfile::to_pkt_line, protocol::send_pack::handle_send_pack, vcs::commands::branch::Branch};

pub struct Push;

impl Push{
    pub fn push(stream: &mut TcpStream, current_repo: &Path) -> Result<(),std::io::Error> {
        println!("ESTOY EN PUSH\n\n");

        let logs_path = current_repo.join(".rust_git").join("logs");
        let log_entries = Self::get_commits_branch(&logs_path)?;
        let mut entry_to_send:Vec<String> = Vec::new();
        println!("LOG entries -----> {:?}\n", log_entries);
        let current_branch:String = Branch::get_current_branch(current_repo)?;
        for entries in &log_entries{
            if entries.contains(&current_branch){
                entry_to_send.push(entries.to_string());
                let ref_to_pkt = to_pkt_line(entries);
                stream.write_all(ref_to_pkt.as_bytes())?;
            }
        }

        println!("Hasta aca desde el cliente le mande las refs que tengo \n");
        handle_send_pack(stream, current_repo, &entry_to_send)?;
        
        Ok(())
    }

    fn extract_old_new_commit(line: String) -> String{
        let parts: Vec<&str> = line.split('-').collect();
        let old_hash = parts[1];
        let new_hash = parts[2];
        let hashes = format!("{} {}", old_hash, new_hash);
        hashes
    }

    fn process_file(file_path: &Path) -> Result<String, std::io::Error> {
        let file = fs::File::open(file_path)?;
        let reader = io::BufReader::new(file);
        let mut last_line = String::new();
    
        for line in reader.lines() {
            last_line = line?;
        }
    
        Ok(last_line)
    }
    
    fn get_commits_branch(path: &Path) -> Result<Vec<String>, std::io::Error> {
        if path.is_dir() {
            let mut refs = Vec::new();
    
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let file_path = entry.path();
    
                if file_path.is_file() {
                    let last_line = Self::process_file(&file_path)?;
                    let hashes = Self::extract_old_new_commit(last_line);
                    println!("ESTOS SON LOS HASHES old new --> {} \n", hashes);
                    let file_name = file_path
                        .file_stem()
                        .and_then(|stem| stem.to_str())
                        .ok_or_else(|| {
                            std::io::Error::new(
                                std::io::ErrorKind::InvalidData,
                                "Invalid file name",
                            )
                        })?;
                    let branch = format!("{} refs/heads/{}", hashes, file_name);
                    refs.push(branch);
                }
            }
            Ok(refs)
        } else {
            eprintln!("La carpeta de logs no existe en la ubicación especificada.");
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "La carpeta de logs no existe",
            ))
        }
    }

    pub fn parse_query_to_extract_path(message: &str) -> Result<&str, std::io::Error> {
        let parts: Vec<&str> = message.split('\0').collect();
        let mut current_repository = "";
        for part in parts {
            if part.starts_with("git-receive-pack /") {
                current_repository = part.trim_start_matches("git-receive-pack /");
                break;
            }
        }
        Ok(current_repository)
    }
    
}