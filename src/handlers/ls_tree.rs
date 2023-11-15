use crate::vcs::version_control_system::VersionControlSystem;


pub fn handler_ls_tree(input: String) -> String{

    let args: Vec<&str> = input.split_whitespace().collect();
    match args.len() {
        3 => {if let Ok(information) = VersionControlSystem::ls_tree(args[2]) {
                for info in information {
                    println!("{:?}\n",info);
                }
            }return "Ok".to_string();},

        4 => {return "Ok".to_string();},
        _ => {return "Error".to_string();},
    }
}