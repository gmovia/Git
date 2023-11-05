use std::{collections::HashMap, fs::{File, OpenOptions, self}, io::{Write, self, BufRead}};

use crate::{constants::constants::{STATE_CREATED, STATE_MODIFIED, STATE_DELETED}, vcs::version_control_system::VersionControlSystem};

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
    let current = VersionControlSystem::read_current_repository()?;
    let temp_path = current.join("temp");
    let mut currents = OpenOptions::new().write(true).create(true).append(true).open(&temp_path)?;
    currents.write_all(conflict.file.to_string().as_bytes())?;
    currents.write_all("-".as_bytes())?;
    currents.write_all(conflict.change_current.hash.to_string().as_bytes())?;
    currents.write_all("-".as_bytes())?;
    currents.write_all(conflict.change_current.state.to_string().as_bytes())?;
    currents.write_all("-".as_bytes())?;
    currents.write_all(conflict.change_branch.hash.to_string().as_bytes())?;
    currents.write_all("-".as_bytes())?;
    currents.write_all(conflict.change_branch.state.to_string().as_bytes())?;
    currents.write_all("-".as_bytes())?;
    currents.write_all(conflict.resolved.to_string().as_bytes())?;
    currents.write_all("\n".as_bytes())?;
    Ok(currents)
}

pub fn read_changes() -> Result<HashMap<String,Conflict>,std::io::Error>{
    let mut conflicts = HashMap::new();
    let current = VersionControlSystem::read_current_repository()?;
    let temp_path = current.join("temp");
    let currents_file = OpenOptions::new().read(true).open(&temp_path)?;
    let reader = io::BufReader::new(currents_file);

    for line in reader.lines().filter_map(Result::ok){
        let parts: Vec<&str> = line.split("-").collect();
        println!("{:?}",parts);
        let change_current = Change { file: parts[0].to_string(), hash: parts[1].to_string(), state: parts[2].to_string() };
        let change_branch = Change { file: parts[0].to_string(), hash: parts[3].to_string(), state: parts[4].to_string() };
        let conflict = Conflict { file: parts[0].to_string(), change_current: change_current, change_branch: change_branch, resolved: parts[5].to_string() };
        conflicts.insert(parts[0].to_string(), conflict);
    }
    fs::remove_file(temp_path)?;
    Ok(conflicts)
}