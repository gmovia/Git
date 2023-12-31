use std::{fs, path::Path};

#[cfg(test)]
mod tests {
    use std::{
        fs,
        io::{Read, Write},
        net::TcpStream,
        path::Path,
        thread,
    };

    use rust_git::{
        server_http::web_server::WebServer,
        vcs::{files::config::Config, version_control_system::VersionControlSystem},
    };

    use crate::{server_http::web_server::words_counter, tests_functions::commit_one_file_client};

    use super::read_directory;

    #[test]
    pub fn test_01_test_400_bad_request() -> Result<(), std::io::Error> {
        let _ = thread::spawn(|| {
            let _ = WebServer::new_listen("server".to_string().into());
        });

        let addres = "127.0.0.1:3000";

        match TcpStream::connect(addres) {
            Ok(mut stream) => {
                let mensaje = "Bad request";
                stream
                    .write_all(mensaje.as_bytes())
                    .expect("Error sending data");

                let mut buffer = [0; 1024];
                let _ = stream
                    .read(&mut buffer)
                    .expect("Error reading servidor response");
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
            let _ = WebServer::new_listen("server".to_string().into());
        });

        VersionControlSystem::init(server_path, Vec::new());
        Config::write_config(("Agus".to_owned(), "amoralejo@fi.uba.ar".to_owned()))?;
        commit_one_file_client(server_path.to_path_buf(), "test_file_1.txt");
        let _ = VersionControlSystem::checkout(
            rust_git::vcs::commands::checkout::CheckoutOptions::CreateAndChangeBranch("new_branch"),
        );
        commit_one_file_client(server_path.to_path_buf(), "test_file_2.txt");

        let addres = "127.0.0.1:3000";

        match TcpStream::connect(addres) {
            Ok(mut stream) => {
                let mensaje = [
                    80, 79, 83, 84, 32, 47, 114, 101, 112, 111, 115, 47, 97, 109, 111, 114, 97,
                    108, 101, 106, 111, 47, 97, 108, 103, 111, 49, 47, 112, 117, 108, 108, 115, 32,
                    72, 84, 84, 80, 47, 49, 46, 49, 13, 10, 67, 111, 110, 116, 101, 110, 116, 45,
                    84, 121, 112, 101, 58, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111, 110,
                    47, 106, 115, 111, 110, 13, 10, 85, 115, 101, 114, 45, 65, 103, 101, 110, 116,
                    58, 32, 80, 111, 115, 116, 109, 97, 110, 82, 117, 110, 116, 105, 109, 101, 47,
                    55, 46, 51, 54, 46, 48, 13, 10, 65, 99, 99, 101, 112, 116, 58, 32, 42, 47, 42,
                    13, 10, 80, 111, 115, 116, 109, 97, 110, 45, 84, 111, 107, 101, 110, 58, 32,
                    97, 55, 56, 54, 56, 56, 51, 102, 45, 101, 50, 102, 52, 45, 52, 55, 52, 48, 45,
                    56, 51, 97, 51, 45, 51, 54, 101, 53, 54, 97, 102, 100, 51, 52, 54, 54, 13, 10,
                    72, 111, 115, 116, 58, 32, 108, 111, 99, 97, 108, 104, 111, 115, 116, 58, 51,
                    48, 48, 48, 13, 10, 65, 99, 99, 101, 112, 116, 45, 69, 110, 99, 111, 100, 105,
                    110, 103, 58, 32, 103, 122, 105, 112, 44, 32, 100, 101, 102, 108, 97, 116, 101,
                    44, 32, 98, 114, 13, 10, 67, 111, 110, 110, 101, 99, 116, 105, 111, 110, 58,
                    32, 107, 101, 101, 112, 45, 97, 108, 105, 118, 101, 13, 10, 67, 111, 110, 116,
                    101, 110, 116, 45, 76, 101, 110, 103, 116, 104, 58, 32, 49, 49, 55, 13, 10, 13,
                    10, 123, 13, 10, 32, 32, 32, 32, 34, 116, 105, 116, 108, 101, 34, 58, 32, 34,
                    97, 108, 103, 111, 49, 34, 44, 13, 10, 32, 32, 32, 32, 34, 98, 111, 100, 121,
                    34, 58, 32, 34, 77, 97, 116, 101, 114, 105, 97, 32, 97, 108, 103, 111, 49, 34,
                    44, 13, 10, 32, 32, 32, 32, 34, 104, 101, 97, 100, 34, 58, 32, 34, 97, 109,
                    111, 114, 97, 108, 101, 106, 111, 58, 110, 101, 119, 95, 98, 114, 97, 110, 99,
                    104, 34, 44, 13, 10, 32, 32, 32, 32, 34, 98, 97, 115, 101, 34, 58, 32, 34, 109,
                    97, 115, 116, 101, 114, 34, 32, 13, 10, 125,
                ];
                stream.write_all(&mensaje).expect("Error sending data");

                let mut buffer = [0; 1024];
                let _ = stream
                    .read(&mut buffer)
                    .expect("Error reading server response");
                let respuesta = String::from_utf8_lossy(&buffer);
                assert!(respuesta.contains("HTTP/1.1 200 OK"));
            }
            Err(e) => eprintln!("Error al conectar al servidor: {}", e),
        }
        fs::remove_dir_all("server")?;
        Ok(())
    }

    #[test]
    pub fn test_03_test_branch_not_found_creating_pr() -> Result<(), std::io::Error> {
        let server_path = Path::new("server/amoralejo/algo1");

        let _ = thread::spawn(|| {
            let _ = WebServer::new_listen("server".to_string().into());
        });

        VersionControlSystem::init(server_path, Vec::new());
        Config::write_config(("Agus".to_owned(), "amoralejo@fi.uba.ar".to_owned()))?;
        commit_one_file_client(server_path.to_path_buf(), "test_file_1.txt");
        let _ = VersionControlSystem::checkout(
            rust_git::vcs::commands::checkout::CheckoutOptions::CreateAndChangeBranch("bad_branch"),
        );
        commit_one_file_client(server_path.to_path_buf(), "test_file_2.txt");

        let addres = "127.0.0.1:3000";

        match TcpStream::connect(addres) {
            Ok(mut stream) => {
                let mensaje = [
                    80, 79, 83, 84, 32, 47, 114, 101, 112, 111, 115, 47, 97, 109, 111, 114, 97,
                    108, 101, 106, 111, 47, 97, 108, 103, 111, 49, 47, 112, 117, 108, 108, 115, 32,
                    72, 84, 84, 80, 47, 49, 46, 49, 13, 10, 67, 111, 110, 116, 101, 110, 116, 45,
                    84, 121, 112, 101, 58, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111, 110,
                    47, 106, 115, 111, 110, 13, 10, 85, 115, 101, 114, 45, 65, 103, 101, 110, 116,
                    58, 32, 80, 111, 115, 116, 109, 97, 110, 82, 117, 110, 116, 105, 109, 101, 47,
                    55, 46, 51, 54, 46, 48, 13, 10, 65, 99, 99, 101, 112, 116, 58, 32, 42, 47, 42,
                    13, 10, 80, 111, 115, 116, 109, 97, 110, 45, 84, 111, 107, 101, 110, 58, 32,
                    97, 55, 56, 54, 56, 56, 51, 102, 45, 101, 50, 102, 52, 45, 52, 55, 52, 48, 45,
                    56, 51, 97, 51, 45, 51, 54, 101, 53, 54, 97, 102, 100, 51, 52, 54, 54, 13, 10,
                    72, 111, 115, 116, 58, 32, 108, 111, 99, 97, 108, 104, 111, 115, 116, 58, 51,
                    48, 48, 48, 13, 10, 65, 99, 99, 101, 112, 116, 45, 69, 110, 99, 111, 100, 105,
                    110, 103, 58, 32, 103, 122, 105, 112, 44, 32, 100, 101, 102, 108, 97, 116, 101,
                    44, 32, 98, 114, 13, 10, 67, 111, 110, 110, 101, 99, 116, 105, 111, 110, 58,
                    32, 107, 101, 101, 112, 45, 97, 108, 105, 118, 101, 13, 10, 67, 111, 110, 116,
                    101, 110, 116, 45, 76, 101, 110, 103, 116, 104, 58, 32, 49, 49, 55, 13, 10, 13,
                    10, 123, 13, 10, 32, 32, 32, 32, 34, 116, 105, 116, 108, 101, 34, 58, 32, 34,
                    97, 108, 103, 111, 49, 34, 44, 13, 10, 32, 32, 32, 32, 34, 98, 111, 100, 121,
                    34, 58, 32, 34, 77, 97, 116, 101, 114, 105, 97, 32, 97, 108, 103, 111, 49, 34,
                    44, 13, 10, 32, 32, 32, 32, 34, 104, 101, 97, 100, 34, 58, 32, 34, 97, 109,
                    111, 114, 97, 108, 101, 106, 111, 58, 110, 101, 119, 95, 98, 114, 97, 110, 99,
                    104, 34, 44, 13, 10, 32, 32, 32, 32, 34, 98, 97, 115, 101, 34, 58, 32, 34, 109,
                    97, 115, 116, 101, 114, 34, 32, 13, 10, 125,
                ];
                stream.write_all(&mensaje).expect("Error sending data");

                let mut buffer = [0; 1024];
                let _ = stream
                    .read(&mut buffer)
                    .expect("Error reading server response");
                let respuesta = String::from_utf8_lossy(&buffer);
                assert!(respuesta.contains("HTTP/1.1 422 Can't find the branch"));
            }
            Err(e) => eprintln!("Error al conectar al servidor: {}", e),
        }
        fs::remove_dir_all("server")?;
        Ok(())
    }

