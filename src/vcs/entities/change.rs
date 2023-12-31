use super::conflict::Conflict;
use crate::{
    constants::constant::{STATE_CREATED, STATE_DELETED, STATE_MODIFIED},
    vcs::files::current_repository::CurrentRepository,
};
use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{self, BufRead, Write},
};

#[derive(Debug, Clone)]
pub struct Change {
    pub file: String,
    pub hash: String,
    pub state: String,
}

/// Agrega los cambios al repositorio
pub fn add_changes(repository: &mut HashMap<String, String>, changes: &HashMap<String, Change>) {
    for change in changes.values() {
        match change.state.as_str() {
            STATE_CREATED | STATE_MODIFIED => {
                repository.insert(change.file.clone(), change.hash.clone());
            }
            STATE_DELETED => {
                repository.remove(&change.file);
            }
            _ => {}
        }
    }
}

/// Escribe los cambios en un archivo temporal
pub fn write_changes(conflict: &Conflict) -> Result<File, std::io::Error> {
    let current = CurrentRepository::read()?;
    let temp_path = current.join("temp_merge");
    let mut currents = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(temp_path)?;
    let data_to_write = format!(
        "{}-{}-{}-{}-{}-{}\n",
        conflict.file,
        conflict.change_current.hash,
        conflict.change_current.state,
        conflict.change_branch.hash,
        conflict.change_branch.state,
        conflict.resolved
    );
    currents.write_all(data_to_write.as_bytes())?;
    Ok(currents)
}

/// Lee los conflictos del archivo
pub fn read_changes() -> Result<HashMap<String, Conflict>, std::io::Error> {
    let mut conflicts = HashMap::new();
    let current = CurrentRepository::read()?;
    let temp_path = current.join("temp_merge");
    let currents_file = OpenOptions::new().read(true).open(&temp_path)?;
    let reader = io::BufReader::new(currents_file);

    for line in reader.lines().map_while(Result::ok) {
        let parts: Vec<&str> = line.split('-').collect();
        let change_current = Change {
            file: parts[0].to_string(),
            hash: parts[1].to_string(),
            state: parts[2].to_string(),
        };
        let change_branch = Change {
            file: parts[0].to_string(),
            hash: parts[3].to_string(),
            state: parts[4].to_string(),
        };
        let conflict = Conflict {
            file: parts[0].to_string(),
            change_current,
            change_branch,
            resolved: parts[5].to_string(),
        };
        conflicts.insert(parts[0].to_string(), conflict);
    }
    fs::remove_file(temp_path)?;
    Ok(conflicts)
}
