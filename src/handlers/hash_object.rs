use crate::{
    constants::constant::{BLOB_CODE, ERR_PATH_IS_NOT_DIRECTORY_OR_NO_SUCH_OR_DIRECTORY},
    vcs::{
        commands::hash_object::WriteOption, files::current_repository::CurrentRepository,
        version_control_system::VersionControlSystem,
    },
};

pub fn handler_hash_object(input: String) -> String {
    let args: Vec<&str> = input.split_whitespace().collect();
    if args.len() == 4 {
        // -w
        if let Ok(current) = CurrentRepository::read() {
            let input_path = current.join(args[3]);
            if let Ok(hash) =
                VersionControlSystem::hash_object(&input_path, WriteOption::Write, BLOB_CODE)
            {
                return hash;
            }
        }
    }
    if let Ok(current) = CurrentRepository::read() {
        let input_path = current.join(args[2]);
        if let Ok(hash) =
            VersionControlSystem::hash_object(&input_path, WriteOption::NoWrite, BLOB_CODE)
        {
            return hash;
        }
    }

    ERR_PATH_IS_NOT_DIRECTORY_OR_NO_SUCH_OR_DIRECTORY.to_string()
}
