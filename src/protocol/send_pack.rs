
use std::{net::TcpStream, path::{Path, PathBuf}, io::Write};
use crate::{packfile::packfile::{process_line, to_pkt_line}, server::encoder::Encoder, vcs::{commands::branch::Branch, files::current_commit::CurrentCommit}};

pub fn handle_send_pack(stream:  &mut TcpStream, current_repo: &PathBuf, log_entries: &Vec<String>) -> Result<(), std::io::Error> {
    // aca leo lo que me responde el servidor 
    //por lo que son una lista de referencias de lo que puede actualizar 

    //dado el caso busco mi commit, y armo el paquete para enviarselo
    // empiezo por mandar el paquete directamente
    // y 0000
    //PACKDATA 
    //leer hasta que reciba un 0
    println!("Entro a handle--- send--pack \n");
    let mut refs = Vec::new();
    // Buffer para almacenar los datos leídos
    let mut send_refs = Vec::new(); // Cambiado a Vec<String>
    loop {
        let value = process_line(stream);
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

    //let vec = reformatted_hash_commit(send_refs, current_repo)?;
    let last_commit = CurrentCommit::read()?;
    
    let packfile = init_packfile(last_commit, current_repo)?;

    send_pack(packfile, stream, log_entries)?;
    //tengo mi vector de lo que quiere actualizar el server
   // init_send_pack(refs, stream, path)?;
    let msg_done = "0000";
    stream.write(msg_done.as_bytes())?;
    Ok(())
}


fn init_packfile(last_commit: String, current_repo: &PathBuf) -> Result<Vec<u8>,std::io::Error>{
    let mut packfile: Vec<u8> = Vec::new();

    let mut objects_data: Vec<(String,usize,usize)> = Vec::new();
    println!("CURREN REPO ---> {:?}\n", current_repo);
    println!("LAS COMMIT ---> {}\n", last_commit);
    Encoder::get_object_for_commit(&current_repo, &mut objects_data, &last_commit)?;
    
    println!("LEN OBJECTS {:?}\n", objects_data.len());
    println!("OBJECTS DATA: {:?}\n", objects_data);

    Encoder::create_size_header(&mut packfile, objects_data.len())?;

    for objects in objects_data.iter().rev() {
        let object_type = Encoder::set_bits(objects.1 as u8, objects.2)?;
        for object in object_type {
            packfile.push(object);
        }
        let path = Path::new(&objects.0);
        
        let compress_data = Encoder::compress_object((&path).to_path_buf(), objects.1)?;
        for byte in compress_data {
            packfile.push(byte);    
        }
    }
    Ok(packfile)
}

fn reformatted_hash_commit(send_ref: Vec<String>, current_repo: &PathBuf)-> Result<Vec<String>, std::io::Error> {
    let mut result = Vec::new();
    let current_branch = Branch::get_current_branch(&current_repo)?;

    let refs_branch = format!("refs/heads/{}", current_branch);

    for ref_info in send_ref {
        if let Some(null_index) = ref_info.find('\0') {
            result.push(ref_info[..null_index].to_string());
        } else if let Some(newline_index) = ref_info.find('\n') {
            result.push(ref_info[..newline_index].to_string());
        }
    }
    Ok(result)
}


fn send_pack(packfile: Vec<u8>, stream: &mut TcpStream, log_entries: &Vec<String>) -> Result<String, std::io::Error> {
    let entry_hash = format!("{}\n", log_entries[0]);
    stream.write(to_pkt_line(&entry_hash).as_bytes())?;
    println!("el mensaje de old y new antes del packfile --> {}\n", to_pkt_line(&log_entries[0]));

    let msg_done = "0000";
    stream.write(msg_done.as_bytes())?;

    stream.write_all(&packfile)?;
    println!("PAQUETE ENVIADO CON EXITO\n");
    Ok("0000".to_string())
}
