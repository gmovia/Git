use crate::vcs::files::{index::Index, vcs_file::VCSFile};
use std::{collections::HashMap, path::Path};

pub struct Reset;

impl Reset {
    pub fn reset(path: &Path) -> Result<HashMap<String, VCSFile>, std::io::Error> {
        let mut index = Index::read_index()?;
        index.remove(&path.to_string_lossy().to_string());
        Index::write_index(&index)?;
        Ok(index)
    }
}
