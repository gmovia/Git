use std::{path::Path, io::{self, Write}};
use rust_git::{vcs::{version_control_system::VersionControlSystem, files::config::Config}, handlers::commands::handler_command, interfaces::login::DrawLogin};
use rust_git::interfaces::interface::RustInterface;


fn main() -> Result<(), std::io::Error>{
    let draw_login = DrawLogin::new();
    let result = Config::read_config();
    if result.is_err() {
        let _ = draw_login.impl_login();
    }else {
        let interface = RustInterface::new();
        let _ = interface.impl_interface();
    }
    Ok(())
}