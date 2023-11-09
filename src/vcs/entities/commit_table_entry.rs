#[derive(Debug, Clone)]
pub struct CommitTableEntry{
    pub id: String,
    pub hash: String,
    pub message: String,
    pub date: String,
}