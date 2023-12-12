use std::{path::{Path, PathBuf}, fs::{self, OpenOptions}, io::Write};

use crate::server_http::requests::create_pull_request::CreatePullRequest;

pub fn get_prs_path(server: &Path, repo: &String) -> PathBuf{ // Ruta que apunta a la carpeta que tiene los PRs del repo
    server.join("pull_requests").join(repo)
}

pub fn get_pr_path(server: &Path, repo: &String, id: &String) -> PathBuf{ // Ruta que apunta a un PR de un repo
    server.join("pull_requests").join(repo).join(id)
}

pub fn create_prs_file(server: &Path, repo: &String) -> PathBuf{
    let folder_path = get_prs_path(server, repo);
    let _ = fs::create_dir_all(&folder_path);
    folder_path
}

pub fn create_table(prs_path: &Path, id: &String, pr: &CreatePullRequest) -> Result<PathBuf, std::io::Error>{
    let head_repo: Vec<&str> = pr.head_repo.split("/").collect();
    let format = format!("{}_{}_{}", head_repo[1], pr.head, pr.base);
    let table = prs_path.join(format);
    let mut file = OpenOptions::new().write(true).create(true).append(true).open(&table)?;
    file.write_all(format!("{}\n",id).as_bytes())?;
    Ok(table)
}