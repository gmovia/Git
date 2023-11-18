use std::fs::{OpenOptions, self};
use std::io::{Write, self, BufRead};
use std::net::Shutdown;
use std::path::Path;
use std::{net::TcpStream, path::PathBuf};

use crate::packfile::packfile::{process_line, to_pkt_line};

use crate::proxy::proxy::Proxy;
use crate::vcs::commands::clone::Clone;
use crate::vcs::commands::init::Init;



pub fn start_handler_receive(writer: &mut TcpStream, server_client_path: PathBuf) -> Result<String, std::io::Error> {
    println!("-----------start_handler_receive in-------------- \n\n");    
    println!("PATH CLIENTE   en start_handler_receive \n {:?}", server_client_path );

    let old_new_hash_commit = handler_receive_pack(writer)?;

    println!("Received from packet: ---> {:?}", old_new_hash_commit); //lo recibe porque lo manda el cliente, pero daemon no hace nada con eso
    //ni crea la rama si no la tiene, pero eso si lo tenems que hacer almenos 
    let last_commit = get_last_commit(&server_client_path)?; //si esto lo corro del server me da entonces test_folder/clone ? y su ultimo commit?
    //porque quiero el ultimo commit pero de la repo del server
    println!("MI LAST COMMIT DEL SERVER ES ---> {}\n", last_commit);
    select_update(writer,last_commit, server_client_path.clone())?;
    
    writer.shutdown(Shutdown::Both)?;
    Ok("Respuesta desde start_handler_receive".to_string())
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
    updating_repo( objects, server_client_path)?;
    writer.flush()?;

    Ok(())
}

pub fn updating_repo( objects: Vec<(u8, Vec<u8>)>, repo_server_client: PathBuf) -> Result<(), std::io::Error> {
    let objects_prcessed = Clone::process_folder(objects.to_vec());
    for obj in &objects_prcessed{
        println!("------> {:?}", obj);
    }
    let commits_created = Clone::create_folders(objects_prcessed.clone(), &repo_server_client);
    let (clave, valor) = commits_created.iter().next().unwrap();
    
    Proxy::write_commit(repo_server_client, valor)?;
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
       