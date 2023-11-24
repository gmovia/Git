use std::{path::Path, collections::HashMap, fs};



pub struct ShowRef;

pub enum ShowRefOptions {
    GetAll,
    GetRefHeads,
    GetRefTags,
}

impl ShowRef {

    pub fn show_ref(repo_path: &Path, option: ShowRefOptions) -> Result<HashMap<String, String>, std::io::Error> {
        
        match option{
            ShowRefOptions::GetAll => {Self::get_all_refs(repo_path)},
            ShowRefOptions::GetRefHeads => {Self::get_refs_heads(repo_path)},
            ShowRefOptions::GetRefTags => {Self::get_refs_tags(repo_path)},
        }
    }
    
    // git show-ref --heads --> Lee la carpeta refs/heads
    pub fn get_refs_heads(repo_path: &Path) -> Result<HashMap<String, String>, std::io::Error> {
        Self::get_dir_refs(repo_path, "heads")
    }

    // git show-ref --tags --> Lee la carpeta refs/tags
    pub fn get_refs_tags(repo_path: &Path) -> Result<HashMap<String, String>, std::io::Error> {
        Self::get_dir_refs(repo_path, "tags")
    }
    
    pub fn get_dir_refs(repo_path: &Path, dir: &str) -> Result<HashMap<String, String>, std::io::Error>{
        let mut refs = HashMap::new();
        let refs_path = repo_path.join(".rust_git").join("refs").join(dir);
        if let Ok(entries) = fs::read_dir(refs_path) {
           for entry in entries.flatten() {
                let file_name = entry.file_name();
                let hash = fs::read_to_string(entry.path())?;
                refs.insert(file_name.to_string_lossy().to_string(), hash);
           } 
        }
        Ok(refs)
    }

    // git show-ref --> Lee todo
    pub fn get_all_refs(repo_path: &Path) -> Result<HashMap<String, String>, std::io::Error> {
        let heads: HashMap<String, String> = Self::get_refs_heads(repo_path)?;
        let  tags: HashMap<String, String> = Self::get_refs_tags(repo_path)?;
        // vengo con dilay
        Ok(heads.into_iter().chain(tags).collect()) 
    }

}