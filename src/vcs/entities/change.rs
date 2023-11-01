use std::collections::HashMap;

use crate::constants::constants::{STATE_CREATED, STATE_MODIFIED, STATE_DELETED};

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