    #[test]
    pub fn test_04_test_error_pr_already_exist_creating_pr() -> Result<(), std::io::Error> {
        let server_path = Path::new("server/amoralejo/algo1");

        let _ = thread::spawn(|| {
            let _ = WebServer::new_listen("server".to_string().into());
        });

        VersionControlSystem::init(server_path, Vec::new());
        Config::write_config(("Agus".to_owned(), "amoralejo@fi.uba.ar".to_owned()))?;
        commit_one_file_client(server_path.to_path_buf(), "test_file_1.txt");
        let _ = VersionControlSystem::checkout(
            rust_git::vcs::commands::checkout::CheckoutOptions::CreateAndChangeBranch("new_branch"),
        );
        commit_one_file_client(server_path.to_path_buf(), "test_file_2.txt");

        let addres = "127.0.0.1:3000";

        match TcpStream::connect(addres) {
            Ok(mut stream) => {
                let mensaje = [
                    80, 79, 83, 84, 32, 47, 114, 101, 112, 111, 115, 47, 97, 109, 111, 114, 97,
                    108, 101, 106, 111, 47, 97, 108, 103, 111, 49, 47, 112, 117, 108, 108, 115, 32,
                    72, 84, 84, 80, 47, 49, 46, 49, 13, 10, 67, 111, 110, 116, 101, 110, 116, 45,
                    84, 121, 112, 101, 58, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111, 110,
                    47, 106, 115, 111, 110, 13, 10, 85, 115, 101, 114, 45, 65, 103, 101, 110, 116,
                    58, 32, 80, 111, 115, 116, 109, 97, 110, 82, 117, 110, 116, 105, 109, 101, 47,
                    55, 46, 51, 54, 46, 48, 13, 10, 65, 99, 99, 101, 112, 116, 58, 32, 42, 47, 42,
                    13, 10, 80, 111, 115, 116, 109, 97, 110, 45, 84, 111, 107, 101, 110, 58, 32,
                    97, 55, 56, 54, 56, 56, 51, 102, 45, 101, 50, 102, 52, 45, 52, 55, 52, 48, 45,
                    56, 51, 97, 51, 45, 51, 54, 101, 53, 54, 97, 102, 100, 51, 52, 54, 54, 13, 10,
                    72, 111, 115, 116, 58, 32, 108, 111, 99, 97, 108, 104, 111, 115, 116, 58, 51,
                    48, 48, 48, 13, 10, 65, 99, 99, 101, 112, 116, 45, 69, 110, 99, 111, 100, 105,
                    110, 103, 58, 32, 103, 122, 105, 112, 44, 32, 100, 101, 102, 108, 97, 116, 101,
                    44, 32, 98, 114, 13, 10, 67, 111, 110, 110, 101, 99, 116, 105, 111, 110, 58,
                    32, 107, 101, 101, 112, 45, 97, 108, 105, 118, 101, 13, 10, 67, 111, 110, 116,
                    101, 110, 116, 45, 76, 101, 110, 103, 116, 104, 58, 32, 49, 49, 55, 13, 10, 13,
                    10, 123, 13, 10, 32, 32, 32, 32, 34, 116, 105, 116, 108, 101, 34, 58, 32, 34,
                    97, 108, 103, 111, 49, 34, 44, 13, 10, 32, 32, 32, 32, 34, 98, 111, 100, 121,
                    34, 58, 32, 34, 77, 97, 116, 101, 114, 105, 97, 32, 97, 108, 103, 111, 49, 34,
                    44, 13, 10, 32, 32, 32, 32, 34, 104, 101, 97, 100, 34, 58, 32, 34, 97, 109,
                    111, 114, 97, 108, 101, 106, 111, 58, 110, 101, 119, 95, 98, 114, 97, 110, 99,
                    104, 34, 44, 13, 10, 32, 32, 32, 32, 34, 98, 97, 115, 101, 34, 58, 32, 34, 109,
                    97, 115, 116, 101, 114, 34, 32, 13, 10, 125,
                ];
                stream.write_all(&mensaje).expect("Error sending data");

                let mut buffer = [0; 1024];
                let _ = stream
                    .read(&mut buffer)
                    .expect("Error reading server response");
                let respuesta = String::from_utf8_lossy(&buffer);
                assert!(respuesta.contains("HTTP/1.1 200 OK"));
            }
            Err(e) => eprintln!("Error al conectar al servidor: {}", e),
        }
        match TcpStream::connect(addres) {
            Ok(mut stream) => {
                let mensaje = [
                    80, 79, 83, 84, 32, 47, 114, 101, 112, 111, 115, 47, 97, 109, 111, 114, 97,
                    108, 101, 106, 111, 47, 97, 108, 103, 111, 49, 47, 112, 117, 108, 108, 115, 32,
                    72, 84, 84, 80, 47, 49, 46, 49, 13, 10, 67, 111, 110, 116, 101, 110, 116, 45,
                    84, 121, 112, 101, 58, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111, 110,
                    47, 106, 115, 111, 110, 13, 10, 85, 115, 101, 114, 45, 65, 103, 101, 110, 116,
                    58, 32, 80, 111, 115, 116, 109, 97, 110, 82, 117, 110, 116, 105, 109, 101, 47,
                    55, 46, 51, 54, 46, 48, 13, 10, 65, 99, 99, 101, 112, 116, 58, 32, 42, 47, 42,
                    13, 10, 80, 111, 115, 116, 109, 97, 110, 45, 84, 111, 107, 101, 110, 58, 32,
                    97, 55, 56, 54, 56, 56, 51, 102, 45, 101, 50, 102, 52, 45, 52, 55, 52, 48, 45,
                    56, 51, 97, 51, 45, 51, 54, 101, 53, 54, 97, 102, 100, 51, 52, 54, 54, 13, 10,
                    72, 111, 115, 116, 58, 32, 108, 111, 99, 97, 108, 104, 111, 115, 116, 58, 51,
                    48, 48, 48, 13, 10, 65, 99, 99, 101, 112, 116, 45, 69, 110, 99, 111, 100, 105,
                    110, 103, 58, 32, 103, 122, 105, 112, 44, 32, 100, 101, 102, 108, 97, 116, 101,
                    44, 32, 98, 114, 13, 10, 67, 111, 110, 110, 101, 99, 116, 105, 111, 110, 58,
                    32, 107, 101, 101, 112, 45, 97, 108, 105, 118, 101, 13, 10, 67, 111, 110, 116,
                    101, 110, 116, 45, 76, 101, 110, 103, 116, 104, 58, 32, 49, 49, 55, 13, 10, 13,
                    10, 123, 13, 10, 32, 32, 32, 32, 34, 116, 105, 116, 108, 101, 34, 58, 32, 34,
                    97, 108, 103, 111, 49, 34, 44, 13, 10, 32, 32, 32, 32, 34, 98, 111, 100, 121,
                    34, 58, 32, 34, 77, 97, 116, 101, 114, 105, 97, 32, 97, 108, 103, 111, 49, 34,
                    44, 13, 10, 32, 32, 32, 32, 34, 104, 101, 97, 100, 34, 58, 32, 34, 97, 109,
                    111, 114, 97, 108, 101, 106, 111, 58, 110, 101, 119, 95, 98, 114, 97, 110, 99,
                    104, 34, 44, 13, 10, 32, 32, 32, 32, 34, 98, 97, 115, 101, 34, 58, 32, 34, 109,
                    97, 115, 116, 101, 114, 34, 32, 13, 10, 125,
                ];
                stream.write_all(&mensaje).expect("Error sending data");

                let mut buffer = [0; 1024];
                let _ = stream
                    .read(&mut buffer)
                    .expect("Error reading server response");
                let respuesta = String::from_utf8_lossy(&buffer);
                assert!(
                    respuesta.contains("HTTP/1.1 403 The requested pr has already been created")
                );
            }
            Err(e) => eprintln!("Error al conectar al servidor: {}", e),
        }
        fs::remove_dir_all("server")?;
        Ok(())
    }

