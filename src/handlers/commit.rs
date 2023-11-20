use crate::{vcs::version_control_system::VersionControlSystem, constants::constants::RESPONSE_OK_COMMIT};

pub fn handler_commit(input: String) -> String{
    let mut chain = String::new();
    let mut args: Vec<&str> = input.split(' ').collect();
    args.remove(1);
    args.remove(0);
    for element in &args{
        chain += element;
        chain.push( ' ');
    }
    let _ = VersionControlSystem::commit(chain.to_string());
    RESPONSE_OK_COMMIT.to_string()
}