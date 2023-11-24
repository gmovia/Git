use std::{path::Path, io::{self, Write}};

use rust_git::{vcs::version_control_system::VersionControlSystem, handlers::commands::handler_command};


use rust_git::interfaces::interface::RustInterface;


fn main() -> Result<(), std::io::Error>{
    // VersionControlSystem::init(Path::new("test_folder/test1"), Vec::new());
    // let _ = VersionControlSystem::init(Path::new("clone"), Vec::new());

    // loop{
    //     let mut input = String::new();
    //     io::stdout().flush().unwrap();
    //     io::stdin().read_line(&mut input).unwrap();
    //     handler_command( &input);
    // }
    let interface = RustInterface::new();
    let _ = interface.impl_interface();
    Ok(())
}