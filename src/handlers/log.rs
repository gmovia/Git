use crate::vcs::commands::init::Init;
use crate::vcs::version_control_system::VersionControlSystem;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn handler_log(vcs: &VersionControlSystem) -> Result<(), std::io::Error> {
    let commits_file = File::open(Init::get_commits_path(&vcs.path)?)?;
    let reader = BufReader::new(commits_file);

    let has_commits = reader.lines().count() > 0;

    if has_commits {
        vcs.log()?;
    } else {
        println!("No commits exist");
    }
    Ok(())
}
