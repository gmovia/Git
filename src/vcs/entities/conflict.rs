use std::collections::HashMap;
use crate::constants::constants::{NONE, CURRENT, INCOMING, BOTH};
use super::change::Change;

#[derive(Debug, Clone)]
pub struct Conflict{
    pub file: String,
    pub change_current: Change,
    pub change_branch: Change,
    pub resolved: char,
}

pub fn conflicts_search(changes: &HashMap<String, Change>, other_changes: &HashMap<String, Change>) -> HashMap<String, Conflict>{
    let mut conflicts: HashMap<String, Conflict> = HashMap::new();
    for change_current in changes.values() {
        for change_branch in other_changes.values() {
            if change_branch.file == change_current.file && (change_branch.hash != change_current.hash || change_branch.state != change_current.state) {
                let conflict = Conflict{file: change_branch.file.clone(), change_current: change_current.clone(), change_branch: change_branch.clone(), resolved: NONE};
                conflicts.insert(change_branch.file.clone(), conflict);
            }
        }
    }
    conflicts
}

pub fn resolve_conflicts(conflicts: &HashMap<String, Conflict>, current_changes: &mut HashMap<String, Change>, incoming_change: &mut HashMap<String, Change>){
    for (file, conflict) in conflicts{
        match conflict.resolved{
            CURRENT => {incoming_change.remove(file);},
            INCOMING => {current_changes.remove(file);},
            BOTH => {todo!()}, // MERGEAR EL ARCHIVO Y GUARDAR!
            _ => {}
        }
    }
}