use std::io::{Write};
use std::net::TcpStream;
use crate::vcs::commands::clone;
use crate::packfile::{decompress_data, to_pkt_line};

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


fn handler_query(query: &str,socket: &mut TcpStream ) {    
    match query {
        "git-upload-pack" => {
            if let Err(e) = clone::Clone::clone(socket) {
                println!("Error: {}", e);
            }            
            println!("Handling git-upload-pack request");
        }
        "git-send-pack" => {
            println!("Handling git-send-pack request");
        }
        _ => {
            println!("Unknown request: {}", query);
        }
    }
}

fn client_run(address: &str, path: &str) -> Result<(),std::io::Error> {

    println!("Conectándome a {:?}", address);
    let mut socket = TcpStream::connect(address)?;
    let msg = format!("git-upload-pack {}", path);
    let pkt_line = to_pkt_line(&msg);
    socket.write(pkt_line.as_bytes())?;
    
    handler_query("git-upload-pack",&mut socket );

    Ok(())

}