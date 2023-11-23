use std::path::Path;



pub struct ShowRef;

pub enum ShowRefOptions {
    GetAll,
    GetRefBranches,
    GetRefTags,
}

impl ShowRef {

    pub fn show_ref(path: &Path, option: ShowRefOptions) -> Result<Vec<String>, std::io::Error> {
        
        match option{
            ShowRefOptions::GetAll => {Self::get_all_refs(path)},
            ShowRefOptions::GetRefBranches => {Self::get_refs_branches(path)},
            ShowRefOptions::GetRefTags => {Self::get_refs_tags(path)},
        }
    }

    pub fn get_refs_branches(path: &Path) -> Result<Vec<String>, std::io::Error> {

        Ok(Vec::new())
    }

    pub fn get_refs_tags(path: &Path) -> Result<Vec<String>, std::io::Error> {

        Ok(Vec::new())
    }

    pub fn get_all_refs(path: &Path) -> Result<Vec<String>, std::io::Error> {

        Ok(Vec::new())
    }
}