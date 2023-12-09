use std::{path::Path, fs::{OpenOptions, self}, io::Write};

use crate::{pull_request::schemas::schemas::{CreatePullRequest, FindPullRequests, PullRequestEntry, FindPullRequest}, utils::randoms::random::Random};

pub struct Query;

impl Query{
    /// Almacena el PR en la base de datos y devuelve un identificador unico.
    pub fn create_pull_request(server: &Path, pr: &CreatePullRequest) -> Result<String,  std::io::Error>{
        let id = Random::random();
        let folder_path = server.join("pull_requests").join(&pr.base_repo);
        fs::create_dir_all(&folder_path)?;
        let pr_path = folder_path.join(&id);
        let mut id_file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(pr_path)?;

        let title = pr.title.clone().map_or("None".to_string(), |u| u);
        let body = pr.body.clone().map_or("None".to_string(), |u| u);
        
        id_file.write_all(
            format!(
                "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
                id,
                title, 
                pr.head_repo,
                pr.base_repo,
                pr.head,
                pr.base,
                pr.username,
                "open",
                body
            ).as_bytes()
        )?;

        Ok(id)
    }  

    pub fn find_pull_requests(server: &Path, query: &FindPullRequests) -> Result<Vec<PullRequestEntry>, std::io::Error>{
        let folder_path = server.join("pull_requests").join(&query.base_repo);
        let mut prs = Self::read_all_pr_in_repo(&folder_path)?;

        for index in 0..prs.len(){
            if let Some(state) = query.state.clone(){
                if prs[index].status != state{
                    prs.remove(index);
                }
            }

            if let Some(head) = query.head.clone(){
                if prs[index].head != head{
                    prs.remove(index);
                }
            }

            if let Some(base) = query.base.clone(){
                if prs[index].base != base{
                    prs.remove(index);
                }
            }
            
            if let Some(username) = query.username.clone(){
                if prs[index].username != username{
                    prs.remove(index);
                }
            }
        }
        
        Ok(prs)
    }

    pub fn parse_pr(content: String) -> PullRequestEntry{
        let array: Vec<&str> = content.split_whitespace().collect();
        PullRequestEntry { 
            id: array[0].to_string(), 
            title: array[1].to_string(),
            head_repo: array[2].to_string(),
            base_repo: array[3].to_string(),
            head: array[4].to_string(),
            base: array[5].to_string(),
            username: array[6].to_string(),
            status: array[7].to_string(),
            body: array[8].to_string()
        }
    }

    pub fn read_all_pr_in_repo(repo: &Path) -> Result<Vec<PullRequestEntry>, std::io::Error>{
        let mut prs: Vec<PullRequestEntry> = Vec::new();

        if let Ok(entries) = fs::read_dir(repo) {
            for entry in entries{
                if let Ok(entry) = entry{
                    let content = fs::read_to_string(entry.path())?;
                    let pr = Query::parse_pr(content);
                    prs.push(pr);
                }
            }
        }
        Ok(prs)
    }

    pub fn find_a_pull_request(server: &Path, query: &FindPullRequest) -> Result<PullRequestEntry, std::io::Error>{
        let folder_path = server.join("pull_requests").join(&query.base_repo).join(&query.id);
        Ok(Self::parse_pr(fs::read_to_string(folder_path)?))
    }
}

