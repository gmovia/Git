use std::io::Write;
use std::net::TcpStream;
use std::path::Path;
use crate::vcs::commands::clone;
use crate::packfiles::packfile::to_pkt_line;
use crate::constants::constant::{PUERTO, HOST};
use crate::vcs::commands::fetch::Fetch;

//comando para levantar el git daemon --> git daemon --base-path=. --export-all --reuseaddr --informative-errors --verbose --verbose
pub struct Client;

impl Client {

    pub fn client(command: String, current_repository: &Path) -> Result<(), std::io::Error> {

        let address = format!("{}:{}", HOST, PUERTO);

        if Self::run_client(&address,  &command , current_repository).is_err() {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "fatal error: not a correct path"));
            //println!("Error: {}",e);
        }
        Ok(())
    }

    pub fn run_client(address: &str, command: &str, current_repository: &Path) -> Result<(),std::io::Error> {
        println!("rust_client");
        let stream = TcpStream::connect(address)?;
        let _ = stream.try_clone()?;

        match command {
        command_str if command_str.contains("git clone") => Self::handler_clone(stream, command, current_repository),
        command_str if command_str.contains("git fetch") => Self::handler_fetch(stream, command, current_repository),
        _ => Ok(())
    }
   }

    pub fn handler_clone(mut stream: TcpStream, command: &str, current_repository: &Path) -> Result<(),std::io::Error>{
        let query_to_send = Self::handler_input(command, current_repository)?;
        let pkt_line = to_pkt_line(&query_to_send);
        let _ = stream.write(pkt_line.as_bytes());
        Self::handler_query(&query_to_send, &mut stream, current_repository, "clone")?;
        Ok(())
    }

    pub fn handler_fetch(mut stream: TcpStream, command: &str, current_repository: &Path) -> Result<(),std::io::Error>{
        println!("CURRENT: {:?}", current_repository);
        let query_to_send = Self::handler_input(command, current_repository)?;
        let pkt_line = to_pkt_line(&query_to_send);

        let _ = stream.write(pkt_line.as_bytes());
        let _ = Self::handler_query(&query_to_send, &mut stream, current_repository, "fetch");
        Ok(())
    }

    fn handler_input(input: &str, current_repository: &Path) -> Result<String,std::io::Error> {
        match input {
            _ if input.contains("git clone") => {
                Ok(format!("git-upload-pack /{}", current_repository.display()))
            },
            _ if input.contains("git fetch") => {
                Ok(format!("git-upload-pack /{}", current_repository.display()))
            },
            _ => Ok(input.to_string()),
        }
    }

    fn handler_query(query: &str, socket: &mut TcpStream, current_repository: &Path, command_type: &str) -> Result<(),std::io::Error> {
            match query {
            command_str if command_str.contains("git-upload-pack") && command_type == "clone" => clone::Clone::git_clone(socket, current_repository),
            command_str if command_str.contains("git-upload-pack") && command_type == "fetch" => Fetch::git_fetch(socket, current_repository),
            _ => Ok(()),
        }
    }
    
}