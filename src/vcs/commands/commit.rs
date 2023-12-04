use crate::constants::constant::{STATE_CREATED, STATE_DELETED, STATE_MODIFIED};
use crate::vcs::files::{commits_table::CommitsTable, index::Index, repository::Repository};
use std::collections::HashMap;
use std::io;
extern crate chrono;

pub struct Commit;

impl Commit {
    pub fn commit(message: String) -> Result<HashMap<String, String>, std::io::Error> {
        let mut repository = Repository::read_repository()?;
        let staging_area = Index::read_index()?;
        if staging_area.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "The staging area is empty, you need to add before commit",
            ));
        }
        for (key, value) in &staging_area {
            match value.clone().state.as_str() {
                STATE_CREATED => {
                    repository.insert(key.to_string(), value.clone().content);
                }
                STATE_MODIFIED => {
                    repository.insert(key.to_string(), value.clone().content);
                }
                STATE_DELETED => {
                    repository.remove(key);
                }
                _ => {}
            }
        }

        CommitsTable::write(&message, &repository)?;
        Index::clear()?;
        Ok(repository)
    }
}
