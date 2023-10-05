#[derive(Clone)]
pub struct VSCFile {
    pub path: String,
    pub content: String,
    pub state: String,
}

impl VSCFile {
    pub fn new(path: String, content: String, state: String) -> VSCFile {
        VSCFile {
            path,
            content,
            state,
        }
    }
}