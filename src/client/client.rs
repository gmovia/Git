use std::io::{Write, Read, self};
use std::net::TcpStream;
use std::path::Path;
use crate::vcs::commands::clone;
use crate::packfile::packfile::to_pkt_line;
use crate::vcs::version_control_system::VersionControlSystem;

static CLIENT_ARGS: usize = 4;

//comando para levantar el git daemon --> git daemon --base-path=. --export-all --reuseaddr --informative-errors --verbose --verbose
//comando para levantar cliente--> cargo run 127.0.0.1 9418 /TEST2

pub struct Client;

impl Client {

    //Checkear que este main te lo tome como main momentaneo
    pub fn client_(args: Vec<String>) -> Result<(), ()> {
        if args.len() < CLIENT_ARGS {
            println!("Cantidad de argumentos inválido");
            println!("{:?} <host> <puerto>", &args[0]);
            return Err(());
        }
        
        let address = args[1].clone() + ":" + &args[2];

        if let Err(e) = Self::connect_rust_server(&address, &args[3]) {
            println!("Error: {}",e);
        }
        
        if let Err(e) = Self::client_run(&address, &args[3]) {
            println!("Error: {}",e);
        }
        Ok(())
    }

    pub fn connect_rust_server(address: &str, path: &str) -> Result<(),std::io::Error> {
        println!("rust_client");
        let mut vcs = VersionControlSystem::init(Path::new("test_folder"), Vec::new());
        
        let mut stream = TcpStream::connect("127.0.0.1:8080")?;

        let reader = stream.try_clone()?;

        std::thread::spawn(move || {
            Self::handle_server_response(reader);
        });

        let mut input = String::new();
        loop {
            io::stdin().read_line(&mut input)?;
            stream.write(input.as_bytes())?;
            input.clear();
        }
    }


    fn handle_server_response(mut reader: TcpStream) {
        let mut buffer = [0; 1024];
        loop {
            match reader.read(&mut buffer) {
                Ok(0) => {
                    break;
                }
                Ok(n) => {
                    let response = String::from_utf8_lossy(&buffer[..n]);
                    println!("Server response: {}", response);
                    buffer = [0; 1024];
                }
                Err(e) => {
                    eprintln!("Error reading from server: {}", e);
                    break;
                }
            }
        }
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
        
        Self::handler_query("git-upload-pack",&mut socket );

        Ok(())

    }

}

