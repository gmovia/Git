use std::io::{Write, Read};
use std::net::TcpStream;
use std::str::from_utf8;
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
        //let parts = line.split(' ').collect::<Vec<&str>>();
        //let hash = line;
        return Some(line.to_string());
    }
    None
}

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
            print!("\n READING MY PACKET ------> {:?} \n", packet);
            packets.push(packet);
        }
    }
    packets
}

fn print_socket_response(socket: &mut TcpStream) -> std::io::Result<()> {
    let mut buffer = Vec::new();
        match socket.read_to_end(&mut buffer) {
            Ok(_) => {
                println!("Received: {:?}\n", buffer);
            }
            Err(e) => println!("Failed to receive data: {}\n", e),
        } 
    Ok(())
}

fn client_run(address: &str) -> std::io::Result<()> {
    let mut socket = TcpStream::connect(address)?;
    let msg = "git-upload-pack /Probando\0";
    let pkt_line = to_pkt_line(msg);
    socket.write(pkt_line.as_bytes())?;

    let list_commits = receive_pack(&mut socket);

    print!("\nTu lista es esta -----> {:?} \n", list_commits);
     //mando la siguiente query 
     for hash in list_commits{
        let msg_commit = format!("want {}", hash);                
        let pkt_commit = to_pkt_line(&msg_commit);
        if hash.contains("HEAD"){
            continue;
        }
        print!("El mensaje vendria a ser: {:?}\n", pkt_commit);
        
        socket.write(pkt_commit.as_bytes())?;
    }

    let msg_done = "0000";
    socket.write(msg_done.as_bytes())?;

    let msg_done2 = "0009done\n";
    socket.write(msg_done2.as_bytes())?;

    print_socket_response(&mut socket)?;
    Ok(())

}