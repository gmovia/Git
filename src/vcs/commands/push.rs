use std::{net::TcpStream, path::{Path, PathBuf}, io::{BufReader, BufWriter, Write, self, BufRead}, fs};

use crate::{ packfile::packfile::{to_pkt_line, send_done_msg}, protocol::send_pack::handle_send_pack};

pub struct Push;

impl Push{
    pub fn push(stream: &mut TcpStream, current_repo: PathBuf) -> Result<(),std::io::Error> {
        println!("ESTOY EN PUSHHHHHH\n\n");
        let mut path_buf = PathBuf::new();

        // Agregar componentes a la ruta (en este caso, un nombre de carpeta)
        path_buf.push("Server");
        let logs_path = path_buf.join(".git").join("logs");
        //let log_entries = Self::get_commits_branch(&logs_path)?;

        let mut ref_string: Vec<String> = Vec::new();
        ref_string.push("397df39557e5c26d2f3206fb5efbd7237e0c92d4 192c5a5175cd72b058f6f86aed23574d56ab5393 refs/heads/master".to_string());
        
        for entries in &ref_string{
            let ref_to_pkt = to_pkt_line(entries);
            stream.write_all(ref_to_pkt.as_bytes())?;
        }
        println!("LOG entries -----> {:?}\n", ref_string);

        println!("Hasta aca desde el cliente le mande las refs que tengo \n");
        handle_send_pack(stream, &current_repo)?;
        Ok(())
    }
    fn get_commits_branch(path: &Path) -> Result<Vec<String>, std::io::Error> {
        if path.is_dir() {
            let mut refs = Vec::new();
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let file_path = entry.path();
                    if file_path.is_file() {
                    let file = fs::File::open(&file_path)?;
                        let reader = io::BufReader::new(file);
                        let mut last_line = String::new();
                    for line in reader.lines() {
                        last_line = line?;
                    }
                    if !last_line.contains('-') {
                        eprintln!("La línea del archivo {} no tiene el formato esperado", file_path.display());
                        continue;
                    }
                    let hash = last_line.split('-').nth(1).unwrap();
                    let file_name = file_path.file_name().unwrap().to_str().unwrap();
                    let branch = format!("{} refs/heads/{}", hash, file_name);
                    refs.push(branch);
                }
            }
            Ok(refs)
        } else {
            eprintln!("La carpeta de logs no existe en la ubicación especificada.");
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "La carpeta de logs no existe"))
        }
    }

}