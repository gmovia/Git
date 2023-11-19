use std::collections::HashMap;
use std::fs::{OpenOptions, self};
use std::io::{Write, self};
use std::net::Shutdown;
use std::path::Path;
use std::{net::TcpStream, path::PathBuf};

use chrono::{DateTime, Utc, NaiveDateTime};

use crate::packfile::packfile::{process_line, to_pkt_line};

use crate::utils::files::files::{delete_all_files_and_folders, create_file_and_their_folders};
use crate::utils::random::random::Random;
use crate::vcs::commands::cat_file::CatFile;
use crate::vcs::commands::clone::Clone;
use crate::vcs::commands::init::Init;
use crate::vcs::entities::commit_entity::CommitEntity;
use crate::vcs::files::current_commit::CurrentCommit;
use crate::vcs::files::repository::Repository;



pub fn start_handler_receive(writer: &mut TcpStream, server_client_path: PathBuf) -> Result<String, std::io::Error> {
    println!("-----------start_handler_receive in-------------- \n\n");    
    println!("PATH CLIENTE   en start_handler_receive \n {:?}", server_client_path );

    let old_new_hash_commit = handler_receive_pack(writer)?;

    let (branch_name, last_commit_client )= extract_branch_name(old_new_hash_commit.to_string())?;
    
    println!("Received from packet: ---> {:?}", old_new_hash_commit); //lo recibe porque lo manda el cliente, pero daemon no hace nada con eso
    //ni crea la rama si no la tiene, pero eso si lo tenems que hacer almenos 
    let last_commit = get_last_commit(&server_client_path)?; 
    println!("MI LAST COMMIT DEL SERVER ES ---> {}\n", last_commit);

    select_update(writer,last_commit, server_client_path.clone())?;
    
    CurrentCommit::write_for_branch(&server_client_path, &branch_name, last_commit_client)?;
    writer.shutdown(Shutdown::Both)?;
    Ok("Respuesta desde start_handler_receive".to_string())
}


fn extract_branch_name(old_new_hash_commit: String) ->  Result<(String, String), std::io::Error> {
    let parts: Vec<&str> = old_new_hash_commit.split_whitespace().collect();
    let last_commit_client = parts[1];
    let branch_name = parts[2].trim_start_matches("refs/heads/");
    Ok((branch_name.to_owned(), last_commit_client.to_string()))
}

fn get_last_commit(server_client_path: &PathBuf) -> Result<String, io::Error> {
    println!("ESTOY en last commit");
    let current_log = Init::get_current_log(server_client_path)?;
    let log_content = fs::read_to_string(current_log)?;
    let last_line = log_content.lines().last().ok_or(io::Error::new(io::ErrorKind::InvalidData, "Log file is empty"))?;
    let line_parts: Vec<&str> = last_line.split("-").collect();
    println!("LINEPARTS {:?}", line_parts);
    let last_commit = line_parts.get(2).ok_or(io::Error::new(io::ErrorKind::InvalidData, "Log line is malformed"))?;
    Ok(last_commit.to_string())
}

fn select_update(writer: &mut TcpStream,last_commit: String, server_client_path: PathBuf) -> Result<(), std::io::Error>{

    let update_info = format!("{}\n",last_commit);
    let info_to_pkt_line = to_pkt_line(&update_info);
    println!("Mi pedido del server al cliente {:?}\n\n", info_to_pkt_line);
    writer.write(info_to_pkt_line.as_bytes())?;

    let msg_done = "0000";
    writer.write(msg_done.as_bytes())?;

    let mut refs = Vec::new();
    let mut send_refs = Vec::new(); // Cambiado a Vec<String>
    loop {
        let value = process_line(writer);
        match value {
            Ok(value) => {
                if value == "0" {
                    break;
                } else {
                    refs.push(value.clone());
                    send_refs.push(value);
                }                
            }
            Err(e) => {
                println!("Error al procesar la línea: {:?}", e);
                return Err(e);
            }
        }
    }
    
    println!("Mi lista que recibo de refs a enviar es:  --->{:?}\n" , send_refs);    

    // aca espero la PACKDATA
    let objects = Clone::get_socket_response(writer)?;

    updating_repo( objects, &server_client_path, last_commit)?;
    writer.flush()?;
    Ok(())
}

pub fn updating_repo( objects: Vec<(u8, Vec<u8>)>, repo_server_client: &PathBuf, last_commit: String) -> Result<(), std::io::Error> {
    let objects_prcessed = Clone::process_folder(objects.to_vec());

    for obj in &objects_prcessed{
        println!("------> {:?}", obj);
    }

    let commits_created = Clone::create_folders(objects_prcessed.clone(), &repo_server_client);
    println!("COMMITS CREATEDDDD ---> {:?}\n", commits_created);

    let hashes_sorted = sort_hashes(&commits_created);

    println!("HASH SORTED ---> {:?}\n", hashes_sorted);
    for(commit_hash, commit_entity ) in &hashes_sorted{
        write_commit_log_push(&commit_entity.parent_hash, &commit_hash, &commit_entity, repo_server_client.to_path_buf())?;
    }

    update_cd(&repo_server_client.clone())?;

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

pub fn update_cd(path: &PathBuf) -> Result<(), std::io::Error>{
    let repository_hashmap = Repository::read(path)?;

    delete_all_files_and_folders(path)?;

    for (key, value) in repository_hashmap{
        let content = CatFile::cat_file(&value, Init::get_object_path(path)?)?;
        create_file_and_their_folders(Path::new(&key), &content)?
    }
    Ok(())
}

fn handler_receive_pack(writer: &mut TcpStream) ->  Result<String, std::io::Error>{
    let response = process_line(writer)?;
    Ok(response)
}


/* 
fn parse_path_client(input: &str) -> String {
    let mut count = 0;
    let mut last_index = 0;

    for (index, c) in input.char_indices() {
        if c == '-' {
            count += 1;
            if count == 3 {
                last_index = index;
                break;
            }
        }
    }

    input[(last_index + 1)..].to_string()
}


fn handler_receive_pack(writer: &mut TcpStream, path: &PathBuf) ->  Result<String, std::io::Error>{
    let response = process_line(writer)?;
    Ok(response)
}

fn compare_content_folder(refs: String, server_repo : &PathBuf, path_client:String)-> (bool, String) {
    //"41e679a956b786ce9cb421fbbe7fcd3b5b6232a2 refs/head/master"
    //1. extraigo master por un lado y branch por otro 
    // busco si e l commit existe en mi file de commits acorde a la rama
    // si no existe significa que tengo que actualizar mi tablla de commit
    // le mando la ref y rama siempre otra vez
    let mut changes: (bool, String) = (false, String::new());
    let parts: Vec<&str> = refs.splitn(2, ' ').collect(); // Divide el elemento en dos partes.
    if parts.len() == 2 {
        let commit = parts[0];
        let ref_part = parts[1];
        if ref_part.starts_with("refs/") {
            let branch_name = ref_part.trim_start_matches("refs/heads/");
            println!("Commit: {}, Branch: {}", commit, branch_name);
            let (can_update, updated_branch, updated_commit) = validate_changes(commit, branch_name, server_repo, path_client);

            let formatted_update = format!("{} refs/heads/{}", updated_commit, updated_branch);
            changes = (can_update, formatted_update);

        }
        //guardo tupla (bool_accesible_tu_update, refs_rama) , de que si puede actualizar
    }
    changes
}


 */
       