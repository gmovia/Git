use std::io::Write;
use std::net::TcpStream;
use std::path::PathBuf;
use crate::vcs::commands::{clone, push};
use crate::packfile::packfile::to_pkt_line;
use crate::constants::constants::{PUERTO, HOST};
use crate::vcs::commands::fetch::Fetch;

//comando para levantar el git daemon -->git daemon --base-path=. --export-all --reuseaddr --informative-errors --verbose --enable=receive-pack

pub struct Client;

impl Client {

    pub fn client(command: String, current_repository: &PathBuf) -> Result<(), ()> {
        let address = format!("{}:{}", HOST, PUERTO);

        if let Err(e) = Self::run_client(&address,  &command , &current_repository) {
            println!("Error: {}",e);
        }
        Ok(())
    }

    pub fn run_client(address: &str, command: &String, current_repository: &PathBuf) -> Result<(),std::io::Error> {
        println!("rust_client");
        let stream = TcpStream::connect(address)?;
        let _ = stream.try_clone()?;

        let _ = match command.as_str() {
        command_str if command_str.contains("git clone") => Self::handler_clone(stream, command, &current_repository),
        command_str if command_str.contains("git push") => Self::handler_push(stream,command,&current_repository),
        _ => Ok(()),

    };
       Ok(())
   }

    pub fn handler_clone(mut stream: TcpStream, command: &String, current_repository: &PathBuf) -> Result<(),std::io::Error>{
        let query_to_send = Self::handler_input(&command, &current_repository)?;
        let pkt_line = to_pkt_line(&query_to_send);
        print!("Query to_pkt_line : {:?} ---> \n", pkt_line);
        stream.write(pkt_line.as_bytes())?;
        let _ = Self::handler_query(&query_to_send, &mut stream, &current_repository, "clone");
        Ok(())
    }

    pub fn handler_fetch(mut stream: TcpStream, command: &String, current_repository: &PathBuf) -> Result<(),std::io::Error>{
        println!("CURRENT: {:?}", current_repository);
        let query_to_send = Self::handler_input(&command, &current_repository)?;
        let pkt_line = to_pkt_line(&query_to_send);
        print!("Query to_pkt_line : {:?} ---> \n", pkt_line);
        stream.write(pkt_line.as_bytes())?;
        let _ = Self::handler_query(&query_to_send, &mut stream, &current_repository, "fetch");
        Ok(())
    }

    fn handler_push(mut stream: TcpStream, command: &String, current_repository: &PathBuf) -> Result<(),std::io::Error>{
        let query_to_send = Self::handler_input(&command, &current_repository)?;
        let pkt_line = to_pkt_line(&query_to_send);
        print!("Query to_pkt_line : {:?} ---> \n", pkt_line);
        stream.write(pkt_line.as_bytes())?;
        let _ = Self::handler_query(&query_to_send, &mut stream, &current_repository);
        Ok(())
    }

    fn handler_input(input: &str, current_repository: &PathBuf) -> Result<String,std::io::Error> {
        match input {
            _ if input.contains("git clone") => {
                return Ok(format!("git-upload-pack /{}", current_repository.display()));
            },
            _ if input.contains("git push") => {
                return Ok(format!("git-receive-pack /{}\0host={}:{}\0\0version=2\0", current_repository.display(), HOST, PUERTO));
            },
            _ => Ok(input.to_string()),
        }
    }

    fn handler_query(query: &str, socket: &mut TcpStream, current_repository: &PathBuf, command_type: &str) -> Result<(),std::io::Error> {
            match query {
            command_str if command_str.contains("git-upload-pack") => clone::Clone::git_clone(socket, (&current_repository).to_path_buf()),
            command_str if command_str.contains("git-receive-pack ") =>  push::Push::push(socket, (&current_repository).to_path_buf()),
            _ => Ok(()),
        }
    }
    
}