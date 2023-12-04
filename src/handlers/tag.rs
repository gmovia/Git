use crate::vcs::{commands::tag::TagOptions, version_control_system::VersionControlSystem};

pub fn handler_tag(input: String) -> String {
    let args: Vec<&str> = input.split_whitespace().collect();

    match args.len() {
        2 => {
            if let Ok(tags) = VersionControlSystem::tag(TagOptions::Get) {
                for tag in tags {
                    println!("tag: {:?}", tag);
                }
            }
            "Ok".to_string()
        }
        3 => {
            let _ = VersionControlSystem::tag(TagOptions::CreateLight(args[2]));
            "CREATED SUCCESSFULLY".to_string()
        }
        4 => match args[2] {
            "-d" => {
                let _ = VersionControlSystem::tag(TagOptions::Delete(args[3]));
                "DELETED SUCCESSFULLY".to_string()
            }
            _ => "ERR_TAG".to_string(),
        },
        5 => match args[2] {
            "-a" => {
                let _ = VersionControlSystem::tag(TagOptions::Create(args[3], args[4]));
                "CREATED SUCCESSFULLY".to_string()
            }
            _ => "ERR_TAG".to_string(),
        },
        _ => "ERR_TAG".to_string(),
    }
}
