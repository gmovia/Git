use serde::{Serialize, Deserialize};

use crate::vcs::entities::commit_entity::CommitEntity;

#[derive(Serialize, Deserialize)]
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

#[derive(Debug, Clone)]
pub struct CommitsPullRequest {
    pub id: String,
    pub parent: String,
    pub hash: String,
    pub message: String,
    pub date: String,
    pub info: CommitEntity
}