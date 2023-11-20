use std::io::Write;
use std::net::TcpStream;
use std::path::Path;
use crate::vcs::commands::clone;
use crate::packfiles::packfile::to_pkt_line;
use crate::constants::constant::{PUERTO, HOST};

//comando para levantar el git daemon --> git daemon --base-path=. --export-all --reuseaddr --informative-errors --verbose --verbose
pub struct Client;

impl Client {

    pub fn client(command: String, current_repository: &Path) -> Result<(), std::io::Error> {
        let address = format!("{}:{}", HOST, PUERTO);

        if let Err(e) = Self::run_client(&address,  &command , current_repository) {
            println!("Error: {}",e);
        }
        Ok(())
    }

    pub fn run_client(address: &str, command: &str, current_repository: &Path) -> Result<(),std::io::Error> {
        println!("rust_client");
        let stream = TcpStream::connect(address)?;
        stream.try_clone()?;

        let _ = match command {
        command_str if command_str.contains("git clone") => Self::handler_clone(stream, command, current_repository),
        _ => Ok(()),

    };
       Ok(())
   }

    pub fn handler_clone(mut stream: TcpStream, command: &str, current_repository: &Path) -> Result<(),std::io::Error>{
        let query_to_send = Self::handler_input(command, current_repository)?;
        let pkt_line = to_pkt_line(&query_to_send);
        let _ =stream.write(pkt_line.as_bytes())?;
        let _ = Self::handler_query(&query_to_send, &mut stream, current_repository);
        Ok(())
    }

    fn handler_input(input: &str, current_repository: &Path) -> Result<String,std::io::Error> {
        match input {
            _ if input.contains("git clone") => {
                Ok(format!("git-upload-pack /{}", current_repository.display()))
            },
            _ => Ok(input.to_string()),
        }
    }

    fn handler_query(query: &str, socket: &mut TcpStream, current_repository: &Path) -> Result<(),std::io::Error> {
            match query {
            command_str if command_str.contains("git-upload-pack") => clone::Clone::git_clone(socket, current_repository.to_path_buf()),
            _ => Ok(()),
        }
    }
    
}