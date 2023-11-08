use std::{collections::HashMap, fs::{File, OpenOptions, self}, io::{Write, self, BufRead}};
use crate::{constants::constants::{STATE_CREATED, STATE_MODIFIED, STATE_DELETED}, vcs::files::current_repository::CurrentRepository};
use super::conflict::Conflict;

#[derive(Debug, Clone)]
pub struct Change{
    pub file: String,
    pub hash: String,
    pub state: String,
}

pub fn add_changes(repository: &mut HashMap<String, String>, changes: &HashMap<String, Change>){
    for (_, change) in changes{
        match change.state.as_str() {
            STATE_CREATED | STATE_MODIFIED => {repository.insert(change.file.clone(), change.hash.clone());},
            STATE_DELETED => {repository.remove(&change.file);},
            _ => {},
        } 
    }
}

pub fn write_changes(conflict: &Conflict) -> Result<File,std::io::Error>{
    let current = CurrentRepository::read()?;
    let temp_path = current.join("temp");
    let mut currents = OpenOptions::new().write(true).create(true).append(true).open(&temp_path)?;
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

pub fn read_changes() -> Result<HashMap<String,Conflict>,std::io::Error>{
    let mut conflicts = HashMap::new();
    let current = CurrentRepository::read()?;
    let temp_path = current.join("temp");
    let currents_file = OpenOptions::new().read(true).open(&temp_path)?;
    let reader = io::BufReader::new(currents_file);

    for line in reader.lines().filter_map(Result::ok){
        let parts: Vec<&str> = line.split("-").collect();
        let change_current = Change { file: parts[0].to_string(), hash: parts[1].to_string(), state: parts[2].to_string() };
        let change_branch = Change { file: parts[0].to_string(), hash: parts[3].to_string(), state: parts[4].to_string() };
        let conflict = Conflict { file: parts[0].to_string(), change_current: change_current, change_branch: change_branch, resolved: parts[5].to_string() };
        conflicts.insert(parts[0].to_string(), conflict);
    }
    fs::remove_file(temp_path)?;
    Ok(conflicts)
}