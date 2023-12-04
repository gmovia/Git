use crate::{
    constants::constant::{
        CHANGES_NOT_BE_COMMITED, CHANGES_TO_BE_COMMITED, ERR_STATUS, UNTRACKED_FILES,
    },
    vcs::version_control_system::VersionControlSystem,
};

pub fn handler_status() -> String {
    let mut result = String::new();

    if let Ok((untracked, not_commited, commited)) = VersionControlSystem::status() {
        result.push_str(UNTRACKED_FILES);
        result.push('\n');

        for (key, value) in untracked {
            result.push_str(&format!("{} {}\n", key, value));
        }
        result.push_str(CHANGES_NOT_BE_COMMITED);
        result.push('\n');

        for (key, value) in not_commited {
            result.push_str(&format!("{} {}\n", key, value));
        }
        result.push_str(CHANGES_TO_BE_COMMITED);
        result.push('\n');

        for (key, value) in commited {
            result.push_str(&format!("{} {}\n", key, value));
        }
    } else {
        return ERR_STATUS.to_string();
    }
    println!("{}", result);
    result
}
