use crate::{
    constants::constant::{STATE_CREATED, STATE_DELETED, STATE_MODIFIED},
    vcs::entities::change::Change,
};
use std::collections::HashMap;

pub struct Diff;

impl Diff {
    /// Recibe dos repositorios y devuelva la diferencia que hay entre ambos (archivos creados, modificados y eliminados)
    pub fn diff(
        parent: &HashMap<String, String>,
        current: &HashMap<String, String>,
    ) -> HashMap<String, Change> {
        let mut diff: HashMap<String, Change> = HashMap::new();
        for (path, hash) in current {
            if !parent.contains_key(path) {
                let change = Change {
                    file: path.to_string(),
                    state: STATE_CREATED.to_string(),
                    hash: hash.to_string(),
                };
                diff.insert(change.file.clone(), change);
            } else if let Some(hash_parent) = parent.get(path) {
                if hash != hash_parent {
                    let change = Change {
                        file: path.to_string(),
                        state: STATE_MODIFIED.to_string(),
                        hash: hash.to_string(),
                    };
                    diff.insert(change.file.clone(), change);
                }
            }
        }

        for (path, hash) in parent {
            if !current.contains_key(path) {
                let change = Change {
                    file: path.to_string(),
                    state: STATE_DELETED.to_string(),
                    hash: hash.to_string(),
                };
                diff.insert(change.file.clone(), change);
            }
        }
        diff
    }
}
