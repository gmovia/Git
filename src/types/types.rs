use std::collections::HashMap;

pub type UntrackedFiles = HashMap<String, String>;
pub type ChangesToBeCommited = HashMap<String, String>;
pub type ChangesNotStagedForCommit = HashMap<String, String>;