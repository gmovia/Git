#[derive(Clone)]
pub struct VCSFile {
    pub path: String,
    pub content: String,
    pub state: String,
}

impl VCSFile {
    pub fn new(path: String, content: String, state: String) -> VCSFile {
        VCSFile {
            path,
            content,
            state,
        }
    }
}
