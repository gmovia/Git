use crate::constants::constant::COMMIT_INIT_HASH;
use crate::packfiles::tag_file::{get_tags, exclude_tag_ref, create_tag_files};
use crate::vcs::commands::branch::Branch;
use crate::vcs::commands::cat_file::CatFile;
use crate::vcs::entities::entity::convert_to_repository;
use crate::vcs::entities::tree_entity::TreeEntity;
use crate::vcs::files::commits_table::CommitsTable;
use std::collections::{HashMap, HashSet};
use std::fs::{OpenOptions, self, File};
use std::io::{Write, self};
use std::net::Shutdown;
use std::path::Path;
use std::{net::TcpStream, path::PathBuf};

use chrono::{DateTime, Utc, NaiveDateTime};

use crate::packfiles::packfile::{process_line, to_pkt_line};

use crate::utils::files::file::{delete_all_files_and_folders, create_file_and_their_folders};
use crate::utils::randoms::random::Random;
use crate::vcs::commands::clone::Clone;
use crate::vcs::commands::init::Init;
use crate::vcs::entities::commit_entity::CommitEntity;
use crate::vcs::files::current_commit::CurrentCommit;


pub fn start_handler_receive(writer: &mut TcpStream, server_client_path: PathBuf) -> Result<String, std::io::Error> {
    let _ = handler_receive_pack(writer)?; // que hago con esto? let _? o lo saco?
    
    send_repo_last_commit_for_branch(writer, &server_client_path)?;
    select_update(writer, server_client_path.clone())?;
    
    writer.shutdown(Shutdown::Both)?;
    Ok("Respuesta desde start_handler_receive".to_string())
}

fn extract_branch_name(old_new_hash_commit: String) ->  Result<(String, String), std::io::Error> {
    let parts: Vec<&str> = old_new_hash_commit.split_whitespace().collect(); 
    let last_commit_client = parts[1];
    let branch_name = parts[2].trim_start_matches("refs/heads/").trim_end_matches('\n');
    Ok((branch_name.to_owned(), last_commit_client.to_string()))
}


fn recovery_last_commit_for_each_branch(server_client_path: &Path) -> Result<Vec<(String, String)>, io::Error> {
    let logs_path = server_client_path.join(".rust_git").join("logs");

    let mut branch_commits = Vec::new();
    for entry in fs::read_dir(logs_path)? {
        let entry = entry?;

        if let Some(file_name) = entry.file_name().to_str() {
            let branch_name = file_name.to_string();
            let file_path = entry.path();
            let file_content = fs::read_to_string(&file_path)?;

            let last_commit = extract_last_commit(&file_content)?;

            branch_commits.push((last_commit, branch_name));
        }
    }

    Ok(branch_commits)
}

fn extract_last_commit(log_content: &str) -> Result<String, io::Error> {
    if log_content.is_empty() {
        return Ok(COMMIT_INIT_HASH.to_string());
    }
    let last_line = log_content.lines().last().ok_or(io::Error::new(io::ErrorKind::InvalidData, "Log file is empty"))?;
    let line_parts: Vec<&str> = last_line.split('-').collect();
    let last_commit = line_parts.get(2).ok_or(io::Error::new(io::ErrorKind::InvalidData, "Log line is malformed"))?;
    Ok(last_commit.to_string())
}

fn send_repo_last_commit_for_branch(writer: &mut TcpStream, server_client_path: &Path) -> Result<(), std::io::Error>{
    let last_commit_and_branch = recovery_last_commit_for_each_branch(server_client_path)?;
    
    for (last_commit, branch_server_name) in last_commit_and_branch{
        let update_info = format!("{} refs/heads/{}\n",last_commit, branch_server_name);

        let info_to_pkt_line = to_pkt_line(&update_info);
        writer.write_all(info_to_pkt_line.as_bytes())?;
    }

    let tags_exist = get_tags(server_client_path)?;

    for tag in tags_exist{
        let tag_to_pkt_line = to_pkt_line(&tag);
        writer.write_all(tag_to_pkt_line.as_bytes())?;
    }

    let msg_done = "0000";
    writer.write_all(msg_done.as_bytes())?;
    Ok(())
}