    #[test]
    pub fn test_05_error_creating_pr_with_same_head_in_both_branchs() -> Result<(), std::io::Error>
    {
        let server_path = Path::new("server/amoralejo/algo1");

        let _ = thread::spawn(|| {
            let _ = WebServer::new_listen("server".to_string().into());
        });

        VersionControlSystem::init(server_path, Vec::new());
        Config::write_config(("Agus".to_owned(), "amoralejo@fi.uba.ar".to_owned()))?;
        commit_one_file_client(server_path.to_path_buf(), "test_file_1.txt");
        let _ = VersionControlSystem::checkout(
            rust_git::vcs::commands::checkout::CheckoutOptions::CreateAndChangeBranch("new_branch"),
        );

        let addres = "127.0.0.1:3000";

        match TcpStream::connect(addres) {
            Ok(mut stream) => {
                let mensaje = [
                    80, 79, 83, 84, 32, 47, 114, 101, 112, 111, 115, 47, 97, 109, 111, 114, 97,
                    108, 101, 106, 111, 47, 97, 108, 103, 111, 49, 47, 112, 117, 108, 108, 115, 32,
                    72, 84, 84, 80, 47, 49, 46, 49, 13, 10, 67, 111, 110, 116, 101, 110, 116, 45,
                    84, 121, 112, 101, 58, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111, 110,
                    47, 106, 115, 111, 110, 13, 10, 85, 115, 101, 114, 45, 65, 103, 101, 110, 116,
                    58, 32, 80, 111, 115, 116, 109, 97, 110, 82, 117, 110, 116, 105, 109, 101, 47,
                    55, 46, 51, 54, 46, 48, 13, 10, 65, 99, 99, 101, 112, 116, 58, 32, 42, 47, 42,
                    13, 10, 80, 111, 115, 116, 109, 97, 110, 45, 84, 111, 107, 101, 110, 58, 32,
                    97, 55, 56, 54, 56, 56, 51, 102, 45, 101, 50, 102, 52, 45, 52, 55, 52, 48, 45,
                    56, 51, 97, 51, 45, 51, 54, 101, 53, 54, 97, 102, 100, 51, 52, 54, 54, 13, 10,
                    72, 111, 115, 116, 58, 32, 108, 111, 99, 97, 108, 104, 111, 115, 116, 58, 51,
                    48, 48, 48, 13, 10, 65, 99, 99, 101, 112, 116, 45, 69, 110, 99, 111, 100, 105,
                    110, 103, 58, 32, 103, 122, 105, 112, 44, 32, 100, 101, 102, 108, 97, 116, 101,
                    44, 32, 98, 114, 13, 10, 67, 111, 110, 110, 101, 99, 116, 105, 111, 110, 58,
                    32, 107, 101, 101, 112, 45, 97, 108, 105, 118, 101, 13, 10, 67, 111, 110, 116,
                    101, 110, 116, 45, 76, 101, 110, 103, 116, 104, 58, 32, 49, 49, 55, 13, 10, 13,
                    10, 123, 13, 10, 32, 32, 32, 32, 34, 116, 105, 116, 108, 101, 34, 58, 32, 34,
                    97, 108, 103, 111, 49, 34, 44, 13, 10, 32, 32, 32, 32, 34, 98, 111, 100, 121,
                    34, 58, 32, 34, 77, 97, 116, 101, 114, 105, 97, 32, 97, 108, 103, 111, 49, 34,
                    44, 13, 10, 32, 32, 32, 32, 34, 104, 101, 97, 100, 34, 58, 32, 34, 97, 109,
                    111, 114, 97, 108, 101, 106, 111, 58, 110, 101, 119, 95, 98, 114, 97, 110, 99,
                    104, 34, 44, 13, 10, 32, 32, 32, 32, 34, 98, 97, 115, 101, 34, 58, 32, 34, 109,
                    97, 115, 116, 101, 114, 34, 32, 13, 10, 125,
                ];
                stream.write_all(&mensaje).expect("Error sending data");

                let mut buffer = [0; 1024];
                let _ = stream
                    .read(&mut buffer)
                    .expect("Error reading server response");
                let respuesta = String::from_utf8_lossy(&buffer);
                println!("RESPUESTA: {}", respuesta);
                assert!(respuesta.contains("HTTP/1.1 422 Head and Base only shared equal commits"));
            }
            Err(e) => eprintln!("Error al conectar al servidor: {}", e),
        }
        fs::remove_dir_all("server")?;
        Ok(())
    }

