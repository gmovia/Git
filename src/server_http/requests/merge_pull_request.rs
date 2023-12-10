
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct MergePullRequest{
    repo: String,
    pull_number: String,
    commit_tittle: String,
    commit_message: String,
}


impl MergePullRequest {

    pub fn merge_pull_request(json_body: &str) -> Result<MergePullRequest, std::io::Error> {
        println!("JSON Body: {}", json_body);
        if let Ok(request) = serde_json::from_str::<MergePullRequest>(json_body) {            
            return Ok(request)
        } else {
            println!("Error al deserializar el mensaje: trailing characters");
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Error parsing request"))
        };
    }


}