fn select_update(writer: &mut TcpStream, server_client_path: PathBuf) -> Result<(), std::io::Error>{
    let mut receive_refs = Vec::new();
    loop {
        let value = process_line(writer);
        match value {
            Ok(value) => {
                if value == "0" {
                    break;
                } else {
                    receive_refs.push(value);
                }                
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    
    let (list_tags, branchs_refs) = exclude_tag_ref(receive_refs)?;
    change_current_branch(branchs_refs.clone(), &server_client_path)?;

    let set: HashSet<String> = list_tags.into_iter().collect();
    let unique_list_tags: Vec<String> = set.into_iter().collect();
    create_tag_files(unique_list_tags, &server_client_path)?;
    let objects = Clone::get_socket_response(writer)?;
    updating_repo( objects, &server_client_path, "asd".to_string())?;
    writer.flush()?;
    Ok(())
}

fn empty_log_file(server_client_path: &Path, branch_name: &str) -> io::Result<()> {
    let logs_path = server_client_path.join(".rust_git").join("logs").join(branch_name);

    // Abre el archivo en modo de escritura y establece la longitud a cero.
    OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(logs_path)?
        .set_len(0)?;

    Ok(())
}

fn change_current_branch(receive_refs: Vec<String>, server_client_path: &Path) -> Result<(), std::io::Error> {
    let (branch_name, last_commit_client) = extract_branch_name(receive_refs[0].to_string())?;
    let current_branch = Init::get_current_branch(server_client_path)?;

    if !branch_exist(&branch_name, server_client_path)? {
        create_new_branch_with_hash(server_client_path, &branch_name, &last_commit_client)?;
        empty_log_file(server_client_path, branch_name.as_str())?;
    }

    if current_branch != branch_name {
        change_branch(server_client_path, branch_name.as_str())?;
    }

    CurrentCommit::write_for_branch(server_client_path, &branch_name, last_commit_client)?;

    Ok(())
}

pub fn change_branch(path: &Path, branch_name: &str) -> Result<(), std::io::Error>{
    let rust_git_path = path.join(".rust_git");
    let head_path = rust_git_path.join("HEAD");
    let mut file = File::create(head_path)?;
    file.write_all(format!("refs/heads/{}", branch_name).as_bytes())?;
    Ok(())
}

pub fn create_new_branch_with_hash(path: &Path, branch_name: &str, hash: &str) -> Result<(),std::io::Error> { 
    let branch_head_path = path.join(".rust_git").join("refs").join("heads").join(branch_name);        
    let mut branch_head = OpenOptions::new().write(true).create(true).append(false).open(branch_head_path)?;

    let branch_log_path = path.join(".rust_git").join("logs").join(branch_name);
    let mut branch_log = OpenOptions::new().write(true).create(true).append(true).open(branch_log_path)?;
    
    let current_log = Init::get_current_log(path)?;
    let table = fs::read_to_string(current_log)?;
    
    branch_head.write_all(hash.as_bytes())?;
    branch_log.write_all(table.as_bytes())?;
    Ok(())
}


fn branch_exist(branch_name: &str, server_client_path: &Path) -> Result<bool, std::io::Error> {
    let branchs: Vec<String> = Branch::get_branches(server_client_path)?;
    Ok(branchs.contains(&branch_name.to_string()))
}


pub fn updating_repo( objects: Vec<(u8, Vec<u8>)>, repo_server_client: &Path, _last_commit: String) -> Result<(), std::io::Error> {
    let objects_prcessed = Clone::process_folder(objects.to_vec());

    let commits_created = Clone::create_folders(objects_prcessed.clone(), repo_server_client);
    let hashes_sorted = sort_hashes(&commits_created);

    for(commit_hash, commit_entity ) in &hashes_sorted{
        write_commit_log_push(&commit_entity.parent_hash, commit_hash, commit_entity, repo_server_client.to_path_buf())?;
    }
    update_cd(repo_server_client)?;

    Ok(())
}

fn sort_hashes(commits_created: &HashMap<String, CommitEntity>) -> Vec<(String, CommitEntity)> {
    let mut commits_vec: Vec<(String, CommitEntity)> = commits_created.iter().map(|(k, v)| (k.clone(), v.clone())).collect();

    commits_vec.sort_by_key(|( _, commit)| {
        let date_str = match commit.author.split_whitespace().nth(3){
            Some(date) => date,
            None => ""
        };
        let date_num = match date_str.parse::<i64>() {
            Ok(date_num) => date_num,
            Err(_) => 0
        };
        let _ = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(date_num, 0), Utc);
    });

    commits_vec
}



fn write_commit_log_push(last_commit_hash: &String, new_commit_hash: &String, commit_entity: &CommitEntity, repo_server_client: PathBuf)  -> Result<(),std::io::Error>{
    let id = Random::random();
    let mut commits_file = OpenOptions::new().write(true).append(true).open(Init::get_current_log(&repo_server_client)?)?; //abro la tabla de commits para escribir - si no existe, la creo
    let date = Clone::get_date(&commit_entity.author);

    let commit = format!("{}-{}-{}-{}-{}\n", id, last_commit_hash, new_commit_hash, commit_entity.message, date); 

    commits_file.write_all(commit.as_bytes())?;
    Ok(())
}

pub fn update_cd(path: &Path) -> Result<(), std::io::Error>{
    let repository_hashmap = read(path)?;
    delete_all_files_and_folders(path)?;

    for (key, value) in repository_hashmap{
        let content = CatFile::cat_file(&value, Init::get_object_path(path)?)?;
        let path_server = Path::new(&key);
        create_file_and_their_folders(path_server, &content)?
    }
    Ok(())
}

pub fn read(repo_path: &Path) -> Result<HashMap<String,String>,std::io::Error>{
    let current_branch = &Init::get_current_branch(repo_path)?;
    
    let current_commit_hash = CurrentCommit::read_for_branch(repo_path, current_branch)?;

    let mut local_repository: HashMap<String, String>  = HashMap::new();
    local_repository.extend(read_repository_of_commit(repo_path.to_path_buf(), current_branch, &current_commit_hash)?);
    Ok(local_repository)
}

pub fn read_repository_of_commit(repo_path: PathBuf, branch: &str, commit_hash: &str) -> Result<HashMap<String, String>,std::io::Error>{
    let commits_table = CommitsTable::read(repo_path.clone(), branch)?;

    for commit in commits_table {
        if commit.hash == commit_hash {
            let commit_entity = CommitEntity::read(&repo_path, commit_hash)?; 
            
            let entities  = TreeEntity::read(&repo_path, commit_entity.tree_hash)?;

            return Ok(convert_to_repository(&entities, repo_path));
        }
    }
    Ok(HashMap::new())
}

fn handler_receive_pack(writer: &mut TcpStream) ->  Result<String, std::io::Error>{
    let response = process_line(writer)?;
    Ok(response)
}
       