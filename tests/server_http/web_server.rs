

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

}
