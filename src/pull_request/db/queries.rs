use std::{path::Path, fs::{OpenOptions, self}, io::Write};

use crate::{pull_request::schemas::schemas::{CreatePullRequest, FindPullRequests, PullRequestEntry, FindPullRequest}, utils::randoms::random::Random, vcs::commands::pull::Pull};

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
                "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
                id,
                title, 
                pr.head_repo,
                pr.base_repo,
                pr.head,
                pr.base,
                pr.username,
                "open",
                body,
                pr.mergeable
            ).as_bytes()
        )?;

        Ok(id)
    }  

    pub fn find_all_pull_requests(prs_path: &Path) -> Result<Vec<PullRequestEntry>, std::io::Error>{
        let mut prs: Vec<PullRequestEntry> = Vec::new();
        
        if let Ok(entries) = fs::read_dir(prs_path) {
            for entry in entries{
                if let Ok(entry) = entry{
                    let pr = Self::read_pull_request(&entry.path())?;
                    prs.push(pr);
                }
            }
        }
        Ok(prs)
    }

    pub fn find_pull_requests(prs_path: &Path, query: &FindPullRequests) -> Result<Vec<PullRequestEntry>, std::io::Error>{
        let mut prs = Self::find_all_pull_requests(&prs_path)?;

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

    pub fn find_a_pull_request(id: &Path) -> Result<PullRequestEntry, std::io::Error>{
        Self::read_pull_request(id)
    }

    pub fn read_pull_request(id: &Path) -> Result<PullRequestEntry, std::io::Error>{
        let content = fs::read_to_string(id)?;
        let array: Vec<&str> = content.split_whitespace().collect();
    
        let mergeable = match array[9].parse::<bool>() {
            Ok(value) => value,
            Err(_) => false,
        };
    
        let pr = PullRequestEntry { 
            id: array[0].to_string(), 
            title: array[1].to_string(),
            head_repo: array[2].to_string(),
            base_repo: array[3].to_string(),
            head: array[4].to_string(),
            base: array[5].to_string(),
            username: array[6].to_string(),
            status: array[7].to_string(),
            body: array[8].to_string(),
            mergeable: mergeable
        };
        
        Ok(pr)
    }

    pub fn write_pull_request(id: &Path, pr: &PullRequestEntry) -> Result<(), std::io::Error>{
        let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(id)?;

        file.set_len(0)?;

        file.write_all(
            format!(
                "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
                pr.id,
                pr.title, 
                pr.head_repo,
                pr.base_repo,
                pr.head,
                pr.base,
                pr.username,
                pr.status,
                pr.body,
                pr.mergeable
            ).as_bytes()
        )?;

        Ok(())
    }
}