    #[test]
    pub fn test_06_test_list_one_pr() -> Result<(), std::io::Error> {
        let server_path = Path::new("server/amoralejo/algo1");

        let _ = thread::spawn(|| {
            let _ = WebServer::new_listen("server".to_string().into());
        });

        VersionControlSystem::init(server_path, Vec::new());
        Config::write_config(("Agus".to_owned(), "amoralejo@fi.uba.ar".to_owned()))?;
        commit_one_file_client(server_path.to_path_buf(), "test_file_1.txt");
        let _ = VersionControlSystem::checkout(
            rust_git::vcs::commands::checkout::CheckoutOptions::CreateAndChangeBranch("new_branch"),
        );
        commit_one_file_client(server_path.to_path_buf(), "test_file_2.txt");

        let addres = "127.0.0.1:3000";

        match TcpStream::connect(addres) {
            Ok(mut stream) => {
                let mensaje = [
                    80, 79, 83, 84, 32, 47, 114, 101, 112, 111, 115, 47, 97, 109, 111, 114, 97,
                    108, 101, 106, 111, 47, 97, 108, 103, 111, 49, 47, 112, 117, 108, 108, 115, 32,
                    72, 84, 84, 80, 47, 49, 46, 49, 13, 10, 67, 111, 110, 116, 101, 110, 116, 45,
                    84, 121, 112, 101, 58, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111, 110,
                    47, 106, 115, 111, 110, 13, 10, 85, 115, 101, 114, 45, 65, 103, 101, 110, 116,
                    58, 32, 80, 111, 115, 116, 109, 97, 110, 82, 117, 110, 116, 105, 109, 101, 47,
                    55, 46, 51, 54, 46, 48, 13, 10, 65, 99, 99, 101, 112, 116, 58, 32, 42, 47, 42,
                    13, 10, 80, 111, 115, 116, 109, 97, 110, 45, 84, 111, 107, 101, 110, 58, 32,
                    97, 55, 56, 54, 56, 56, 51, 102, 45, 101, 50, 102, 52, 45, 52, 55, 52, 48, 45,
                    56, 51, 97, 51, 45, 51, 54, 101, 53, 54, 97, 102, 100, 51, 52, 54, 54, 13, 10,
                    72, 111, 115, 116, 58, 32, 108, 111, 99, 97, 108, 104, 111, 115, 116, 58, 51,
                    48, 48, 48, 13, 10, 65, 99, 99, 101, 112, 116, 45, 69, 110, 99, 111, 100, 105,
                    110, 103, 58, 32, 103, 122, 105, 112, 44, 32, 100, 101, 102, 108, 97, 116, 101,
                    44, 32, 98, 114, 13, 10, 67, 111, 110, 110, 101, 99, 116, 105, 111, 110, 58,
                    32, 107, 101, 101, 112, 45, 97, 108, 105, 118, 101, 13, 10, 67, 111, 110, 116,
                    101, 110, 116, 45, 76, 101, 110, 103, 116, 104, 58, 32, 49, 49, 55, 13, 10, 13,
                    10, 123, 13, 10, 32, 32, 32, 32, 34, 116, 105, 116, 108, 101, 34, 58, 32, 34,
                    97, 108, 103, 111, 49, 34, 44, 13, 10, 32, 32, 32, 32, 34, 98, 111, 100, 121,
                    34, 58, 32, 34, 77, 97, 116, 101, 114, 105, 97, 32, 97, 108, 103, 111, 49, 34,
                    44, 13, 10, 32, 32, 32, 32, 34, 104, 101, 97, 100, 34, 58, 32, 34, 97, 109,
                    111, 114, 97, 108, 101, 106, 111, 58, 110, 101, 119, 95, 98, 114, 97, 110, 99,
                    104, 34, 44, 13, 10, 32, 32, 32, 32, 34, 98, 97, 115, 101, 34, 58, 32, 34, 109,
                    97, 115, 116, 101, 114, 34, 32, 13, 10, 125,
                ];
                stream.write_all(&mensaje).expect("Error sending data");

                let mut buffer = [0; 1024];
                let _ = stream
                    .read(&mut buffer)
                    .expect("Error reading server response");
                let respuesta = String::from_utf8_lossy(&buffer);
                assert!(respuesta.contains("HTTP/1.1 200 OK"));
            }
            Err(e) => eprintln!("Error al conectar al servidor: {}", e),
        }
        match TcpStream::connect(addres) {
            Ok(mut stream) => {
                let mensaje = [
                    71, 69, 84, 32, 47, 114, 101, 112, 111, 115, 47, 97, 109, 111, 114, 97, 108,
                    101, 106, 111, 47, 97, 108, 103, 111, 49, 47, 112, 117, 108, 108, 115, 32, 72,
                    84, 84, 80, 47, 49, 46, 49, 13, 10, 85, 115, 101, 114, 45, 65, 103, 101, 110,
                    116, 58, 32, 80, 111, 115, 116, 109, 97, 110, 82, 117, 110, 116, 105, 109, 101,
                    47, 55, 46, 51, 54, 46, 48, 13, 10, 65, 99, 99, 101, 112, 116, 58, 32, 42, 47,
                    42, 13, 10, 80, 111, 115, 116, 109, 97, 110, 45, 84, 111, 107, 101, 110, 58,
                    32, 57, 54, 101, 100, 97, 52, 53, 51, 45, 52, 54, 53, 53, 45, 52, 48, 101, 97,
                    45, 97, 53, 53, 54, 45, 100, 57, 100, 98, 102, 99, 56, 51, 100, 99, 55, 56, 13,
                    10, 72, 111, 115, 116, 58, 32, 108, 111, 99, 97, 108, 104, 111, 115, 116, 58,
                    51, 48, 48, 48, 13, 10, 65, 99, 99, 101, 112, 116, 45, 69, 110, 99, 111, 100,
                    105, 110, 103, 58, 32, 103, 122, 105, 112, 44, 32, 100, 101, 102, 108, 97, 116,
                    101, 44, 32, 98, 114, 13, 10, 67, 111, 110, 110, 101, 99, 116, 105, 111, 110,
                    58, 32, 107, 101, 101, 112, 45, 97, 108, 105, 118, 101, 13, 10, 13, 10,
                ];
                stream.write_all(&mensaje).expect("Error sending data");

                let mut buffer = [0; 1024];
                let _ = stream
                    .read(&mut buffer)
                    .expect("Error reading server response");
                let respuesta = String::from_utf8_lossy(&buffer);
                assert_eq!(words_counter(&respuesta, "init_commit"), 1);
            }
            Err(e) => eprintln!("Error al conectar al servidor: {}", e),
        }
        fs::remove_dir_all("server")?;
        Ok(())
    }

