#[cfg(test)]
mod tests {
    use std::path::Path;

    use rust_git::{pull_request::{schemas::schemas::{FindPullRequests}, controllers::pull_request::PullRequest}, server_http::requests::{create_pull_request::CreatePullRequest, list_pull_request::ListPullRequests}};
    use rust_git::server_http::requests::get_pull_request::GetPullRequest;

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

        let query = ListPullRequests {
            base_repo: String::from("gmovia/test_create_pr"),
            status: None,
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


        let query = GetPullRequest {
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

    #[test]
    pub fn test_07_get_commits_of_a_pull_request() -> Result<(), std::io::Error> {
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

        let query = GetPullRequest {
            base_repo: String::from("gmovia/test_create_pr"),
            id,
        };

        let commits = pull_request_server.find_commits(query)?;
        
        assert_eq!(commits.len(), 2);
        Ok(())
    }
}