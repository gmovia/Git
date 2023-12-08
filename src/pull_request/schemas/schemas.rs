pub struct CreatePullRequest{
    pub title: Option<String>,
    pub body: Option<String>,
    pub head_repo: Option<String>,
    pub base_repo: String,
    pub head: String,
    pub base: String,
    pub username: String
}
