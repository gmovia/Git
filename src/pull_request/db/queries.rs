use std::{path::Path, fs::{OpenOptions, self}, io::{Write, self}, collections::HashMap};

use crate::{pull_request::{schemas::schema::{PullRequestEntry, CommitsPullRequest}, utils::path::{create_prs_file, create_table}}, utils::randoms::random::Random, vcs::{files::commits_table::CommitsTable, entities::{commit_entity::CommitEntity, commit_table_entry::CommitTableEntry}, commands::{merge::Merge, rebase::Rebase}}, server_http::requests::{create_pull_request::CreatePullRequest, list_pull_request::ListPullRequests, update_pull_request::UpdatePullRequest}};

pub struct Query;

impl Query{
    pub fn create_pull_request(server: &Path, pr: &CreatePullRequest) -> Result<String, std::io::Error>{     
        
        let id = Random::random();
        let title = pr.title.clone().map_or("None".to_string(), |u| u);
        let body = pr.body.clone().map_or("None".to_string(), |u| u);

        let folder_path = create_prs_file(server, &pr.base_repo);
        let table = create_table(&folder_path, &id, pr)?;

        let init_commit = Self::get_init_commit(server, pr, &table)?;

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
            end_commit: "None".to_string(),
        };

        Self::write_pull_request(&folder_path.join(&id), &pr_entry)?;
        Ok(id)
    }

    pub fn update_pull_request(id: &Path, pr: &UpdatePullRequest) -> Result<String, std::io::Error>{
        let mut old_pr = Query::find_a_pull_request(id)?;

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
            let ids: Vec<&str> = content.split('\n').collect();

            if ids.len() <= 2 || !table.exists() {
                for commit in &head_commits_table {
                    if commit.last_hash == parent_commit.hash {
                        return Ok(commit.hash.clone())
                    }
                }    
            }

            let path_id = server.join("pull_requests").join(&pr.base_repo).join(ids[ids.len()-3]);
            let pr_entry = Query::find_a_pull_request(&path_id)?;
            for commit in head_commits_table{
                if commit.last_hash == pr_entry.end_commit.clone(){
                    return Ok(commit.hash);
                } 
                
            }      
        } 
        
        Ok("None".to_string())
    }

    pub fn find_all_pull_requests(prs_path: &Path) -> Result<Vec<PullRequestEntry>, std::io::Error>{
        let mut prs: Vec<PullRequestEntry> = Vec::new();
        
        if let Ok(entries) = fs::read_dir(prs_path) {
            for entry in entries.flatten(){
                let content = fs::read_to_string(entry.path())?;
                let array: Vec<&str> = content.split('\n').collect();
                if array[0] == "PR" {
                    let pr = Self::find_a_pull_request(&entry.path())?;
                    prs.push(pr);
                }
            }
        }
        Ok(prs)
    }

    pub fn find_pull_requests(prs_path: &Path, query: &ListPullRequests) -> Result<Vec<PullRequestEntry>, std::io::Error>{
        let mut prs = Self::find_all_pull_requests(prs_path)?;

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

        let array: Vec<&str> = content.split('\n').collect();

        let mergeable = array[10].parse::<bool>().unwrap_or(false);
    
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
            end_commit: array[12].to_string()
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
                "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
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
                pr.end_commit,
            ).as_bytes()
        )?;

        Ok(())
    }

    pub fn get_commits_pull_request(server: &Path, id: &Path) -> Result<Vec<CommitsPullRequest>, std::io::Error>{
        let mut commits: Vec<CommitsPullRequest> = Vec::new();
        let pr_entry = Self::find_a_pull_request(id)?;
        let init_commit = pr_entry.init_commit;
        let end_commit = pr_entry.end_commit;

        let head_repo = server.join(&pr_entry.head_repo);
        let head_commits_table = CommitsTable::read(head_repo.clone(), &pr_entry.head)?;
        
        let mut found_init_commit = false;

        for (index,entry) in head_commits_table.iter().enumerate() {
            if entry.hash == init_commit {
                found_init_commit = true;
            }
    
            if found_init_commit {
                if end_commit == *"None" && index == head_commits_table.len() {
                    break;
                }

                let info = CommitEntity::read(&head_repo,&entry.hash)?;

                let commit = CommitsPullRequest{
                    id: entry.id.clone(),
                    parent: entry.last_hash.clone(),
                    hash: entry.hash.clone(),
                    message: entry.message.clone(),
                    date: entry.date.clone(),
                    info
                };
                commits.push(commit);
            }
    
            
            if entry.hash == end_commit.clone() {
                break;
            }
            

        }
        
        Ok(commits)
    }


    pub fn merge_pull_request(server: &Path, id: &Path, merge_method: &mut Option<String>) -> Result<String, std::io::Error> {
        let mut pr_entry = Self::find_a_pull_request(id)?;
        
        let head_repo = server.join(&pr_entry.head_repo);
        let base_repo = server.join(&pr_entry.base_repo);

        if Merge::are_conflicts(&pr_entry.head, &pr_entry.base, &head_repo.clone(), &base_repo.clone())?{
            return Err(io::Error::new(
                io::ErrorKind::Other,"405 Method Not Allowed if merge cannot be performed"))
        }

        if let Some(method) = merge_method {
            let head_commits_table = CommitsTable::read(head_repo.clone(), &pr_entry.head)?;
            match method.as_str() {
                "merge" => {let response = Merge::merge_pr(&pr_entry.username, &pr_entry.head, &pr_entry.base, &head_repo, &base_repo,HashMap::new());
                                if response.is_ok() {
                                    Self::update_pr(head_commits_table, &mut pr_entry, id)
                                }else{
                                    Err(io::Error::new(
                                        io::ErrorKind::Other,"405 Method Not Allowed if merge cannot be performed"))
                                }
                },
                "rebase" => {let response = Rebase::rebase_pr(&pr_entry.username, &pr_entry.head, &pr_entry.base, &head_repo, &base_repo);
                            let head_commits_table = CommitsTable::read(head_repo.clone(), &pr_entry.head)?;
                                if response.is_ok() {
                                    Self::update_pr(head_commits_table, &mut pr_entry, id)
                                }else{
                                    Err(io::Error::new(
                                        io::ErrorKind::Other,"405 Method Not Allowed if merge cannot be performed"))
                                }
                },
                _ => {Err(io::Error::new(
                    io::ErrorKind::Other,"422 The method is wrong for merge a pull request"))}
            }
        }else {
            Err(io::Error::new(
                io::ErrorKind::Other,"405 Method Not Allowed if merge cannot be performed"))
        }
    }

    pub fn update_pr(base_commits_table: Vec<CommitTableEntry>, pr_entry: &mut PullRequestEntry, id: &Path) -> Result<String, std::io::Error>{
        if let Some(commit) = base_commits_table.iter().last() {
            pr_entry.end_commit = commit.hash.clone();
            }
        pr_entry.status = "close".to_string();
        Self::write_pull_request(id, pr_entry)?;
        Ok("200 Merge successfully".to_string())
    }
}

