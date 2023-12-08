#[cfg(test)]
mod tests {
    use std::{path::Path, fs};

    use rust_git::pull_request::{schemas::schemas::CreatePullRequest, controllers::pull_request::PullRequest};

    #[test]
    pub fn test_01_create_repo_with_valid_parameters() -> Result<(), std::io::Error> {
        let pr = CreatePullRequest{
            title: Some(String::from("Title")),
            body: Some(String::from("Description")),
            head_repo: String::from("gmovia/test_create_pr"),
            base_repo: String::from("gmovia/test_create_pr"),
            head: String::from("new_branch"),
            base: String::from("master"),
            username: String::from("ldefeo"),
        };

        assert_eq!(PullRequest::create(Path::new("tests/pull_request/server_test"), pr).is_ok(), true);
        Ok(())
    }

    #[test]
    pub fn test_02_cant_create_repo_with_invalid_name() -> Result<(), std::io::Error> {
        let pr = CreatePullRequest{
            title: Some(String::from("Title")),
            body: Some(String::from("Description")),
            head_repo: String::from("gmovia/algo3"),
            base_repo: String::from("gmovia/test_create_pr"),
            head: String::from("new_branch"),
            base: String::from("master"),
            username: String::from("ldefeo"),
        };

        assert_eq!(PullRequest::create(Path::new("tests/pull_request/server_test"), pr).is_err(), true);
        Ok(())
    }

    #[test]
    pub fn test_03_cant_create_repo_with_invalid_branch() -> Result<(), std::io::Error> {
        let pr = CreatePullRequest{
            title: Some(String::from("Title")),
            body: Some(String::from("Description")),
            head_repo: String::from("gmovia/test_create_pr"),
            base_repo: String::from("gmovia/test_create_pr"),
            head: String::from("new_branch2"),
            base: String::from("master"),
            username: String::from("ldefeo"),
        };

        assert_eq!(PullRequest::create(Path::new("tests/pull_request/server_test"), pr).is_err(), true);
        Ok(())
    }
}