use rust_git::server::server::Server;

fn main() -> Result<(), std::io::Error> {

    let argv = std::env::args().collect::<Vec<String>>();    

   // print!("{:?}", argv);

    match Server::init_server(argv[1].clone()) {
        Ok(_) => println!("La función server se ejecutó correctamente"),
        Err(_) => println!("Hubo un error al ejecutar la función server....."),
    };
    Ok(()) 

}