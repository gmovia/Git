

#[cfg(test)]
mod tests {
    use std::{fs, path::Path, thread, net::TcpStream, io::{Write, Read}, time::Duration};

    use rust_git::{
        servers::server::Server,
        vcs::version_control_system::VersionControlSystem,
    };

    use crate::tests_functions::{commit_one_file, commit_one_file_client};

    #[test]
    pub fn test_01_test_400_bad_request() -> Result<(), std::io::Error> {
        let _ = thread::spawn(|| {
            let _ = Server::server("server".to_string());
        });

        let addres = "127.0.0.1:3000";

        match TcpStream::connect(addres) {
            Ok(mut stream) => {
                let mensaje = "Bad request";
                stream.write_all(mensaje.as_bytes()).expect("Error sending data");
    
                let mut buffer = [0; 1024];
                let _ = stream.read(&mut buffer).expect("Error reading servidor response");
                let respuesta = String::from_utf8_lossy(&buffer);
                assert!(respuesta.contains("HTTP/1.1 400 Bad Request"));
            }
            Err(e) => eprintln!("Error al conectar al servidor: {}", e),
        }
        Ok(())
    }

    #[test]
    pub fn test_02_test_create_pr_with_2_file_2_commit_2_branches() -> Result<(), std::io::Error> {
        let server_path = Path::new("server/amoralejo/algo1");

        let _ = thread::spawn(|| {
            let _ = Server::server("server".to_string());
        });

        VersionControlSystem::init(server_path, Vec::new());
        commit_one_file_client(server_path.to_path_buf(), "test_file_1.txt");
        let _ = VersionControlSystem::checkout(rust_git::vcs::commands::checkout::CheckoutOptions::CreateAndChangeBranch("new_branch"));
        commit_one_file_client(server_path.to_path_buf(), "test_file_2.txt");
        
        let addres = "127.0.0.1:3000";

        match TcpStream::connect(addres) {
            Ok(mut stream) => {
                let mensaje = [80, 79, 83, 84, 32, 47, 114, 101, 112, 111, 115, 47, 97, 109, 111, 114, 97, 108, 101, 106, 111, 47, 97, 108, 103, 111, 49, 47, 112, 117, 108, 108, 115, 32, 72, 84, 84, 80, 47, 49, 46, 49, 13, 10, 67, 111, 110, 116, 101, 110, 116, 45, 84, 121, 112, 101, 58, 32, 116, 101, 120, 116, 47, 112, 108, 97, 105, 110, 13, 10, 85, 115, 101, 114, 45, 65, 103, 101, 110, 116, 58, 32, 80, 111, 115, 116, 109, 97, 110, 82, 117, 110, 116, 105, 109, 101, 47, 55, 46, 51, 54, 46, 48, 13, 10, 65, 99, 99, 101, 112, 116, 58, 32, 42, 47, 42, 13, 10, 80, 111, 115, 116, 109, 97, 110, 45, 84, 111, 107, 101, 110, 58, 32, 97, 102, 54, 100, 98, 50, 54, 98, 45, 52, 102, 49, 49, 45, 52, 56, 99, 102, 45, 97, 50, 50, 50, 45, 48, 52, 97, 101, 101, 51, 102, 50, 52, 57, 100, 57, 13, 10, 72, 111, 115, 116, 58, 32, 108, 111, 99, 97, 108, 104, 111, 115, 116, 58, 51, 48, 48, 48, 13, 10, 65, 99, 99, 101, 112, 116, 45, 69, 110, 99, 111, 100, 105, 110, 103, 58, 32, 103, 122, 105, 112, 44, 32, 100, 101, 102, 108, 97, 116, 101, 44, 32, 98, 114, 13, 10, 67, 111, 110, 110, 101, 99, 116, 105, 111, 110, 58, 32, 107, 101, 101, 112, 45, 97, 108, 105, 118, 101, 13, 10, 67, 111, 110, 116, 101, 110, 116, 45, 76, 101, 110, 103, 116, 104, 58, 32, 49, 49, 55, 13, 10, 13, 10, 123, 13, 10, 32, 32, 32, 32, 34, 116, 105, 116, 108, 101, 34, 58, 32, 34, 97, 108, 103, 111, 49, 34, 44, 13, 10, 32, 32, 32, 32, 34, 98, 111, 100, 121, 34, 58, 32, 34, 77, 97, 116, 101, 114, 105, 97, 32, 97, 108, 103, 111, 49, 34, 44, 13, 10, 32, 32, 32, 32, 34, 104, 101, 97, 100, 34, 58, 32, 34, 97, 109, 111, 114, 97, 108, 101, 106, 111, 58, 110, 101, 119, 95, 98, 114, 97, 110, 99, 104, 34, 44, 13, 10, 32, 32, 32, 32, 34, 98, 97, 115, 101, 34, 58, 32, 34, 109, 97, 115, 116, 101, 114, 34, 32, 13, 10, 125];
                stream.write_all(&mensaje).expect("Error sending data");
    
                let mut buffer = [0; 1024];
                let _ = stream.read(&mut buffer).expect("Error reading server response");
                let respuesta = String::from_utf8_lossy(&buffer);
                assert!(respuesta.contains("HTTP/1.1 200 OK"));
            }
            Err(e) => eprintln!("Error al conectar al servidor: {}", e),
        }
        fs::remove_dir_all("server")?;
        Ok(())
    }

}
