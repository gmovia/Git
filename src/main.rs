use std::{path::Path, io::{self, Write}};

use rust_git::{vcs::{version_control_system::VersionControlSystem, files::config::Config}, handlers::commands::handler_command, interfaces::login::DrawLogin};


use rust_git::interfaces::interface::RustInterface;


fn main() -> Result<(), std::io::Error>{

    //VersionControlSystem::init(Path::new("test_folder/clone"), Vec::new());
    //let _ = VersionControlSystem::init(Path::new("clone"), Vec::new());

    //loop{
    //    let mut input = String::new();
    //    io::stdout().flush().unwrap();
    //    io::stdin().read_line(&mut input).unwrap();
    //    handler_command( &input);
    //}
    let draw_login = DrawLogin::new();
    let result = Config::read_config();
    if result.is_err() {
        let _ = draw_login.impl_login();
    }else {
        let interface = RustInterface::new();
        let _ = interface.impl_interface();
    }
    
    // let interface = RustInterface::new();
    // let _ = interface.impl_interface();
    Ok(())
}





















