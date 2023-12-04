use rust_git::interfaces::interface::RustInterface;
use rust_git::interfaces::login::DrawLogin;
use rust_git::vcs::files::config::Config;

fn main() -> Result<(), std::io::Error> {
    let draw_login = DrawLogin::new();
    let result = Config::read_config();
    if result.is_err() {
        draw_login.impl_login();
    } else {
        let interface = RustInterface::new();
        let _ = interface.impl_interface();
    }
    Ok(())
}

/*
fn main() -> Result<(), std::io::Error>{
    let _ = VersionControlSystem::init(Path::new("test_delta"), Vec::new());
    loop{
        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        handler_command( &input);
    }

}
*/
