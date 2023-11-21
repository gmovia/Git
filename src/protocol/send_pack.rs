
use std::{net::TcpStream, path::{Path, PathBuf}, io::Write};
use crate::{packfiles::packfile::{process_line, to_pkt_line}, servers::encoder::Encoder, vcs::{commands::{branch::Branch, init::Init}, files::current_commit::CurrentCommit}, constants::constant::COMMIT_INIT_HASH};

pub fn handle_send_pack(stream:  &mut TcpStream, current_repo: &PathBuf, log_entries: &Vec<String>) -> Result<(), std::io::Error> {
    // aca leo lo que me responde el servidor 
    //por lo que son una lista de referencias de lo que puede actualizar 

    //dado el caso busco mi commit, y armo el paquete para enviarselo
    // empiezo por mandar el paquete directamente
    // y 0000
    //PACKDATA 
    //leer hasta que reciba un 0
    println!("Entro a handle--- send--pack \n");
    // Buffer para almacenar los datos leídos
    let mut send_refs = Vec::new(); // Cambiado a Vec<String>
    loop {
        let value = process_line(stream);
        match value {
            Ok(value) => {
                if value == "0" {
                    break;
                } else {
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
    let last_commit_server = process_hash_server(&send_refs, (&current_repo).to_path_buf())?; //handlear despues para mas ramas
    let last_commit_current = CurrentCommit::read()?;

    let packfile = init_packfile(last_commit_current, current_repo, &last_commit_server)?;

    send_pack(packfile, stream, log_entries)?;

    let msg_done = "0000";
    stream.write(msg_done.as_bytes())?;
    Ok(())
}


fn process_hash_server(send_ref: &Vec<String>, current_repo: PathBuf) -> Result<String, std::io::Error>{
    let mut exist_branch_in_server = false;
    let mut last_commit_server= String::new();

    for refs in send_ref{
        if refs.contains(&Init::get_current_branch(&current_repo)?){
            exist_branch_in_server = true;
            let parts: Vec<&str>  = refs.split_ascii_whitespace().collect();
            last_commit_server = parts[0].to_string();
            break;
        }
    }
    if exist_branch_in_server == false {
        last_commit_server = COMMIT_INIT_HASH.to_string();
    }
    Ok(last_commit_server)
}


fn init_packfile(last_commit_current: String, current_repo: &PathBuf, last_commit_server: &String) -> Result<Vec<u8>,std::io::Error>{
    let mut packfile: Vec<u8> = Vec::new();

    let mut objects_data: Vec<(String,usize,usize)> = Vec::new();
    println!("CURREN REPO ---> {:?}\n", current_repo);
    println!("LAS COMMIT ---> {}\n", last_commit_current);
    Encoder::get_object_for_commit(&current_repo, &mut objects_data, &last_commit_current, &last_commit_server)?;
    
    println!("LEN OBJECTS {:?}\n", objects_data.len());
    println!("OBJECTS DATA: {:?}\n", objects_data);

    Encoder::create_size_header(&mut packfile, objects_data.len())?;

    for objects in objects_data.iter().rev() {
        let object_type = Encoder::set_bits(objects.1 as u8, objects.2)?;
        for object in object_type {
            packfile.push(object);
        }
        let path = Path::new(&objects.0);
        
        let compress_data = Encoder::compress_object(path.clone(), objects.1)?;
        for byte in compress_data {
            packfile.push(byte);    
        }
    }
    Ok(packfile)
}

fn send_pack(packfile: Vec<u8>, stream: &mut TcpStream, log_entries: &Vec<String>) -> Result<String, std::io::Error> {
    let entry_hash = format!("{}\n", log_entries[0]);
    println!("LOG ENTIRES ---> {:?}", entry_hash);
    stream.write(to_pkt_line(&entry_hash).as_bytes())?;
    println!("el mensaje de old y new antes del packfile --> {}\n", to_pkt_line(&entry_hash));

    let msg_done = "0000";
    stream.write(msg_done.as_bytes())?;

    stream.write_all(&packfile)?;
    println!("PAQUETE ENVIADO CON EXITO\n");
    Ok("0000".to_string())
}
