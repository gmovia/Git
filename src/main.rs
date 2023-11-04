use rust_git::{interface::interface::RustInterface, handlers::commands::handler_command, vcs::version_control_system::VersionControlSystem};
use std::{io::{self, Write}, path::Path};

fn main() -> Result<(), std::io::Error>{
    // let mut vcs = VersionControlSystem::init(Path::new("test_folder"), Vec::new());
    // loop{
    //     let mut input = String::new();
    //     io::stdout().flush().unwrap();
    //     io::stdin().read_line(&mut input).unwrap();
    //     handler_command(&mut vcs, &input);
    // }
    let interface = RustInterface::new();
    let _ = interface.impl_interface();
    Ok(())
    //
}
