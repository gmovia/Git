use crate::vcs::{version_control_system::VersionControlSystem, commands::show_ref::ShowRefOptions};



pub fn handler_show_ref(input: String) -> String {
    let args: Vec<&str> = input.split_whitespace().collect();

    match args.len() {
        2 => {if let Ok(refs) = VersionControlSystem::show_ref(ShowRefOptions::GetAll) {
                for (key, value) in refs {
                    let format = format!("{} ---> {}", key, value);
                    println!("{:?}",format);
                }
                return "Ok".to_string();
            }
        },
        3 => {match args[2] {
                "--heads" => {if let Ok(refs) = VersionControlSystem::show_ref(ShowRefOptions::GetRefHeads) {
                    for (key, value) in refs {
                        let format = format!("{}\n          {}\n", key, value);
                        println!("{:?}",format);
                    }
                    return "Ok".to_string();
                }},
                "--tags" => {if let Ok(refs) = VersionControlSystem::show_ref(ShowRefOptions::GetRefTags) {
                    for (key, value) in refs {
                        let format = format!("{}\n          {}\n", key, value);
                        println!("{:?}",format);
                    }
                    return "Ok".to_string();
                }},
                _ => {return "Err".to_string();}
            }
        },
        _ => {return "Err".to_string();},
    }
    "Err".to_string()
}