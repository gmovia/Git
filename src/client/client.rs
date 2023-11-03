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
    pub fn client_(command: String, puerto: String, host: String, server_repo: String) -> Result<(), ()> {
        
        let address = format!("{}:{}", puerto, host);

        if let Err(e) = Self::connect_rust_server(&address, &server_repo, &command) {
            println!("Error: {}",e);
        }
        
        if let Err(e) = Self::client_run(&address, &server_repo) {
            println!("Error: {}",e);
        }
        Ok(())
    }

    fn handler_input(input: &str) -> String {
        match input {
            _ if input.contains("git clone") => {
                let rest_of_input = input.trim_start_matches("git clone");
                format!("git-upload-pack{}", rest_of_input)
            }
            _ => input.to_string(),
        }
    }

    pub fn handler_clone(mut stream: TcpStream, command: &String) -> Result<(),std::io::Error>{
        let query_to_send = Self::handler_input(&command);
        let pkt_line = to_pkt_line(&query_to_send);
        print!("Query to_pkt_line : {:?} ---> \n", pkt_line);
        stream.write(pkt_line.as_bytes())?;
        Self::handler_query(&query_to_send, &mut stream); //rompe algo si mandó asi?
        Ok(())
    }

    pub fn connect_rust_server(address: &str, path: &str, command: &String) -> Result<(),std::io::Error> {
        println!("rust_client");
       // let mut vcs = VersionControlSystem::init(Path::new("test_folder"), Vec::new());
       let mut stream = TcpStream::connect("127.0.0.1:8080")?;

       let reader = stream.try_clone()?;

       let mut input = String::new();
       match command.as_str() {
        "git clone" => Self::handler_clone(stream, command),
        _ => Ok({}),
    };
       Ok(())
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


    fn handler_query(query: &str, socket: &mut TcpStream) {
        match (query.contains("git-upload-pack"), query.contains("git-send-pack")) {
            (true, _) => {
                 if let Err(e) = clone::Clone::clone(socket) {
                    println!("Error: {}", e);
                }           
                println!("Handling git-upload-pack request");
            }
            (_, true) => {
                println!("Handling git-send-pack request");
            }
            _ => {
                println!("Unknown request: {}", query);
            }
        }
    }
    
    fn client_run(address: &str, path: &str) -> Result<(),std::io::Error> {

        //println!("Conectándome a {:?}", address);
        //let mut socket = TcpStream::connect(address)?;
/*         let msg = format!("git-upload-pack {}", path);
        let pkt_line = to_pkt_line(&msg);
        socket.write(pkt_line.as_bytes())?;
        
        Self::handler_query("git-upload-pack",&mut socket ); */

        Ok(())

    }

}

