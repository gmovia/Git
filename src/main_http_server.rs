use rust_git::server_http::web_server::WebServer;

fn main() -> Result<(), std::io::Error> {
    //let argv = std::env::args().collect::<Vec<String>>();
    //let path = &argv[1];

    match WebServer::new() {
        Ok(_) => println!("La función server se ejecutó correctamente."),
        Err(_) => println!("Hubo un error al ejecutar la función server."),
    };

    Ok(())
}
