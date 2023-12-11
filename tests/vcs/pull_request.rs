#[cfg(test)]
mod tests {
    use std::path::Path;

    use rust_git::pull_request::{schemas::schemas::{CreatePullRequest, FindPullRequests, FindPullRequest}, controllers::pull_request::PullRequest};

    #[test]
    pub fn test_01_create_repo_with_valid_parameters() -> Result<(), std::io::Error> {
        let pull_request_server = PullRequest::init(Path::new("tests/pull_request/server_test"));

        let mut pr = CreatePullRequest{
            title: Some(String::from("Title")),
            body: Some(String::from("Description")),
            head_repo: String::from("gmovia/test_create_pr"),
            base_repo: String::from("gmovia/test_create_pr"),
            head: String::from("new_branch"),
            base: String::from("master"),
            username: String::from("ldefeo"),
            mergeable: false
        };

        assert_eq!(pull_request_server.create(&mut pr).is_ok(), true);
        Ok(())
    }

    #[test]
    pub fn test_02_cant_create_repo_with_invalid_name() -> Result<(), std::io::Error> {
        let pull_request_server = PullRequest::init(Path::new("tests/pull_request/server_test"));
        let mut pr = CreatePullRequest{
            title: Some(String::from("Title")),
            body: Some(String::from("Description")),
            head_repo: String::from("gmovia/algo3"),
            base_repo: String::from("gmovia/test_create_pr"),
            head: String::from("new_branch"),
            base: String::from("master"),
            username: String::from("ldefeo"),
            mergeable: false,
        };

        assert_eq!(pull_request_server.create(&mut pr).is_err(), true);
        Ok(())
    }

    #[test]
    pub fn test_03_cant_create_repo_with_invalid_branch() -> Result<(), std::io::Error> {
        let pull_request_server = PullRequest::init(Path::new("tests/pull_request/server_test"));
        let mut pr = CreatePullRequest{
            title: Some(String::from("Title")),
            body: Some(String::from("Description")),
            head_repo: String::from("gmovia/test_create_pr"),
            base_repo: String::from("gmovia/test_create_pr"),
            head: String::from("new_branch2"),
            base: String::from("master"),
            username: String::from("ldefeo"),
            mergeable: false,
        };

        assert_eq!(pull_request_server.create(&mut pr).is_err(), true);
        Ok(())
    }

    #[test]
    pub fn test_04_get() -> Result<(), std::io::Error> {
        let pull_request_server = PullRequest::init(Path::new("tests/pull_request/server_test"));
        let mut pr = CreatePullRequest{
            title: Some(String::from("Title")),
            body: Some(String::from("Description")),
            head_repo: String::from("gmovia/test_create_pr"),
            base_repo: String::from("gmovia/test_create_pr"),
            head: String::from("new_branch"),
            base: String::from("master"),
            username: String::from("ldefeo"),
            mergeable: false
        };

        pull_request_server.create(&mut pr)?;

        let query = FindPullRequests{
            base_repo: String::from("gmovia/test_create_pr"),
            state: None,
            head: None,
            base: None,
            username: None,
            per_page: None
        };

        let prs = pull_request_server.find_all( query)?;

        assert_eq!(prs.len(), 1);
        Ok(())
    }

    #[test]
    pub fn test_05_get_one() -> Result<(), std::io::Error> {
        let pull_request_server = PullRequest::init(Path::new("tests/pull_request/server_test"));
        let mut pr = CreatePullRequest{
            title: Some(String::from("Title")),
            body: Some(String::from("Description")),
            head_repo: String::from("gmovia/test_create_pr"),
            base_repo: String::from("gmovia/test_create_pr"),
            head: String::from("new_branch"),
            base: String::from("master"),
            username: String::from("ldefeo"),
            mergeable: false
        };

        let id = pull_request_server.create(&mut pr)?;


        let query = FindPullRequest {
            base_repo: String::from("gmovia/test_create_pr"),
            id,
        };

        assert_eq!(pull_request_server.find_one(query).is_ok(), true);

        Ok(())
    }

    #[test]
    pub fn test_06_cant_create_pr_that_has_already_been_created() -> Result<(), std::io::Error> {
        let pull_request_server = PullRequest::init(Path::new("tests/pull_request/server_test"));
        let mut pr = CreatePullRequest{
            title: Some(String::from("Title")),
            body: Some(String::from("Description")),
            head_repo: String::from("gmovia/test_create_pr"),
            base_repo: String::from("gmovia/test_create_pr"),
            head: String::from("new_branch"),
            base: String::from("master"),
            username: String::from("ldefeo"),
            mergeable: false
        };

        let _ = pull_request_server.create(&mut pr);

        let mut pr1 = CreatePullRequest{
            title: Some(String::from("Title")),
            body: Some(String::from("Description")),
            head_repo: String::from("gmovia/test_create_pr"),
            base_repo: String::from("gmovia/test_create_pr"),
            head: String::from("new_branch"),
            base: String::from("master"),
            username: String::from("ldefeo"),
            mergeable: false
        };

        assert_eq!(pull_request_server.create(&mut pr1).is_err(), true);
        Ok(())
    }
}