use std::{
    fs,
    io::{self, BufRead, Write},
    net::TcpStream,
    path::Path,
};

use crate::{
    packfiles::packfile::to_pkt_line, protocol::send_pack::handle_send_pack,
    servers::upload_pack::process_tag_content, vcs::commands::branch::Branch,
};

pub struct Push;

/// Realiza la operación de "push" en un sistema de control de versiones Git.
/// # Arguments
/// * `stream` - Referencia mutable al flujo TcpStream para la comunicación con el servidor remoto.
/// * `current_repo` - Ruta al directorio del repositorio actual.
impl Push {
    pub fn push(stream: &mut TcpStream, current_repo: &Path) -> Result<(), std::io::Error> {
        let logs_path = current_repo.join(".rust_git").join("logs");
        let mut log_entries = Self::get_commits_branch(&logs_path)?;
        let mut tag_entries = Self::get_tags(current_repo)?;
        let mut entry_to_send: Vec<String> = Vec::new();

        if !tag_entries.is_empty() {
            log_entries.append(&mut tag_entries);
        }
        let current_branch: String = Branch::get_current_branch(current_repo)?;
        for entries in &log_entries {
            let entry: Vec<&str> = entries.split_whitespace().collect();

            if entry.len() == 3 {
                let refs_name: Vec<&str> = entry[2].split('/').collect();
                if refs_name[2].trim_end_matches('\n') == current_branch.trim_end_matches('\n') {
                    entry_to_send.push(entries.to_string());
                    let ref_to_pkt = to_pkt_line(entries);
                    stream.write_all(ref_to_pkt.as_bytes())?;
                }
            } else {
                entry_to_send.push(entries.to_string());
                let ref_to_pkt = to_pkt_line(entries);
                stream.write_all(ref_to_pkt.as_bytes())?;
            }
        }
        handle_send_pack(stream, current_repo, &entry_to_send)?;
        Ok(())
    }

    /// Obtiene las entradas de tags del repositorio en la ruta especificada.
    fn get_tags(path: &Path) -> Result<Vec<String>, std::io::Error> {
        let mut log_entries = Vec::new();
        let tags_path = path.join(".rust_git").join("refs").join("tags");
        let entries_tag = fs::read_dir(tags_path)?;

        for entry in entries_tag {
            let tag_file: fs::DirEntry = entry?;
            let _ = fs::File::open(tag_file.path())?;
            if let Some(tag_name) = tag_file.path().file_name() {
                let tag_hash = fs::read_to_string(tag_file.path())?;
                let is_comun = process_tag_content(tag_hash.clone(), path)?;
                let format_tag = if is_comun {
                    format!(
                        "{} refs/tags/{}^{}",
                        tag_hash,
                        tag_name.to_string_lossy(),
                        "{}"
                    )
                } else {
                    format!("{} refs/tags/{}", tag_hash, tag_name.to_string_lossy())
                };
                log_entries.push(format_tag);
            }
        }
        Ok(log_entries)
    }

    /// Extrae los hashes antiguo y nuevo de una línea de texto formateada como "old_hash-new_hash".
    fn extract_old_new_commit(line: String) -> String {
        let parts: Vec<&str> = line.split('-').collect();
        let old_hash = parts[1];
        let new_hash = parts[2];
        let hashes = format!("{} {}", old_hash, new_hash);
        hashes
    }

    /// Procesa un archivo, devolviendo la última línea del mismo como un Resultado.
    fn process_file(file_path: &Path) -> Result<String, std::io::Error> {
        let file = fs::File::open(file_path)?;
        let reader = io::BufReader::new(file);
        let mut last_line = String::new();

        for line in reader.lines() {
            last_line = line?;
        }

        Ok(last_line)
    }

    /// Obtiene las entradas de commits de las ramas en la ruta especificada.
    fn get_commits_branch(path: &Path) -> Result<Vec<String>, std::io::Error> {
        if path.is_dir() {
            let mut refs = Vec::new();

            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let file_path = entry.path();

                if file_path.is_file() {
                    let last_line = Self::process_file(&file_path)?;
                    let hashes = Self::extract_old_new_commit(last_line);

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
    
    /// Analiza un mensaje y extrae la ruta del repositorio actual.
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
