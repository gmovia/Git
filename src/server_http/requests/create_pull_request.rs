use std::net::TcpStream;

use serde::{Deserialize, Serialize};
use serde_xml_rs::from_str as xml_from_str;

use crate::{
    pull_request::controllers::pull_request::PullRequest,
    server_http::sender::{send_error, send_response, send_server_error_msg},
};

#[derive(Serialize, Deserialize)]
pub struct StructCreatePR {
    title: Option<String>,
    body: Option<String>,
    head_repo: Option<String>,
    head: String,
    base: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreatePullRequest {
    pub title: Option<String>,
    pub body: Option<String>,
    pub head_repo: String,
    pub base_repo: String,
    pub head: String,
    pub base: String,
    pub username: String,
    pub mergeable: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseOkCreatePullRequest {
    id: String,
}

impl CreatePullRequest {
    pub fn response_create_pull_request_object(
        json_body: &str,
        path: String,
        stream: &mut TcpStream,
        pull_request: PullRequest,
        media_type: &str,
    ) -> Result<CreatePullRequest, std::io::Error> {
        match media_type {
            "application/json" => Self::deserialize_json(json_body, path, stream, pull_request),
            "application/xml" => Self::deserialize_xml(json_body, path, stream, pull_request),
            _ => {
                println!("Tipo de media no soportado: {}", media_type);
                send_server_error_msg(stream);
                Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Tipo de media no soportado",
                ))
            }
        }
    }

    fn deserialize_json(
        json_body: &str,
        path: String,
        stream: &mut TcpStream,
        pull_request: PullRequest,
    ) -> Result<CreatePullRequest, std::io::Error> {
        if let Ok(request) = serde_json::from_str::<StructCreatePR>(json_body) {
            let head_vec: Vec<&str> = request.head.split(':').collect();

            let mut create = CreatePullRequest {
                title: request.title,
                body: request.body,
                head_repo: "amoralejo/algo1".to_string(),
                base_repo: path,
                head: head_vec[1].to_owned(),
                base: request.base,
                username: head_vec[0].to_owned(),
                mergeable: false,
            };

            if request.head_repo.is_none() {
                create.head_repo = create.base_repo.clone()
            }

            match pull_request.create(&mut create) {
                Ok(response) => send_response(stream, ResponseOkCreatePullRequest { id: response }),
                Err(error_code) => send_error(stream, error_code.to_string()),
            };

            Ok(create)
        } else {
            println!("Error al deserializar el mensaje: trailing characters");
            send_server_error_msg(stream);
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Error parsing request",
            ))
        }
    }

    fn deserialize_xml(
        xml_body: &str,
        path: String,
        stream: &mut TcpStream,
        pull_request: PullRequest,
    ) -> Result<CreatePullRequest, std::io::Error> {
        if let Ok(request) = xml_from_str::<StructCreatePR>(xml_body) {
            let head_vec: Vec<&str> = request.head.split(':').collect();

            let mut create = CreatePullRequest {
                title: request.title,
                body: request.body,
                head_repo: "amoralejo/algo1".to_string(),
                base_repo: path,
                head: head_vec[1].to_owned(),
                base: request.base,
                username: head_vec[0].to_owned(),
                mergeable: false,
            };

            if request.head_repo.is_none() {
                create.head_repo = create.base_repo.clone()
            }

            match pull_request.create(&mut create) {
                Ok(response) => send_response(stream, ResponseOkCreatePullRequest { id: response }),
                Err(error_code) => send_error(stream, error_code.to_string()),
            };

            Ok(create)
        } else {
            println!("Error al deserializar el mensaje: trailing characters");
            send_server_error_msg(stream);
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Error parsing request",
            ))
        }
    }
}
