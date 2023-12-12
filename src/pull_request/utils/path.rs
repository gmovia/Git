use std::path::{Path, PathBuf};

pub fn get_prs_path(server: &Path, repo: &String) -> PathBuf{ // Ruta que apunta a la carpeta que tiene los PRs del repo
    server.join("pull_requests").join(repo)
}

pub fn get_pr_path(server: &Path, repo: &String, id: &String) -> PathBuf{ // Ruta que apunta a un PR de un repo
    server.join("pull_requests").join(repo).join(id)
}

