use std::io::{Write, Read};
use std::net::TcpStream;
use std::path::Path;
use std::str::from_utf8;

use flate2::read::ZlibDecoder;

use crate::handlers::add::handler_add;
use crate::handlers::cat_file::handler_cat_file;
use crate::handlers::commit::handler_commit;
use crate::handlers::status::handler_status;
use crate::vcs::commands::init::Init;
use crate::vcs::version_control_system::VersionControlSystem;

static CLIENT_ARGS: usize = 4;

//comando para levantar el git daemon --> git daemon --base-path=. --export-all --reuseaddr --informative-errors --verbose --verbose
//comando para levantar cliente--> cargo run 127.0.0.1 9418 /TEST2

//Checkear que este main te lo tome como main momentaneo
pub fn client_(args: Vec<String>) -> Result<(), ()> {
    if args.len() != CLIENT_ARGS {
        println!("Cantidad de argumentos inválido");
        println!("{:?} <host> <puerto>", &args[0]);
        return Err(());
    }
    
    let address = args[1].clone() + ":" + &args[2];
    if let Err(e) = client_run(&address, &args[3]) {
        println!("Error: {}",e);
    }
    Ok(())
}

fn to_pkt_line(msg: &str) -> String {
    let len = msg.len() + 4; 
    let hex = format!("{:04x}", len); 
    hex + msg 
}

/// Funcion que recibe el primer upload-receive-pack con este formato
/// 003d5b287f4198d774d33b43175f21270018c6517be6 refs/heads/main
/// 0042d143b748d2a0b186458ca5f6a13e51e6a8332cf3 refs/heads/otra_rama
/// Los primeros digitos representan la longitud de la linea en hexadecimal
/// Retora solo el hash del commit que tiene dentro
/* 
fn process_received_line(line: &str) -> Option<String> {
    print!("Estoy recibiendo {:?}\n", line);
    if line.contains("HEAD") {
        return None;
    }
    let size = line.len() + 1; // Agregamos 1 para el espacio pq te lo cuenta
    let hex = format!("{:04x}", size);
    if line.starts_with(&hex) {
        let line = line.trim_start_matches(&hex);
        //let parts = line.split(' ').collect::<Vec<&str>>();
        //let hash = line;
        return Some(line.to_string());
    }
    None
}
*/

fn read_packet(stream: &mut TcpStream, len: usize) -> String {
    let mut packet_buf = vec![0; len - 4];
    let _ = stream.read_exact(&mut packet_buf);
    String::from_utf8_lossy(&packet_buf).to_string()
}


fn receive_pack(socket: &mut TcpStream) -> Vec<String> {
    let mut packets = Vec::new();
    loop {
        let mut len_buf = [0; 4]; 
        if socket.read_exact(&mut len_buf).is_ok() {
            let len_str = from_utf8(&len_buf).unwrap();
            let len = usize::from_str_radix(len_str, 16).unwrap();
            if len == 0 {
                break;
            }
            let packet = read_packet(socket, len);
            packets.push(packet);
        }
    }
    packets
}

fn client_run(address: &str, path: &str) -> Result<(),std::io::Error> {
    let init_path = Path::new("/home/amoralejo/TEST2");
    let mut vcs = VersionControlSystem::init(init_path, Vec::new());
    //handler_status(&vcs);
    //let _ = handler_add(&mut vcs, "git add .".to_owned());
    handler_status(&vcs);
    //let _ = handler_commit(&mut vcs, "git commit test_commit".to_owned());

    
    
    println!("Conectándome a {:?}", address);
    let mut socket = TcpStream::connect(address)?;
    let msg = format!("git-upload-pack {}", path);
    let pkt_line = to_pkt_line(&msg);
    socket.write(pkt_line.as_bytes())?;
    
    let list_commits = receive_pack(&mut socket);    
    for want in get_want_msgs(list_commits) {
        socket.write(want.as_bytes())?;
    }
    send_done_msg(&mut socket)?;
    print_socket_response(&mut socket)?;

    // Para imprimir el cat file de .rustgit (cat-file ahora esta modificado para que lea de .git)
    //let response = handler_cat_file(&vcs, "git cat-file e87bc769b6a934012b58467455cd8aee1f583a3b".to_owned());
    //println!("Respuesta final: {:?}", response);
    
    // Para imprimir el cat que nos devuelve el want. (con commit real de git)
    let response = handler_cat_file(&vcs, "git cat-file a481a3e22ed24dee0b408dc35314ca0847a520ba".to_owned());
    println!("Respuesta final a48: {:?}", response);
    
    let response = handler_cat_file(&vcs, "git cat-file e87bc769b6a934012b58467455cd8aee1f583a3b".to_owned());
    println!("Respuesta final e87: {:?}", response);

    let response = handler_cat_file(&vcs, "git cat-file f87ac2a4ba1831e85cd8fceeb5131bdad5e70f0d".to_owned());
    println!("Respuesta final f87: {:?}", response);

    Ok(())

}

fn get_want_msgs(commits_list: Vec<String>) -> Vec<String> {
    let mut want_msgs = Vec::new();

    for commit in commits_list {
        let msg_commit = format!("want {}", commit);                
        let pkt_commit = to_pkt_line(&msg_commit);
        if commit.contains("HEAD"){
            continue;
        }
        want_msgs.push(pkt_commit);
    }
    want_msgs
}

fn print_socket_response(socket: &mut TcpStream) -> std::io::Result<()> {
    let mut buffer = Vec::new();
        match socket.read_to_end(&mut buffer) {
            Ok(_) => {
                match decompress_data((&buffer[22..]).to_vec()) {
                    Ok(decompressed_data) => {
                        let text = String::from_utf8_lossy(&decompressed_data);
                        println!("Datos descomprimidos: {}", text);
                    },
                    Err(e) => {
                        eprintln!("Error al descomprimir los datos: {}", e);
                    }
                }
            }
            Err(e) => println!("Failed to receive data: {}\n", e),
        } 
        Ok(())
}

fn send_done_msg(socket: &mut TcpStream) -> Result<(), std::io::Error> {
    let msg_done = "0000";
    socket.write(msg_done.as_bytes())?;

    let msg_done2 = "0009done\n";
    socket.write(msg_done2.as_bytes())?;
    Ok(())
}

fn decompress_data(compressed_data: Vec<u8>) -> Result<Vec<u8>, std::io::Error> {
    let mut decompressed_data = Vec::new();
    let mut decoder = ZlibDecoder::new(&compressed_data[..]);
    decoder.read_to_end(&mut decompressed_data)?;
    Ok(decompressed_data)
}