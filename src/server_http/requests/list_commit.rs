
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ListCommitsPullRequest{
    pub base_repo: String,
    pub id: String,
}


impl ListCommitsPullRequest {

    pub fn list_commits_pull_request(json_body: &str) -> Result<ListCommitsPullRequest, std::io::Error> {
        println!("JSON Body: {}", json_body);
        if let Ok(request) = serde_json::from_str::<ListCommitsPullRequest>(json_body) {            
            return Ok(request)
        } else {
            println!("Error al deserializar el mensaje: trailing characters");
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Error parsing request"))
        };
    }


}