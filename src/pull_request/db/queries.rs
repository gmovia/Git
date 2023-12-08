use std::{path::Path, fs::{OpenOptions, self}, io::Write};

use crate::{pull_request::schemas::schemas::CreatePullRequest, utils::randoms::random::Random};

pub struct Query;

impl Query{
    /// Almacena el PR en la base de datos y devuelve un identificador unico.
    pub fn create_pull_request(server: &Path, pr: CreatePullRequest) -> Result<String,  std::io::Error>{
        let id = Random::random();
        let folder_path = server.join("pull_requests").join(&pr.base_repo);
        fs::create_dir_all(&folder_path)?;
        let pr_path = folder_path.join(&id);
        let mut id_file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(pr_path)?;

        let title = pr.title.map_or("None".to_string(), |u| u);
        let body = pr.body.map_or("None".to_string(), |u| u);
        
        id_file.write_all(
            format!(
                "{}\n{}\n{}\n{}\n{}\n{}\n{}",
                title, 
                pr.head_repo,
                pr.base_repo,
                pr.head,
                pr.base,
                pr.username,
                body
            ).as_bytes()
        )?;

        Ok(id)
    }  
}

