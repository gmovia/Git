use std::{collections::HashMap, fs, path::Path};

pub struct ShowRef;

pub enum ShowRefOptions {
    GetAll,
    GetRefHeads,
    GetRefTags,
}

impl ShowRef {

    /// Comando show_ref.
    /// Matchea distintas operaciones y devuelve un hashmap con la informacion leida de refs
    pub fn show_ref(
        repo_path: &Path,
        option: ShowRefOptions,
    ) -> Result<HashMap<String, String>, std::io::Error> {
        match option {
            ShowRefOptions::GetAll => Self::get_all_refs(repo_path),
            ShowRefOptions::GetRefHeads => Self::get_refs_heads(repo_path),
            ShowRefOptions::GetRefTags => Self::get_refs_tags(repo_path),
        }
    }

    /// Lee la carpeta heads y devuelve un hashmap con los nombres de los archivos (seria informacion de las ramas del repositorio)
    pub fn get_refs_heads(repo_path: &Path) -> Result<HashMap<String, String>, std::io::Error> {
        Self::get_dir_refs(repo_path, "heads")
    }

    /// Lee la carpeta tags y devuelve un hashmap con los tags asociados a los commits
    pub fn get_refs_tags(repo_path: &Path) -> Result<HashMap<String, String>, std::io::Error> {
        Self::get_dir_refs(repo_path, "tags")
    }

    /// Devuelve toda la informacion, tanto heads como tags
    pub fn get_dir_refs(
        repo_path: &Path,
        dir: &str,
    ) -> Result<HashMap<String, String>, std::io::Error> {
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

    pub fn get_all_refs(repo_path: &Path) -> Result<HashMap<String, String>, std::io::Error> {
        let heads: HashMap<String, String> = Self::get_refs_heads(repo_path)?;
        let tags: HashMap<String, String> = Self::get_refs_tags(repo_path)?;
        Ok(heads.into_iter().chain(tags).collect())
    }
}
