use crate::vcs::{files::current_repository::CurrentRepository, version_control_system::VersionControlSystem};

use super::init::Init;

pub struct Pull;

impl Pull{
    pub fn pull(message: String) -> Result<(), std::io::Error> {
        let current = CurrentRepository::read()?;
        let branch_name = Init::get_current_branch(&current)?;
        
        let parts: Vec<&str> = message.split_whitespace().collect();
    
        VersionControlSystem::fetch(format!("git fetch {}", parts[2]))?;
        VersionControlSystem::rebase(&format!("origin_{}", branch_name))?;

        Ok(())
    }
}