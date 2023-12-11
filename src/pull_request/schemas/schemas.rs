pub struct PullRequestEntry{
    pub id: String,
    pub title: String,
    pub body: String,
    pub head_repo: String,
    pub base_repo: String,
    pub head: String,
    pub base: String,
    pub username: String,
    pub status: String,
    pub mergeable: bool,
    pub init_commit: String,
    pub last_commit: Option<String>,
}

pub struct CreatePullRequest{
    pub title: Option<String>,
    pub body: Option<String>,
    pub head_repo: String,
    pub base_repo: String,
    pub head: String,
    pub base: String,
    pub username: String,
    pub mergeable: bool,
}

pub struct FindPullRequests{
    pub base_repo: String,
    pub state: Option<String>,
    pub head: Option<String>,
    pub base: Option<String>,
    pub username: Option<String>,
    pub per_page: Option<i32> // default 30
}

pub struct FindPullRequest{
    pub base_repo: String,
    pub id: String,
}


#[derive(Debug, Clone)]
pub struct CommitsPullRequest {
    pub id: String,
    pub parent: String,
    pub hash: String,
    pub message: String,
    pub info: String,
}