    #[test]
    pub fn test_07_test_list_two_pr() -> Result<(), std::io::Error> {
        let server_path = Path::new("server/amoralejo/algo1");

        let _ = thread::spawn(|| {
            let _ = WebServer::new_listen("server".to_string().into());
        });

        VersionControlSystem::init(server_path, Vec::new());
        Config::write_config(("Agus".to_owned(), "amoralejo@fi.uba.ar".to_owned()))?;
        commit_one_file_client(server_path.to_path_buf(), "test_file_1.txt");
        let _ = VersionControlSystem::checkout(
            rust_git::vcs::commands::checkout::CheckoutOptions::CreateAndChangeBranch("new_branch"),
        );
        commit_one_file_client(server_path.to_path_buf(), "test_file_2.txt");

        let addres = "127.0.0.1:3000";

        match TcpStream::connect(addres) {
            Ok(mut stream) => {
                let mensaje = [
                    80, 79, 83, 84, 32, 47, 114, 101, 112, 111, 115, 47, 97, 109, 111, 114, 97,
                    108, 101, 106, 111, 47, 97, 108, 103, 111, 49, 47, 112, 117, 108, 108, 115, 32,
                    72, 84, 84, 80, 47, 49, 46, 49, 13, 10, 67, 111, 110, 116, 101, 110, 116, 45,
                    84, 121, 112, 101, 58, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111, 110,
                    47, 106, 115, 111, 110, 13, 10, 85, 115, 101, 114, 45, 65, 103, 101, 110, 116,
                    58, 32, 80, 111, 115, 116, 109, 97, 110, 82, 117, 110, 116, 105, 109, 101, 47,
                    55, 46, 51, 54, 46, 48, 13, 10, 65, 99, 99, 101, 112, 116, 58, 32, 42, 47, 42,
                    13, 10, 80, 111, 115, 116, 109, 97, 110, 45, 84, 111, 107, 101, 110, 58, 32,
                    97, 55, 56, 54, 56, 56, 51, 102, 45, 101, 50, 102, 52, 45, 52, 55, 52, 48, 45,
                    56, 51, 97, 51, 45, 51, 54, 101, 53, 54, 97, 102, 100, 51, 52, 54, 54, 13, 10,
                    72, 111, 115, 116, 58, 32, 108, 111, 99, 97, 108, 104, 111, 115, 116, 58, 51,
                    48, 48, 48, 13, 10, 65, 99, 99, 101, 112, 116, 45, 69, 110, 99, 111, 100, 105,
                    110, 103, 58, 32, 103, 122, 105, 112, 44, 32, 100, 101, 102, 108, 97, 116, 101,
                    44, 32, 98, 114, 13, 10, 67, 111, 110, 110, 101, 99, 116, 105, 111, 110, 58,
                    32, 107, 101, 101, 112, 45, 97, 108, 105, 118, 101, 13, 10, 67, 111, 110, 116,
                    101, 110, 116, 45, 76, 101, 110, 103, 116, 104, 58, 32, 49, 49, 55, 13, 10, 13,
                    10, 123, 13, 10, 32, 32, 32, 32, 34, 116, 105, 116, 108, 101, 34, 58, 32, 34,
                    97, 108, 103, 111, 49, 34, 44, 13, 10, 32, 32, 32, 32, 34, 98, 111, 100, 121,
                    34, 58, 32, 34, 77, 97, 116, 101, 114, 105, 97, 32, 97, 108, 103, 111, 49, 34,
                    44, 13, 10, 32, 32, 32, 32, 34, 104, 101, 97, 100, 34, 58, 32, 34, 97, 109,
                    111, 114, 97, 108, 101, 106, 111, 58, 110, 101, 119, 95, 98, 114, 97, 110, 99,
                    104, 34, 44, 13, 10, 32, 32, 32, 32, 34, 98, 97, 115, 101, 34, 58, 32, 34, 109,
                    97, 115, 116, 101, 114, 34, 32, 13, 10, 125,
                ];
                stream.write_all(&mensaje).expect("Error sending data");
                let mut buffer = [0; 1024];
                let _ = stream
                    .read(&mut buffer)
                    .expect("Error reading server response");
                let respuesta = String::from_utf8_lossy(&buffer);
                assert!(respuesta.contains("HTTP/1.1 200 OK"));
            }
            Err(e) => eprintln!("Error al conectar al servidor: {}", e),
        }
        match TcpStream::connect(addres) {
            Ok(mut stream) => {
                let mensaje = [
                    71, 69, 84, 32, 47, 114, 101, 112, 111, 115, 47, 97, 109, 111, 114, 97, 108,
                    101, 106, 111, 47, 97, 108, 103, 111, 49, 47, 112, 117, 108, 108, 115, 32, 72,
                    84, 84, 80, 47, 49, 46, 49, 13, 10, 85, 115, 101, 114, 45, 65, 103, 101, 110,
                    116, 58, 32, 80, 111, 115, 116, 109, 97, 110, 82, 117, 110, 116, 105, 109, 101,
                    47, 55, 46, 51, 54, 46, 48, 13, 10, 65, 99, 99, 101, 112, 116, 58, 32, 42, 47,
                    42, 13, 10, 80, 111, 115, 116, 109, 97, 110, 45, 84, 111, 107, 101, 110, 58,
                    32, 57, 54, 101, 100, 97, 52, 53, 51, 45, 52, 54, 53, 53, 45, 52, 48, 101, 97,
                    45, 97, 53, 53, 54, 45, 100, 57, 100, 98, 102, 99, 56, 51, 100, 99, 55, 56, 13,
                    10, 72, 111, 115, 116, 58, 32, 108, 111, 99, 97, 108, 104, 111, 115, 116, 58,
                    51, 48, 48, 48, 13, 10, 65, 99, 99, 101, 112, 116, 45, 69, 110, 99, 111, 100,
                    105, 110, 103, 58, 32, 103, 122, 105, 112, 44, 32, 100, 101, 102, 108, 97, 116,
                    101, 44, 32, 98, 114, 13, 10, 67, 111, 110, 110, 101, 99, 116, 105, 111, 110,
                    58, 32, 107, 101, 101, 112, 45, 97, 108, 105, 118, 101, 13, 10, 13, 10,
                ];
                stream.write_all(&mensaje).expect("Error sending data");
                let mut buffer = [0; 1024];
                let _ = stream
                    .read(&mut buffer)
                    .expect("Error reading server response");
                let respuesta = String::from_utf8_lossy(&buffer);
                assert_eq!(words_counter(&respuesta, "init_commit"), 1);
            }
            Err(e) => eprintln!("Error al conectar al servidor: {}", e),
        }

        let _ = VersionControlSystem::checkout(
            rust_git::vcs::commands::checkout::CheckoutOptions::CreateAndChangeBranch(
                "new_branch2",
            ),
        );
        commit_one_file_client(server_path.to_path_buf(), "test_file_3.txt");

        match TcpStream::connect(addres) {
            Ok(mut stream) => {
                let mensaje = [
                    80, 79, 83, 84, 32, 47, 114, 101, 112, 111, 115, 47, 97, 109, 111, 114, 97,
                    108, 101, 106, 111, 47, 97, 108, 103, 111, 49, 47, 112, 117, 108, 108, 115, 32,
                    72, 84, 84, 80, 47, 49, 46, 49, 13, 10, 67, 111, 110, 116, 101, 110, 116, 45,
                    84, 121, 112, 101, 58, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111, 110,
                    47, 106, 115, 111, 110, 13, 10, 85, 115, 101, 114, 45, 65, 103, 101, 110, 116,
                    58, 32, 80, 111, 115, 116, 109, 97, 110, 82, 117, 110, 116, 105, 109, 101, 47,
                    55, 46, 51, 54, 46, 48, 13, 10, 65, 99, 99, 101, 112, 116, 58, 32, 42, 47, 42,
                    13, 10, 80, 111, 115, 116, 109, 97, 110, 45, 84, 111, 107, 101, 110, 58, 32,
                    102, 97, 100, 53, 53, 55, 101, 55, 45, 56, 55, 102, 49, 45, 52, 48, 57, 97, 45,
                    56, 56, 98, 55, 45, 56, 57, 97, 54, 102, 51, 101, 101, 97, 102, 49, 99, 13, 10,
                    72, 111, 115, 116, 58, 32, 108, 111, 99, 97, 108, 104, 111, 115, 116, 58, 51,
                    48, 48, 48, 13, 10, 65, 99, 99, 101, 112, 116, 45, 69, 110, 99, 111, 100, 105,
                    110, 103, 58, 32, 103, 122, 105, 112, 44, 32, 100, 101, 102, 108, 97, 116, 101,
                    44, 32, 98, 114, 13, 10, 67, 111, 110, 110, 101, 99, 116, 105, 111, 110, 58,
                    32, 107, 101, 101, 112, 45, 97, 108, 105, 118, 101, 13, 10, 67, 111, 110, 116,
                    101, 110, 116, 45, 76, 101, 110, 103, 116, 104, 58, 32, 49, 49, 56, 13, 10, 13,
                    10, 123, 13, 10, 32, 32, 32, 32, 34, 116, 105, 116, 108, 101, 34, 58, 32, 34,
                    97, 108, 103, 111, 49, 34, 44, 13, 10, 32, 32, 32, 32, 34, 98, 111, 100, 121,
                    34, 58, 32, 34, 77, 97, 116, 101, 114, 105, 97, 32, 97, 108, 103, 111, 49, 34,
                    44, 13, 10, 32, 32, 32, 32, 34, 104, 101, 97, 100, 34, 58, 32, 34, 97, 109,
                    111, 114, 97, 108, 101, 106, 111, 58, 110, 101, 119, 95, 98, 114, 97, 110, 99,
                    104, 50, 34, 44, 13, 10, 32, 32, 32, 32, 34, 98, 97, 115, 101, 34, 58, 32, 34,
                    109, 97, 115, 116, 101, 114, 34, 32, 13, 10, 125,
                ];
                stream.write_all(&mensaje).expect("Error sending data");
                let mut buffer = [0; 1024];
                let _ = stream
                    .read(&mut buffer)
                    .expect("Error reading server response");
                let respuesta = String::from_utf8_lossy(&buffer);
                assert!(respuesta.contains("HTTP/1.1 200 OK"));
            }
            Err(e) => eprintln!("Error al conectar al servidor: {}", e),
        }
        match TcpStream::connect(addres) {
            Ok(mut stream) => {
                let mensaje = [
                    71, 69, 84, 32, 47, 114, 101, 112, 111, 115, 47, 97, 109, 111, 114, 97, 108,
                    101, 106, 111, 47, 97, 108, 103, 111, 49, 47, 112, 117, 108, 108, 115, 32, 72,
                    84, 84, 80, 47, 49, 46, 49, 13, 10, 85, 115, 101, 114, 45, 65, 103, 101, 110,
                    116, 58, 32, 80, 111, 115, 116, 109, 97, 110, 82, 117, 110, 116, 105, 109, 101,
                    47, 55, 46, 51, 54, 46, 48, 13, 10, 65, 99, 99, 101, 112, 116, 58, 32, 42, 47,
                    42, 13, 10, 80, 111, 115, 116, 109, 97, 110, 45, 84, 111, 107, 101, 110, 58,
                    32, 57, 54, 101, 100, 97, 52, 53, 51, 45, 52, 54, 53, 53, 45, 52, 48, 101, 97,
                    45, 97, 53, 53, 54, 45, 100, 57, 100, 98, 102, 99, 56, 51, 100, 99, 55, 56, 13,
                    10, 72, 111, 115, 116, 58, 32, 108, 111, 99, 97, 108, 104, 111, 115, 116, 58,
                    51, 48, 48, 48, 13, 10, 65, 99, 99, 101, 112, 116, 45, 69, 110, 99, 111, 100,
                    105, 110, 103, 58, 32, 103, 122, 105, 112, 44, 32, 100, 101, 102, 108, 97, 116,
                    101, 44, 32, 98, 114, 13, 10, 67, 111, 110, 110, 101, 99, 116, 105, 111, 110,
                    58, 32, 107, 101, 101, 112, 45, 97, 108, 105, 118, 101, 13, 10, 13, 10,
                ];
                stream.write_all(&mensaje).expect("Error sending data");
                let mut buffer = [0; 1024];
                let _ = stream
                    .read(&mut buffer)
                    .expect("Error reading server response");
                let respuesta = String::from_utf8_lossy(&buffer);
                assert_eq!(words_counter(&respuesta, "init_commit"), 2);
            }
            Err(e) => eprintln!("Error al conectar al servidor: {}", e),
        }

        fs::remove_dir_all("server")?;
        Ok(())
    }

