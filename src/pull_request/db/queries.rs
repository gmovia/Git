use std::{path::Path, fs::{OpenOptions, self}, io::Write};

use crate::{pull_request::schemas::schemas::{PullRequestEntry, CommitsPullRequest, UpdatePullRequest}, utils::randoms::random::Random, vcs::{files::commits_table::CommitsTable, entities::commit_entity::CommitEntity}, server_http::requests::{create_pull_request::CreatePullRequest, list_pull_request::ListPullRequests}};

pub struct Query;

impl Query{
    pub fn create_pull_request(server: &Path, pr: &CreatePullRequest) -> Result<String,  std::io::Error>{
        let folder_path = server.join("pull_requests").join(&pr.base_repo);
        fs::create_dir_all(&folder_path)?;        
        
        let id = Random::random();
        let title = pr.title.clone().map_or("None".to_string(), |u| u);
        let body = pr.body.clone().map_or("None".to_string(), |u| u);

        let head_repo: Vec<&str> = pr.head_repo.split("/").collect();
        let format = format!("{}_{}_{}", head_repo[1], pr.head, pr.base);
        let table = folder_path.join(format);
        let mut file = OpenOptions::new().write(true).create(true).append(true).open(&table)?;

        let init_commit = Self::get_init_commit(&server, &pr, &table)?;

        let pr_entry = PullRequestEntry{
            id: id.clone(),
            title, 
            head_repo: pr.head_repo.clone(),
            base_repo: pr.base_repo.clone(),
            head: pr.head.clone(),
            base: pr.base.clone(),
            username: pr.username.clone(),
            status: "open".to_string(),
            body,
            mergeable: pr.mergeable,
            init_commit: init_commit.clone(),
            last_commit: None,
        };

        Self::write_pull_request(&folder_path.join(&id), &pr_entry)?;
        file.write_all(format!("{}\n",id).as_bytes())?;
        Ok(id)
    }

    pub fn update_pull_request(id: &Path, pr: &UpdatePullRequest) -> Result<String, std::io::Error>{
        let mut old_pr = Query::find_a_pull_request(&id)?;

        if let Some(title) = &pr.title{
            old_pr.title = title.to_string();
        }

        if let Some(body) = &pr.body{
            old_pr.body = body.to_string();
        }

        if let Some(status) = &pr.status{
            old_pr.status = status.to_string();
        }

        if let Some(base) = &pr.base{
            old_pr.base = base.to_string();
        }

        Self::write_pull_request(id, &old_pr)?;
        Ok("Pull Request is updated successfully".to_string())
    }  

    pub fn get_init_commit(server: &Path, pr: &CreatePullRequest, table: &Path)-> Result<String, std::io::Error>{
        let base_repo = server.join(&pr.base_repo);
        let head_repo = server.join(&pr.head_repo);

        let base_commits_table = CommitsTable::read(base_repo.clone(), &pr.base)?;
        let head_commits_table = CommitsTable::read(head_repo.clone(), &pr.head)?;

        
        let content = fs::read_to_string(table)?;

        if let Some(parent_commit) = CommitsTable::get_parent_commit(&base_commits_table, &head_commits_table){
            if !table.exists() || content.is_empty(){

                for commit in head_commits_table{
                    if commit.last_hash == parent_commit.hash{
                        return Ok(commit.hash);
                    }
                }
            }else{
                let ids: Vec<&str> = content.split('\n').collect();
                let path_id = server.join("pull_requests").join(&pr.base_repo).join(ids[ids.len()-1]);
                let pr_entry = Query::find_a_pull_request(&path_id)?;
                for commit in head_commits_table{
                    if let Some(last_hash) = pr_entry.last_commit.clone() {
                        if commit.hash == last_hash{
                            return Ok(commit.hash);
                        }
                    }
                    
                }
                                
            }
        }

        Ok("None".to_string())
    }

    pub fn find_all_pull_requests(prs_path: &Path) -> Result<Vec<PullRequestEntry>, std::io::Error>{
        let mut prs: Vec<PullRequestEntry> = Vec::new();
        
        if let Ok(entries) = fs::read_dir(prs_path) {
            for entry in entries{
                if let Ok(entry) = entry{
                    let content = fs::read_to_string(entry.path())?;
                    let array: Vec<&str> = content.split("\n").collect();
                    if array[0] == "PR" {
                        let pr = Self::find_a_pull_request(&entry.path())?;
                        prs.push(pr);
                    }
                }
            }
        }
        Ok(prs)
    }

    pub fn find_pull_requests(prs_path: &Path, query: &ListPullRequests) -> Result<Vec<PullRequestEntry>, std::io::Error>{
        let mut prs = Self::find_all_pull_requests(&prs_path)?;

        for index in 0..prs.len(){
            if let Some(state) = query.status.clone(){
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
        let content = fs::read_to_string(id)?;
        let array: Vec<&str> = content.split("\n").collect();
        println!("ARRAY: {:?}", array);
        let mergeable = match array[10].parse::<bool>() {
            Ok(value) => value,
            Err(_) => false,
        };
    
        let pr = PullRequestEntry { 
            id: array[1].to_string(), 
            title: array[2].to_string(),
            head_repo: array[3].to_string(),
            base_repo: array[4].to_string(),
            head: array[5].to_string(),
            base: array[6].to_string(),
            username: array[7].to_string(),
            status: array[8].to_string(),
            body: array[9].to_string(),
            mergeable,
            init_commit: array[11].to_string(),
            last_commit: Some(array[12].to_string())
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
                "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{:?}",
                "PR",
                pr.id,
                pr.title, 
                pr.head_repo,
                pr.base_repo,
                pr.head,
                pr.base,
                pr.username,
                pr.status,
                pr.body,
                pr.mergeable,
                pr.init_commit,
                pr.last_commit,
            ).as_bytes()
        )?;

        Ok(())
    }

    pub fn get_commits_pull_request(server: &Path, id: &Path) -> Result<Vec<CommitsPullRequest>, std::io::Error>{
        let mut commits: Vec<CommitsPullRequest> = Vec::new();
        let pr_entry = Self::find_a_pull_request(id)?;
        let init_commit = pr_entry.init_commit;
        let last_commit = pr_entry.last_commit;

        let head_repo = server.join(&pr_entry.head_repo);
        let head_commits_table = CommitsTable::read(head_repo.clone(), &pr_entry.head)?;
        
        let mut found_init_commit = false;

        for (index,entry) in head_commits_table.iter().enumerate() {
            if entry.hash == init_commit {
                found_init_commit = true;
            }
    
            if found_init_commit {
                if last_commit.is_none() && index == head_commits_table.len() {
                    break;
                }

                let info = CommitEntity::read(&head_repo,&entry.hash)?;

                let commit = CommitsPullRequest{
                    id: entry.id.clone(),
                    parent: entry.last_hash.clone(),
                    hash: entry.hash.clone(),
                    message: entry.message.clone(),
                    date: entry.date.clone(),
                    info: info
                };
                commits.push(commit);
            }
    
            if let Some(last_commit_hash) = last_commit.clone() {
                if entry.hash == last_commit_hash {
                    break;
                }
            }

        }
        
        Ok(commits)
    }
}

