use crate::vcs::version_control_system::VersionControlSystem;

pub fn handler_status(vcs: &VersionControlSystem) -> String {
    let mut result = String::new();
    
    if let Ok((untracked, not_commited, commited)) = vcs.status() {
        result.push_str("UNTRACKED\n");
        for (key, value) in untracked {
            result.push_str(&format!("{} {}\n", key, value));
        }
        result.push_str("NOT COMMITED\n");
        for (key, value) in not_commited {
            result.push_str(&format!("{} {}\n", key, value));
        }
        result.push_str("AREA\n");
        for (key, value) in commited {
            result.push_str(&format!("{} {}\n", key, value));
        }
    } else {
        return "Failed to get status".to_string();
    }
    result
}
