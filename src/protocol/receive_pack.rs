use crate::packfiles::tag_file::{get_tags, exclude_tag_ref, create_tag_files};
use crate::vcs::commands::branch::Branch;
use crate::vcs::commands::cat_file::CatFile;
use crate::vcs::entities::entity::convert_to_repository;
use crate::vcs::entities::tree_entity::TreeEntity;
use crate::vcs::files::commits_table::CommitsTable;
use std::collections::HashMap;
use std::fs::{OpenOptions, self, File};
use std::io::{Write, self};
use std::net::Shutdown;
use std::path::Path;
use std::{net::TcpStream, path::PathBuf};

use chrono::{DateTime, Utc, NaiveDateTime};

use crate::packfiles::packfile::{process_line, to_pkt_line};

use crate::utils::files::file::{delete_all_files_and_folders, create_file_and_their_folders, self};
use crate::utils::randoms::random::Random;
use crate::vcs::commands::clone::Clone;
use crate::vcs::commands::init::Init;
use crate::vcs::entities::commit_entity::CommitEntity;
use crate::vcs::files::current_commit::CurrentCommit;
use crate::vcs::files::repository::Repository;


pub fn start_handler_receive(writer: &mut TcpStream, server_client_path: PathBuf) -> Result<String, std::io::Error> {
    println!("-----------start_handler_receive in-------------- \n\n");    
    println!("PATH CLIENTE   en start_handler_receive \n {:?}", server_client_path );

    let old_new_hash_commit = handler_receive_pack(writer)?;

    let (_, _ )= extract_branch_name(old_new_hash_commit.to_string())?;
    
    println!("Received from packet: ---> {:?}", old_new_hash_commit); //lo recibe porque lo manda el cliente, pero daemon no hace nada con eso
    //ni crea la rama si no la tiene, pero eso si lo tenems que hacer almenos

    send_repo_last_commit_for_branch(writer, &server_client_path)?;
    select_update(writer, server_client_path.clone())?;
    
    writer.shutdown(Shutdown::Both)?;
    Ok("Respuesta desde start_handler_receive".to_string())
}

fn extract_branch_name(old_new_hash_commit: String) ->  Result<(String, String), std::io::Error> {
    let parts: Vec<&str> = old_new_hash_commit.split_whitespace().collect();
    // incluyen referencias de ramas o tags 
    // old_hash new_hash nombre_rama
    // tags 
    let last_commit_client = parts[1];
    let branch_name = parts[2].trim_start_matches("refs/heads/").trim_end_matches('\n');
    Ok((branch_name.to_owned(), last_commit_client.to_string()))
}

//fn extract_refs_tags()

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
        println!("Mi pedido del server al cliente {:?}\n\n", info_to_pkt_line);
        writer.write_all(info_to_pkt_line.as_bytes())?;
    }

    //agregar enviar todos los tags que tenga
    let tags_exist = get_tags(server_client_path)?;

    for tag in tags_exist{
        let tag_to_pkt_line = to_pkt_line(&tag);
        println!("Mi pedido de los tags al cliente es: {:?}\n\n", tag_to_pkt_line);
        writer.write_all(tag_to_pkt_line.as_bytes())?;
    }

    let msg_done = "0000";
    writer.write_all(msg_done.as_bytes())?;
    Ok(())
}

