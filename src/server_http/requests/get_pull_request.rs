
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct GetPullRequest{
    pub base_repo: String,
    pub id: String,
}


impl GetPullRequest {

    pub fn get_pull_request(json_body: &str) -> Result<GetPullRequest, std::io::Error> {
        println!("JSON Body: {}", json_body);
        if let Ok(request) = serde_json::from_str::<GetPullRequest>(json_body) {            
            
            return Ok(request)
        } else {
            println!("Error al deserializar el mensaje: trailing characters");
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Error parsing request"))
        };
    }


}