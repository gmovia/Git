use std::io::Write;
use std::path::Path;
use std::{net::TcpStream, path::PathBuf};

use crate::packfile::packfile::{process_line, to_pkt_line};

use crate::vcs::commands::clone::Clone;



pub fn start_handler_receive(writer: &mut TcpStream, path: &PathBuf) -> Result<String, std::io::Error> {
    println!("-----------start_handler_receive in-------------- \n\n");
    // path = test_folder/test6
    
    println!("PATH CLIENTE   en start_handler_receive \n {:?}", path );
    let old_new_hash_commit = handler_receive_pack(writer, path)?;

    println!("Received from packet: ---> {:?}", old_new_hash_commit);

    //let different_founded = compare_content_folder(first_response, &server_repo, path_client.clone());
    //println!("DIFERENT FOUNDED ---> {:?}", different_founded);
    println!("PATH QEUE ENTRA A UPLOAD pakc ---> {:?}\n",  path);
    //select_update(writer,different_founded, &server_repo, &pathClient)?;
    //writer.shutdown(Shutdown::Both)?;
    //reader.shutdown(Shutdown::Both)?;  
    Ok("Respuesta desde start_handler_receive".to_string())
}


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

fn validate_changes<'a>(commit: &'a str, branch: &'a str, server_repo: &'a PathBuf, path_client: String) -> (bool, String, &'a str) {
    let logs_path = server_repo.join(path_client).join(".rust_git").join("logs");
    //busco el archivo que tenga de nombre branch, entro a leer, y me fijo si el ultimo
    //commit es el que me llego por parametro como commit
    //si es el mismo, mando false, si no es el mismo mando true
    //devuelve entonces una tupla de bool y branch
    let branch_log_file = logs_path.join(branch);

    let contents = match std::fs::read_to_string(&branch_log_file) {
        Ok(content) => content,
        Err(_) => {
            return (true, branch.to_string(), commit);
        }
    };

    let lines: Vec<&str> = contents.lines().collect();

    if let Some(last_commit) = lines.last() {
        if last_commit.contains(&commit) {
            return (false, branch.to_string(), commit);
        }
    }
    (true, branch.to_string(), commit)
}


fn select_update(writer: &mut TcpStream, changes: (bool, String), server_repo : &PathBuf, path_client: &String) -> Result<(), std::io::Error>{
    let mut packets = Vec::new();
    for (can_update, formatted_update) in [changes] {
        if can_update {
            let update_info = format!("{}\n",formatted_update);
            packets.push(update_info.clone());
            // S: 003f74730d410fcb6603ace96f1dc55ea6196122532d refs/heads/master\n
            let info_to_pkt_line = to_pkt_line(&update_info);
            println!("Mi pedido del server al cliente {:?}\n\n", info_to_pkt_line);
            writer.write(info_to_pkt_line.as_bytes())?;
        }
    }
    let msg_done = "0000";
    writer.write(msg_done.as_bytes())?;

    let init_repo = server_repo.join(path_client);
    let init_path = Path::new(&init_repo);
    
    //RECIBO EL PAQEUTE Y DESEMPAQUETO Y ME LO GUARDO
    //este vcs pertenece a la repo remota.
    let objects = Clone::get_socket_response(writer)?;
    println!("Mis packets son -----------> {:?}\n", packets );
    
    updating_repo(&packets, init_path, objects,(path_client).to_string())?;
    writer.flush()?;

    Ok(())
}



pub fn updating_repo(packets: &Vec<String>, vcs_path: &Path, objects: Vec<(u8, Vec<u8>)>, path_client: String) -> Result<(), std::io::Error> {
    for item in packets {
        let parts: Vec<&str> = item.splitn(2, ' ').collect(); // Divide el elemento en dos partes.
        println!(" PARTS : {:?} \n", parts);
        if parts.len() == 2 {
            let commit = parts[0];
            let ref_part = parts[1];
            let branch_name = ref_part.trim_start_matches("refs/heads/");
            //write_commit_log(vcs_path, branch_name, commit)?;
            let mut tree_hash = String::new();
            
            let path = format!("{}/.rust_git/objects", vcs_path.display());
            for object in &objects {
                let object1_str = std::str::from_utf8(&object.1).expect("No es una cadena UTF-8 válida");
                match object.0 {
                    1 => {
                        println!("SOY OBJETO DE TIPO 1 {:?}", &object.1);
                        //tree_hash = Clone::create_commit_folder(vcs, &object.1, commit)?;
                    }
                    2 => {
                        //let hash_tree = create_folder(&object1_str, vcs_path, path_client.clone())?;        
                     }
                    3 => {
                        //let hash_blob = create_folder(&object1_str, vcs_path, path_client.clone())?;
                        //print!("TIPO 3 tengo {}", hash_blob);
                    }
                    _ => return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Unknown object")),
                }
            }
        }
    }
    //creates_files(&vcs_path)?;
    Ok(())
}


       