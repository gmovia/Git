use crate::constants::constant::{HOST, PUERTO};
use crate::packfiles::packfile::to_pkt_line;
use crate::vcs::commands::fetch::Fetch;
use crate::vcs::commands::{clone, push};
use crate::vcs::files::current_repository::CurrentRepository;
use std::io::Write;
use std::net::TcpStream;
use std::path::Path;

//comando para levantar el git daemon --> git daemon --base-path=. --export-all --reuseaddr --informative-errors --verbose --verbose
/// Este struct represeta al cliente
pub struct Client;

impl Client {
    /// Esta funcion sirve como inicializadora para el cliente cuando vamos a hacer uso de algun comado remoto.
    pub fn client(command: String, input_repository: &Path) -> Result<(), std::io::Error> {
        let address = format!("{}:{}", HOST, PUERTO);

        if Self::run_client(&address, &command, input_repository).is_err() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "fatal error: not a correct path",
            ));
        }
        Ok(())
    }

    /// Esta funcion se encarga de handlear los diferentes comando remotos posibles
    pub fn run_client(
        address: &str,
        command: &str,
        input_repository: &Path,
    ) -> Result<(), std::io::Error> {
        let stream = TcpStream::connect(address)?;
        let _ = stream.try_clone()?;

        match command {
            command_str if command_str.contains("git clone") => {
                Self::handler_clone(stream, command, input_repository)
            }
            command_str if command_str.contains("git fetch") => {
                Self::handler_fetch(stream, command, input_repository)
            }
            command_str if command_str.contains("git push") => {
                Self::handler_push(stream, command, input_repository)
            }
            _ => Ok(()),
        }
    }

    /// Esta funcion se encarga de handlear el comando clone
    pub fn handler_clone(
        mut stream: TcpStream,
        command: &str,
        input_repository: &Path,
    ) -> Result<(), std::io::Error> {
        let query_to_send = Self::handler_input(command, input_repository)?;
        let pkt_line = to_pkt_line(&query_to_send);
        let _ = stream.write(pkt_line.as_bytes());
        Self::handler_query(&query_to_send, &mut stream, "clone", input_repository)?;
        Ok(())
    }

    /// Esta funcion se encarga de handlear el comando push
    fn handler_push(
        mut stream: TcpStream,
        command: &str,
        input_repository: &Path,
    ) -> Result<(), std::io::Error> {
        let query_to_send = Self::handler_input(command, input_repository)?;
        let pkt_line = to_pkt_line(&query_to_send);
        stream.write_all(pkt_line.as_bytes())?;
        let _ = Self::handler_query(&query_to_send, &mut stream, "", input_repository);
        Ok(())
    }

    /// Esta funcion se encarga de handlear el comando fetch
    pub fn handler_fetch(
        mut stream: TcpStream,
        command: &str,
        input_repository: &Path,
    ) -> Result<(), std::io::Error> {
        let query_to_send = Self::handler_input(command, input_repository)?;
        let pkt_line = to_pkt_line(&query_to_send);

        let _ = stream.write(pkt_line.as_bytes());
        let _ = Self::handler_query(&query_to_send, &mut stream, "fetch", input_repository);
        Ok(())
    }

    /// Esta funcion se encarga de handlear el input que le vamos  enviar al servidor
    fn handler_input(input: &str, input_repository: &Path) -> Result<String, std::io::Error> {
        match input {
            _ if input.contains("git clone") => {
                Ok(format!("git-upload-pack /{}", input_repository.display()))
            }
            _ if input.contains("git fetch") => {
                Ok(format!("git-upload-pack /{}", input_repository.display()))
            }
            _ if input.contains("git push") => Ok(format!(
                "git-receive-pack /{}\0host={}:{}\0\0version=2\0",
                input_repository.display(),
                HOST,
                PUERTO
            )),
            _ => Ok(input.to_string()),
        }
    }

    fn handler_query(
        query: &str,
        socket: &mut TcpStream,
        command_type: &str,
        owner_repo: &Path,
    ) -> Result<(), std::io::Error> {
        let current_repo = CurrentRepository::read()?;
        match query {
            command_str if command_str.contains("git-upload-pack") && command_type == "clone" => {
                clone::Clone::git_clone(socket, &current_repo, owner_repo)
            }
            command_str if command_str.contains("git-upload-pack") && command_type == "fetch" => {
                Fetch::git_fetch(socket, &current_repo)
            }
            command_str if command_str.contains("git-receive-pack") => {
                push::Push::push(socket, &current_repo)
            }
            _ => Ok(()),
        }
    }
}
