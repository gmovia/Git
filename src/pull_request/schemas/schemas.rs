use serde::{Serialize, Deserialize};

use crate::vcs::entities::commit_entity::CommitEntity;

#[derive(Clone, Debug, Serialize, Deserialize)]
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
    pub end_commit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitsPullRequest {
    pub id: String,
    pub parent: String,
    pub hash: String,
    pub message: String,
    pub date: String,
    pub info: CommitEntity
}

pub struct UpdatePullRequest{
    pub id: String,
    pub base_repo: String,
    pub title: Option<String>,
    pub body: Option<String>,
    pub status: Option<String>,
    pub base: Option<String>,
}