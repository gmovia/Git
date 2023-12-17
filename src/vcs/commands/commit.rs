use crate::constants::constant::{STATE_CREATED, STATE_DELETED, STATE_MODIFIED};
use crate::vcs::files::config::Config;
use crate::vcs::files::current_repository::CurrentRepository;
use crate::vcs::files::{commits_table::CommitsTable, index::Index, repository::Repository};
use std::collections::HashMap;
use std::io;

use super::init::Init;
extern crate chrono;

pub struct Commit;

impl Commit {
    /// Recibe un mensaje y realiza un commit
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
        let config = Config::read_config()?;
        let current_repository = CurrentRepository::read()?;
        let current_branch = &Init::get_current_branch(&current_repository)?;
        CommitsTable::write(
            &current_repository,
            current_branch,
            &message,
            config,
            &repository,
        )?;
        Index::clear()?;
        Ok(repository)
    }
}
