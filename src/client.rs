use std::io::Write;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
static CLIENT_ARGS: usize = 3;

//comando para levantar el git daemon --> git daemon --base-path=. --export-all --reuseaddr --informative-errors --verbose --verbose
//comando para levantar cliente--> cargo run 127.0.0.1 9418

//Checkear que este main te lo tome como main momentaneo
pub fn client_(args: Vec<String>) -> Result<(), ()> {
    if args.len() != CLIENT_ARGS {
        println!("Cantidad de argumentos inválido");
        let app_name = &args[0];
        println!("{:?} <host> <puerto>", app_name);
        return Err(());
    }
    let address = args[1].clone() + ":" + &args[2];
    println!("Conectándome a {:?}", address);
    client_run(&address).unwrap();
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
fn process_received_line(line: &str) -> Option<String> {
    print!("Estoy recibiendo {:?}\n", line);
    if line.contains("HEAD") {
        return None;
    }
    let size = line.len() + 1; // Agregamos 1 para el espacio pq te lo cuenta
    let hex = format!("{:04x}", size);
    if line.starts_with(&hex) {
        let line = line.trim_start_matches(&hex);
        let parts = line.split(' ').collect::<Vec<&str>>();
        let hash = parts[0];
        return Some(hash.to_string());
    }
    None
}
fn print_socket_response(socket: &mut TcpStream) -> std::io::Result<()> {
    let reader = BufReader::new(socket);
    print!("Tuvo una respuesta {:?}\n", reader);
    for line in reader.lines() {
        if let Ok(line) = line {
            println!("Respuesta del servidor: {:?}", line);
        }
    }
    Ok(())
}

fn client_run(address: &str) -> std::io::Result<()> {
    let mut socket = TcpStream::connect(address)?;
    let msg = "git-upload-pack /Probando\0";
    let pkt_line = to_pkt_line(msg);
    socket.write(pkt_line.as_bytes())?;
    let reader = BufReader::new(&socket);
    let mut list_commits: Vec<String> = Vec::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            println!("Recibido: {:?}\n", line);
            if line == "0000" {
                //servidor termino de enviar las referencias
                break;
            }
            if let Some(commit_hash) = process_received_line(&line) {
                println!("Hash del commit: {}\n", commit_hash);
                list_commits.push(commit_hash.clone());
            }
        }
    }
    //let socket = reader.into_inner(); 
     //mando la siguiente query 
     for hash in list_commits{
        let msg_commit = format!("want {}\n", hash);                
        let pkt_commit = to_pkt_line(&msg_commit);
        print!("El mensaje vendria a ser: {:?}\n", pkt_commit);
        
        socket.write(pkt_commit.as_bytes())?;
        let _ = print_socket_response(&mut socket); 
    }
    let msg_done = "0000";
    let pkt_done = to_pkt_line(&msg_done);
    socket.write(pkt_done.as_bytes())?;

    let msg_done = "0009done\n";
    let pkt_done = to_pkt_line(&msg_done);
    socket.write(pkt_done.as_bytes())?;

    socket.flush()?;
    Ok(())
}