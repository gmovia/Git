use std::{path::PathBuf, collections::HashMap};

use crate::vcs::files::{repository::{self, Repository}, vcs_file::VCSFile, index::Index};

pub struct Reset;

impl Reset{
    pub fn reset(path: PathBuf) -> Result<HashMap<String, VCSFile>, std::io::Error>{
        let mut index = Index::read_index()?;
        index.remove(&path.to_string_lossy().to_string());
        Index::write_index(&index)?;
        Ok(index)
    }
}