    #[test]
    pub fn test_08_test_get_specific_pr() -> Result<(), std::io::Error> {
        let server_path = Path::new("server/amoralejo/algo1");

        let _ = thread::spawn(|| {
            let _ = WebServer::new_listen("server".to_string().into());
        });

        VersionControlSystem::init(server_path, Vec::new());
        Config::write_config(("Agus".to_owned(), "amoralejo@fi.uba.ar".to_owned()))?;
        commit_one_file_client(server_path.to_path_buf(), "test_file_1.txt");
        let _ = VersionControlSystem::checkout(
            rust_git::vcs::commands::checkout::CheckoutOptions::CreateAndChangeBranch("new_branch"),
        );
        commit_one_file_client(server_path.to_path_buf(), "test_file_2.txt");

        let addres = "127.0.0.1:3000";

        match TcpStream::connect(addres) {
            Ok(mut stream) => {
                let mensaje = [
                    80, 79, 83, 84, 32, 47, 114, 101, 112, 111, 115, 47, 97, 109, 111, 114, 97,
                    108, 101, 106, 111, 47, 97, 108, 103, 111, 49, 47, 112, 117, 108, 108, 115, 32,
                    72, 84, 84, 80, 47, 49, 46, 49, 13, 10, 67, 111, 110, 116, 101, 110, 116, 45,
                    84, 121, 112, 101, 58, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111, 110,
                    47, 106, 115, 111, 110, 13, 10, 85, 115, 101, 114, 45, 65, 103, 101, 110, 116,
                    58, 32, 80, 111, 115, 116, 109, 97, 110, 82, 117, 110, 116, 105, 109, 101, 47,
                    55, 46, 51, 54, 46, 48, 13, 10, 65, 99, 99, 101, 112, 116, 58, 32, 42, 47, 42,
                    13, 10, 80, 111, 115, 116, 109, 97, 110, 45, 84, 111, 107, 101, 110, 58, 32,
                    97, 55, 56, 54, 56, 56, 51, 102, 45, 101, 50, 102, 52, 45, 52, 55, 52, 48, 45,
                    56, 51, 97, 51, 45, 51, 54, 101, 53, 54, 97, 102, 100, 51, 52, 54, 54, 13, 10,
                    72, 111, 115, 116, 58, 32, 108, 111, 99, 97, 108, 104, 111, 115, 116, 58, 51,
                    48, 48, 48, 13, 10, 65, 99, 99, 101, 112, 116, 45, 69, 110, 99, 111, 100, 105,
                    110, 103, 58, 32, 103, 122, 105, 112, 44, 32, 100, 101, 102, 108, 97, 116, 101,
                    44, 32, 98, 114, 13, 10, 67, 111, 110, 110, 101, 99, 116, 105, 111, 110, 58,
                    32, 107, 101, 101, 112, 45, 97, 108, 105, 118, 101, 13, 10, 67, 111, 110, 116,
                    101, 110, 116, 45, 76, 101, 110, 103, 116, 104, 58, 32, 49, 49, 55, 13, 10, 13,
                    10, 123, 13, 10, 32, 32, 32, 32, 34, 116, 105, 116, 108, 101, 34, 58, 32, 34,
                    97, 108, 103, 111, 49, 34, 44, 13, 10, 32, 32, 32, 32, 34, 98, 111, 100, 121,
                    34, 58, 32, 34, 77, 97, 116, 101, 114, 105, 97, 32, 97, 108, 103, 111, 49, 34,
                    44, 13, 10, 32, 32, 32, 32, 34, 104, 101, 97, 100, 34, 58, 32, 34, 97, 109,
                    111, 114, 97, 108, 101, 106, 111, 58, 110, 101, 119, 95, 98, 114, 97, 110, 99,
                    104, 34, 44, 13, 10, 32, 32, 32, 32, 34, 98, 97, 115, 101, 34, 58, 32, 34, 109,
                    97, 115, 116, 101, 114, 34, 32, 13, 10, 125,
                ];
                stream.write_all(&mensaje).expect("Error sending data");

                let mut buffer = [0; 1024];
                let _ = stream
                    .read(&mut buffer)
                    .expect("Error reading server response");
                let respuesta = String::from_utf8_lossy(&buffer);
                assert!(respuesta.contains("HTTP/1.1 200 OK"));
            }
            Err(e) => eprintln!("Error al conectar al servidor: {}", e),
        }

        let pull_requests = read_directory()?;

        let message = format!("GET /repos/amoralejo/algo1/pulls/{} HTTP/1.1\nUser-Agent: PostmanRuntime/7.36.0\nAccept: */*\nPostman-Token: 8dfb536a-8779-492f-8dee-268944580a5c\nHost: localhost:3000\nAccept-Encoding: gzip, deflate, br\nConnection: keep-alive\r\n\r\n", pull_requests[0]);

        match TcpStream::connect(addres) {
            Ok(mut stream) => {
                stream
                    .write_all(&message.as_bytes())
                    .expect("Error sending data");

                let mut buffer = [0; 1024];
                let _ = stream
                    .read(&mut buffer)
                    .expect("Error reading server response");
                let respuesta = String::from_utf8_lossy(&buffer);
                assert!(respuesta.contains("HTTP/1.1 200 OK"));
            }
            Err(e) => eprintln!("Error al conectar al servidor: {}", e),
        }

        fs::remove_dir_all("server")?;
        Ok(())
    }

