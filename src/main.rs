use rust_git::interface::interface::RustInterface;

fn main() -> Result<(), std::io::Error>{
    let interface = RustInterface::new();
    let _ = interface.impl_interface();
    Ok(())
}
