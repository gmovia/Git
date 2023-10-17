use crate::vcs::version_control_system::VersionControlSystem;

/// Recibe input del comando, por ejemplo, git commit "primer commit"
/// Llama a commit() para agregarlo en la tabla de commits
pub fn handler_commit(vcs: &mut VersionControlSystem, input: String) -> Result<(), std::io::Error>{
    let mut chain = String::new();
    let mut args: Vec<&str> = input.split(" ").collect();
    args.remove(1);
    args.remove(0);
    for element in &args{
        chain += element;
        chain.push( ' ');
    }
    vcs.commit(chain.to_string())?;
    
    Ok(())
}