    #[test]
    pub fn test_09_test_get_commits_in_specific_pr() -> Result<(), std::io::Error> {
        let server_path = Path::new("server/amoralejo/algo1");

        let _ = thread::spawn(|| {
            let _ = WebServer::new_listen("server".to_string().into());
        });

        VersionControlSystem::init(server_path, Vec::new());
        Config::write_config(("Agus".to_owned(), "amoralejo@fi.uba.ar".to_owned()))?;
        commit_one_file_client(server_path.to_path_buf(), "test_file_1.txt");
        let _ = VersionControlSystem::checkout(
            rust_git::vcs::commands::checkout::CheckoutOptions::CreateAndChangeBranch("new_branch"),
        );
        commit_one_file_client(server_path.to_path_buf(), "test_file_2.txt");

        let addres = "127.0.0.1:3000";

        match TcpStream::connect(addres) {
            Ok(mut stream) => {
                let mensaje = [
                    80, 79, 83, 84, 32, 47, 114, 101, 112, 111, 115, 47, 97, 109, 111, 114, 97,
                    108, 101, 106, 111, 47, 97, 108, 103, 111, 49, 47, 112, 117, 108, 108, 115, 32,
                    72, 84, 84, 80, 47, 49, 46, 49, 13, 10, 67, 111, 110, 116, 101, 110, 116, 45,
                    84, 121, 112, 101, 58, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111, 110,
                    47, 106, 115, 111, 110, 13, 10, 85, 115, 101, 114, 45, 65, 103, 101, 110, 116,
                    58, 32, 80, 111, 115, 116, 109, 97, 110, 82, 117, 110, 116, 105, 109, 101, 47,
                    55, 46, 51, 54, 46, 48, 13, 10, 65, 99, 99, 101, 112, 116, 58, 32, 42, 47, 42,
                    13, 10, 80, 111, 115, 116, 109, 97, 110, 45, 84, 111, 107, 101, 110, 58, 32,
                    97, 55, 56, 54, 56, 56, 51, 102, 45, 101, 50, 102, 52, 45, 52, 55, 52, 48, 45,
                    56, 51, 97, 51, 45, 51, 54, 101, 53, 54, 97, 102, 100, 51, 52, 54, 54, 13, 10,
                    72, 111, 115, 116, 58, 32, 108, 111, 99, 97, 108, 104, 111, 115, 116, 58, 51,
                    48, 48, 48, 13, 10, 65, 99, 99, 101, 112, 116, 45, 69, 110, 99, 111, 100, 105,
                    110, 103, 58, 32, 103, 122, 105, 112, 44, 32, 100, 101, 102, 108, 97, 116, 101,
                    44, 32, 98, 114, 13, 10, 67, 111, 110, 110, 101, 99, 116, 105, 111, 110, 58,
                    32, 107, 101, 101, 112, 45, 97, 108, 105, 118, 101, 13, 10, 67, 111, 110, 116,
                    101, 110, 116, 45, 76, 101, 110, 103, 116, 104, 58, 32, 49, 49, 55, 13, 10, 13,
                    10, 123, 13, 10, 32, 32, 32, 32, 34, 116, 105, 116, 108, 101, 34, 58, 32, 34,
                    97, 108, 103, 111, 49, 34, 44, 13, 10, 32, 32, 32, 32, 34, 98, 111, 100, 121,
                    34, 58, 32, 34, 77, 97, 116, 101, 114, 105, 97, 32, 97, 108, 103, 111, 49, 34,
                    44, 13, 10, 32, 32, 32, 32, 34, 104, 101, 97, 100, 34, 58, 32, 34, 97, 109,
                    111, 114, 97, 108, 101, 106, 111, 58, 110, 101, 119, 95, 98, 114, 97, 110, 99,
                    104, 34, 44, 13, 10, 32, 32, 32, 32, 34, 98, 97, 115, 101, 34, 58, 32, 34, 109,
                    97, 115, 116, 101, 114, 34, 32, 13, 10, 125,
                ];
                stream.write_all(&mensaje).expect("Error sending data");

                let mut buffer = [0; 1024];
                let _ = stream
                    .read(&mut buffer)
                    .expect("Error reading server response");
                let respuesta = String::from_utf8_lossy(&buffer);
                assert!(respuesta.contains("HTTP/1.1 200 OK"));
            }
            Err(e) => eprintln!("Error al conectar al servidor: {}", e),
        }

        let pull_requests = read_directory()?;

        let message = format!("GET /repos/amoralejo/algo1/pulls/{}/commits HTTP/1.1\nUser-Agent: PostmanRuntime/7.36.0\nAccept: */*\nPostman-Token: 8dfb536a-8779-492f-8dee-268944580a5c\nHost: localhost:3000\nAccept-Encoding: gzip, deflate, br\nConnection: keep-alive\r\n\r\n", pull_requests[0]);

        match TcpStream::connect(addres) {
            Ok(mut stream) => {
                stream
                    .write_all(&message.as_bytes())
                    .expect("Error sending data");

                let mut buffer = [0; 1024];
                let _ = stream
                    .read(&mut buffer)
                    .expect("Error reading server response");
                let respuesta = String::from_utf8_lossy(&buffer);
                assert!(respuesta.contains("HTTP/1.1 200 OK"));
            }
            Err(e) => eprintln!("Error al conectar al servidor: {}", e),
        }

        fs::remove_dir_all("server")?;
        Ok(())
    }