fn select_update(writer: &mut TcpStream, server_client_path: PathBuf) -> Result<(), std::io::Error>{
    //
    let mut receive_refs = Vec::new(); // Cambiado a Vec<String>
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
                println!("Error al procesar la línea: {:?}", e);
                return Err(e);
            }
        }
    }
    
    //Este contiene el old new name_branch del cliente,
    // 1. puede pasar que el sea master y los doslos tienen se pushea normal
    // 2. puede pasar que el que pushee sea nueva_rama y el server lo tenga, se guarda normal
    // 3. puede pasar que el server no lo tenga por lo tanto tendra que crear nueva_rama, checkout a esa rama, y guardar datos ahi!

    println!("::::::::::::::Mi lista de refs que recibo es :  --->{:?}\n" , receive_refs);    

    
    change_current_branch(receive_refs.clone(), &server_client_path)?;

    let (list_tags, _) = exclude_tag_ref(receive_refs)?;
    create_tag_files(list_tags, &server_client_path)?;    //recibo refs de ultimos branchs  y al final los de tags

    // aca espero la PACKDATA
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

    for obj in &objects_prcessed{
        println!("------> {:?}", obj);
    }

    let commits_created = Clone::create_folders(objects_prcessed.clone(), repo_server_client);
    println!("COMMITS CREATEDDDD ---> {:?}\n", commits_created);

    let hashes_sorted = sort_hashes(&commits_created);

    println!("HASH SORTED ---> {:?}\n", hashes_sorted);
    for(commit_hash, commit_entity ) in &hashes_sorted{
        write_commit_log_push(&commit_entity.parent_hash, commit_hash, commit_entity, repo_server_client.to_path_buf())?;
    }

    update_cd(repo_server_client)?;

    Ok(())
}

fn sort_hashes(commits_created: &HashMap<String, CommitEntity>) -> Vec<(String, CommitEntity)> {
    let mut commits_vec: Vec<(String, CommitEntity)> = commits_created.iter().map(|(k, v)| (k.clone(), v.clone())).collect();

    commits_vec.sort_by_key(|(_, commit)| {
        let date_str = commit.author.split_whitespace().nth(3).unwrap_or("");
        println!("Fecha extraída: {}", date_str);
        let date_num = date_str.parse::<i64>().unwrap_or(0);
        let date_time = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(date_num, 0), Utc);
        println!("DATE .> {:?}", date_time);
        date_time
    });

    commits_vec
}



fn write_commit_log_push(last_commit_hash: &String, new_commit_hash: &String, commit_entity: &CommitEntity, repo_server_client: PathBuf)  -> Result<(),std::io::Error>{
    let id = Random::random();
    let mut commits_file = OpenOptions::new().write(true).append(true).open(Init::get_current_log(&repo_server_client)?)?; //abro la tabla de commits para escribir - si no existe, la creo
    let date = Clone::get_date(&commit_entity.author);
    println!("New commit hash {}\n", new_commit_hash);

    let commit = format!("{}-{}-{}-{}-{}\n", id, last_commit_hash, new_commit_hash, commit_entity.message, date); 

    commits_file.write_all(commit.as_bytes())?;
    Ok(())
}

pub fn update_cd(path: &Path) -> Result<(), std::io::Error>{
    println!("REPO PATH --> {:?}", path);
    let repository_hashmap = Repository::read(path)?;
    println!("repository_hashmap--> {:?}", repository_hashmap);

    delete_all_files_and_folders(path)?;

    for (key, value) in repository_hashmap{
        let content = CatFile::cat_file(&value, Init::get_object_path(path)?)?;
        println!("CONTENT ----------->{}", content);
        
        if let Some(first_component) = path.display().to_string().split('/').next(){
            // Crear un nuevo PathBuf con el nombre de archivo base y la clave
            let path_server = Path::new(first_component).join(key.clone());
            println!("key ---> {:?}\n", key);
            create_file_and_their_folders(&path_server, &content)?
        }
    }
    Ok(())
}

pub fn read(repo_path: &Path) -> Result<HashMap<String,String>,std::io::Error>{
    let current_branch = &Init::get_current_branch(repo_path)?;
    
    let current_commit_hash = CurrentCommit::read_for_branch(repo_path, current_branch)?;

    let mut local_repository: HashMap<String, String>  = HashMap::new();
    local_repository.extend(Repository::read_repository_of_commit(repo_path.to_path_buf(), current_branch, &current_commit_hash)?);
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
       