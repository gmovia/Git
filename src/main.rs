use rust_git::interfaces::interface::RustInterface;

fn main() -> Result<(), std::io::Error>{
    let interface = RustInterface::new();
    interface.impl_interface()?;
    Ok(())
}