    #[test]
    pub fn test_10_test_patch() -> Result<(), std::io::Error> {
        let server_path = Path::new("server/amoralejo/algo1");

        let _ = thread::spawn(|| {
            let _ = WebServer::new_listen("server".to_string().into());
        });

        VersionControlSystem::init(server_path, Vec::new());
        Config::write_config(("Agus".to_owned(), "amoralejo@fi.uba.ar".to_owned()))?;
        commit_one_file_client(server_path.to_path_buf(), "test_file_1.txt");
        let _ = VersionControlSystem::checkout(
            rust_git::vcs::commands::checkout::CheckoutOptions::CreateAndChangeBranch("new_branch"),
        );
        commit_one_file_client(server_path.to_path_buf(), "test_file_2.txt");

        let addres = "127.0.0.1:3000";

        match TcpStream::connect(addres) {
            Ok(mut stream) => {
                let mensaje = [
                    80, 79, 83, 84, 32, 47, 114, 101, 112, 111, 115, 47, 97, 109, 111, 114, 97,
                    108, 101, 106, 111, 47, 97, 108, 103, 111, 49, 47, 112, 117, 108, 108, 115, 32,
                    72, 84, 84, 80, 47, 49, 46, 49, 13, 10, 67, 111, 110, 116, 101, 110, 116, 45,
                    84, 121, 112, 101, 58, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111, 110,
                    47, 106, 115, 111, 110, 13, 10, 85, 115, 101, 114, 45, 65, 103, 101, 110, 116,
                    58, 32, 80, 111, 115, 116, 109, 97, 110, 82, 117, 110, 116, 105, 109, 101, 47,
                    55, 46, 51, 54, 46, 48, 13, 10, 65, 99, 99, 101, 112, 116, 58, 32, 42, 47, 42,
                    13, 10, 80, 111, 115, 116, 109, 97, 110, 45, 84, 111, 107, 101, 110, 58, 32,
                    97, 55, 56, 54, 56, 56, 51, 102, 45, 101, 50, 102, 52, 45, 52, 55, 52, 48, 45,
                    56, 51, 97, 51, 45, 51, 54, 101, 53, 54, 97, 102, 100, 51, 52, 54, 54, 13, 10,
                    72, 111, 115, 116, 58, 32, 108, 111, 99, 97, 108, 104, 111, 115, 116, 58, 51,
                    48, 48, 48, 13, 10, 65, 99, 99, 101, 112, 116, 45, 69, 110, 99, 111, 100, 105,
                    110, 103, 58, 32, 103, 122, 105, 112, 44, 32, 100, 101, 102, 108, 97, 116, 101,
                    44, 32, 98, 114, 13, 10, 67, 111, 110, 110, 101, 99, 116, 105, 111, 110, 58,
                    32, 107, 101, 101, 112, 45, 97, 108, 105, 118, 101, 13, 10, 67, 111, 110, 116,
                    101, 110, 116, 45, 76, 101, 110, 103, 116, 104, 58, 32, 49, 49, 55, 13, 10, 13,
                    10, 123, 13, 10, 32, 32, 32, 32, 34, 116, 105, 116, 108, 101, 34, 58, 32, 34,
                    97, 108, 103, 111, 49, 34, 44, 13, 10, 32, 32, 32, 32, 34, 98, 111, 100, 121,
                    34, 58, 32, 34, 77, 97, 116, 101, 114, 105, 97, 32, 97, 108, 103, 111, 49, 34,
                    44, 13, 10, 32, 32, 32, 32, 34, 104, 101, 97, 100, 34, 58, 32, 34, 97, 109,
                    111, 114, 97, 108, 101, 106, 111, 58, 110, 101, 119, 95, 98, 114, 97, 110, 99,
                    104, 34, 44, 13, 10, 32, 32, 32, 32, 34, 98, 97, 115, 101, 34, 58, 32, 34, 109,
                    97, 115, 116, 101, 114, 34, 32, 13, 10, 125,
                ];
                stream.write_all(&mensaje).expect("Error sending data");

                let mut buffer = [0; 1024];
                let _ = stream
                    .read(&mut buffer)
                    .expect("Error reading server response");
                let respuesta = String::from_utf8_lossy(&buffer);
                assert!(respuesta.contains("HTTP/1.1 200 OK"));
            }
            Err(e) => eprintln!("Error al conectar al servidor: {}", e),
        }

        let pull_requests = read_directory()?;

        let message = format!("PATCH /repos/amoralejo/algo1/pulls/{} HTTP/1.1\nContent-Type: application/json\nUser-Agent: PostmanRuntime/7.36.0\nAccept: */*\nPostman-Token: 8dfb536a-8779-492f-8dee-268944580a5c\nHost: localhost:3000\nAccept-Encoding: gzip, deflate, br\nConnection: keep-alive\r\n\r\n{}", pull_requests[0], r#"{"title": "new title"}"#);

        match TcpStream::connect(addres) {
            Ok(mut stream) => {
                stream
                    .write_all(&message.as_bytes())
                    .expect("Error sending data");

                let mut buffer = [0; 1024];
                let _ = stream
                    .read(&mut buffer)
                    .expect("Error reading server response");
                let respuesta = String::from_utf8_lossy(&buffer);
                println!("Respuesta: {}", respuesta);
                assert!(respuesta.contains("HTTP/1.1 200 OK"));
            }
            Err(e) => eprintln!("Error al conectar al servidor: {}", e),
        }

        fs::remove_dir_all("server")?;
        Ok(())
    }

    #[test]
    pub fn test_11_test_merge() -> Result<(), std::io::Error> {
        let server_path = Path::new("server/amoralejo/algo1");

        let _ = thread::spawn(|| {
            let _ = WebServer::new_listen("server".to_string().into());
        });

        VersionControlSystem::init(server_path, Vec::new());
        Config::write_config(("Agus".to_owned(), "amoralejo@fi.uba.ar".to_owned()))?;
        commit_one_file_client(server_path.to_path_buf(), "test_file_1.txt");
        let _ = VersionControlSystem::checkout(
            rust_git::vcs::commands::checkout::CheckoutOptions::CreateAndChangeBranch("new_branch"),
        );
        commit_one_file_client(server_path.to_path_buf(), "test_file_2.txt");

        let addres = "127.0.0.1:3000";

        match TcpStream::connect(addres) {
            Ok(mut stream) => {
                let mensaje = [
                    80, 79, 83, 84, 32, 47, 114, 101, 112, 111, 115, 47, 97, 109, 111, 114, 97,
                    108, 101, 106, 111, 47, 97, 108, 103, 111, 49, 47, 112, 117, 108, 108, 115, 32,
                    72, 84, 84, 80, 47, 49, 46, 49, 13, 10, 67, 111, 110, 116, 101, 110, 116, 45,
                    84, 121, 112, 101, 58, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111, 110,
                    47, 106, 115, 111, 110, 13, 10, 85, 115, 101, 114, 45, 65, 103, 101, 110, 116,
                    58, 32, 80, 111, 115, 116, 109, 97, 110, 82, 117, 110, 116, 105, 109, 101, 47,
                    55, 46, 51, 54, 46, 48, 13, 10, 65, 99, 99, 101, 112, 116, 58, 32, 42, 47, 42,
                    13, 10, 80, 111, 115, 116, 109, 97, 110, 45, 84, 111, 107, 101, 110, 58, 32,
                    97, 55, 56, 54, 56, 56, 51, 102, 45, 101, 50, 102, 52, 45, 52, 55, 52, 48, 45,
                    56, 51, 97, 51, 45, 51, 54, 101, 53, 54, 97, 102, 100, 51, 52, 54, 54, 13, 10,
                    72, 111, 115, 116, 58, 32, 108, 111, 99, 97, 108, 104, 111, 115, 116, 58, 51,
                    48, 48, 48, 13, 10, 65, 99, 99, 101, 112, 116, 45, 69, 110, 99, 111, 100, 105,
                    110, 103, 58, 32, 103, 122, 105, 112, 44, 32, 100, 101, 102, 108, 97, 116, 101,
                    44, 32, 98, 114, 13, 10, 67, 111, 110, 110, 101, 99, 116, 105, 111, 110, 58,
                    32, 107, 101, 101, 112, 45, 97, 108, 105, 118, 101, 13, 10, 67, 111, 110, 116,
                    101, 110, 116, 45, 76, 101, 110, 103, 116, 104, 58, 32, 49, 49, 55, 13, 10, 13,
                    10, 123, 13, 10, 32, 32, 32, 32, 34, 116, 105, 116, 108, 101, 34, 58, 32, 34,
                    97, 108, 103, 111, 49, 34, 44, 13, 10, 32, 32, 32, 32, 34, 98, 111, 100, 121,
                    34, 58, 32, 34, 77, 97, 116, 101, 114, 105, 97, 32, 97, 108, 103, 111, 49, 34,
                    44, 13, 10, 32, 32, 32, 32, 34, 104, 101, 97, 100, 34, 58, 32, 34, 97, 109,
                    111, 114, 97, 108, 101, 106, 111, 58, 110, 101, 119, 95, 98, 114, 97, 110, 99,
                    104, 34, 44, 13, 10, 32, 32, 32, 32, 34, 98, 97, 115, 101, 34, 58, 32, 34, 109,
                    97, 115, 116, 101, 114, 34, 32, 13, 10, 125,
                ];
                stream.write_all(&mensaje).expect("Error sending data");

                let mut buffer = [0; 1024];
                let _ = stream
                    .read(&mut buffer)
                    .expect("Error reading server response");
                let respuesta = String::from_utf8_lossy(&buffer);
                assert!(respuesta.contains("HTTP/1.1 200 OK"));
            }
            Err(e) => eprintln!("Error al conectar al servidor: {}", e),
        }

        let pull_requests = read_directory()?;

        let message = format!("PUT /repos/amoralejo/algo1/pulls/{}/merge HTTP/1.1\nUser-Agent: PostmanRuntime/7.36.0\nAccept: */*\nPostman-Token: 8dfb536a-8779-492f-8dee-268944580a5c\nHost: localhost:3000\nAccept-Encoding: gzip, deflate, br\nConnection: keep-alive\r\n\r\n", pull_requests[0]);

        match TcpStream::connect(addres) {
            Ok(mut stream) => {
                stream
                    .write_all(&message.as_bytes())
                    .expect("Error sending data");

                let mut buffer = [0; 1024];
                let _ = stream
                    .read(&mut buffer)
                    .expect("Error reading server response");
                let respuesta = String::from_utf8_lossy(&buffer);
                println!("RESPUESTA: {}", respuesta);
                assert!(respuesta.contains("HTTP/1.1 200 OK"));
            }
            Err(e) => eprintln!("Error al conectar al servidor: {}", e),
        }

        fs::remove_dir_all("server")?;
        Ok(())
    }
}

fn read_directory() -> Result<Vec<String>, std::io::Error> {
    let path = Path::new("server/pull_requests/amoralejo/algo1");
    let mut pull_requests: Vec<String> = Vec::new();
    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let name = entry.file_name();

        if let Some(name_str) = name.to_str() {
            if !name_str.contains("algo1") {
                pull_requests.push(name_str.to_string())
            }
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Error reading directory",
            ));
        }
    }
    Ok(pull_requests)
}

fn words_counter(text: &str, word: &str) -> usize {
    let mut counter: usize = 0;
    for actual_word in text.split_whitespace() {
        if actual_word.contains(word) {
            counter += 1;
        }
    }
    counter
}
