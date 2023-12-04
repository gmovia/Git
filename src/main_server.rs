use rust_git::servers::server::Server;

fn main() -> Result<(), std::io::Error> {
    let argv = std::env::args().collect::<Vec<String>>();
    let path = &argv[1];

    match Server::server(path.to_string()) {
        Ok(_) => println!("La función server se ejecutó correctamente."),
        Err(_) => println!("Hubo un error al ejecutar la función server."),
    };

    Ok(())
}
