
use std::{net::TcpStream, path::Path, io::{Write, Read}};
use crate::{packfile::packfile::process_line, server::encoder::Encoder};

pub fn handle_send_pack(stream:  &mut TcpStream, path: &Path) -> Result<(), std::io::Error> {
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
    let end = false;
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
    for entry in &send_refs{
        println!("---> {}", entry);
    }
    //tengo mi vector de lo que quiere actualizar el server
   // init_send_pack(refs, stream, path)?;
    let msg_done = "0000";
    stream.write(msg_done.as_bytes())?;
    Ok(())
}


fn init_send_pack(refs: Vec<String>, stream: &mut TcpStream, path: &Path) -> Result<String, std::io::Error> {
    // S: 003f74730d410fcb6603ace96f1dc55ea6196122532d refs/heads/master\n
    // lo que yo voy a procesar ---> 03f 74730d410fcb6603ace96f1dc55ea6196122532d refs/heads/master\n
    
    println!("ESTE ES EL PACK QUE VOY A USAR ::::::::::::::{:?}\n" , path);
    let packfile_result = Encoder::init_encoder(path.to_path_buf(), (Vec::new(), Vec::new()));

    match packfile_result {
        Ok(packfile) => {
            stream.write(&packfile)?;
            println!("--------------------------------ENVIADO\n");
        }
        Err(e) => {
            println!("Error al inicializar el packfile: {:?}", e);
        }
    }
    Ok("0000".to_string())
}