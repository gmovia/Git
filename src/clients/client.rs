use std::io::Write;
use std::net::TcpStream;
use std::path::Path;
use crate::vcs::commands::{clone, push};
use crate::packfiles::packfile::to_pkt_line;
use crate::constants::constant::{PUERTO, HOST};
use crate::vcs::commands::fetch::Fetch;
use crate::vcs::files::current_repository::CurrentRepository;

//comando para levantar el git daemon --> git daemon --base-path=. --export-all --reuseaddr --informative-errors --verbose --verbose
pub struct Client;

impl Client {

    pub fn client(command: String, input_repository: &Path) -> Result<(), std::io::Error> {

        let address = format!("{}:{}", HOST, PUERTO);

        if Self::run_client(&address,  &command , input_repository).is_err() {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "fatal error: not a correct path"));
            //println!("Error: {}",e);
        }
        Ok(())
    }

    pub fn run_client(address: &str, command: &str, input_repository: &Path) -> Result<(),std::io::Error> {
        println!("rust_client");
        let stream = TcpStream::connect(address)?;
        let _ = stream.try_clone()?;

        match command {
        command_str if command_str.contains("git clone") => Self::handler_clone(stream, command, input_repository),
        command_str if command_str.contains("git fetch") => Self::handler_fetch(stream, command, input_repository),
        command_str if command_str.contains("git push") => Self::handler_push(stream,command, input_repository),
        _ => Ok(())
    }
   }

    pub fn handler_clone(mut stream: TcpStream, command: &str, input_repository: &Path) -> Result<(),std::io::Error>{
        let query_to_send = Self::handler_input(command, input_repository)?;
        let pkt_line = to_pkt_line(&query_to_send);
        let _ = stream.write(pkt_line.as_bytes());
        Self::handler_query(&query_to_send, &mut stream, "clone")?;
        Ok(())
    }

    fn handler_push(mut stream: TcpStream, command: &str, input_repository: &Path) -> Result<(),std::io::Error>{
        let query_to_send = Self::handler_input(command, input_repository)?;
        let pkt_line = to_pkt_line(&query_to_send);
        stream.write_all(pkt_line.as_bytes())?;
        let _ = Self::handler_query(&query_to_send, &mut stream, "");
        Ok(())
    }

    pub fn handler_fetch(mut stream: TcpStream, command: &str, input_repository: &Path) -> Result<(),std::io::Error>{
        println!("CURRENT: {:?}", input_repository);
        let query_to_send = Self::handler_input(command, input_repository)?;
        let pkt_line = to_pkt_line(&query_to_send);

        let _ = stream.write(pkt_line.as_bytes());
        let _ = Self::handler_query(&query_to_send, &mut stream, "fetch");
        Ok(())
    }

    fn handler_input(input: &str, input_repository: &Path) -> Result<String,std::io::Error> {
        match input {
            _ if input.contains("git clone") => {
                Ok(format!("git-upload-pack /{}", input_repository.display()))
            },
            _ if input.contains("git fetch") => {
                Ok(format!("git-upload-pack /{}", input_repository.display()))
            },
            _ if input.contains("git push") => {
                Ok(format!("git-receive-pack /{}\0host={}:{}\0\0version=2\0", input_repository.display(), HOST, PUERTO))
            },
            _ => Ok(input.to_string()),
        }
    }

    fn handler_query(query: &str, socket: &mut TcpStream, command_type: &str) -> Result<(),std::io::Error> {
            let current_repo = CurrentRepository::read()?;
            match query {
            command_str if command_str.contains("git-upload-pack") && command_type == "clone" => clone::Clone::git_clone(socket, &current_repo),
            command_str if command_str.contains("git-upload-pack") && command_type == "fetch" => Fetch::git_fetch(socket, &current_repo),
            command_str if command_str.contains("git-receive-pack") =>  push::Push::push(socket, &current_repo),
            _ => Ok(()),
        }
    }
    
}