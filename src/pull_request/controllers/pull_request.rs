use std::{path::Path, io};

use crate::pull_request::{schemas::schemas::CreatePullRequest, validator::validator::Validator, db::queries::Query};

pub struct PullRequest;

/*
Example

CreatePullRequest{
    title: "Mergeo new_branch a main",
    description: "Estoy creando mi primer pull request",
    base_repo: "gmovia/algo2",
    head_repo: "gmovia/algo1",
    base: "main",
    head: "new_branch",
    username: "ldefeo" // esta persona es la que crea el PR. No necesariamente tiene que ser el dueño del repo.
}

Primero tendria que fijarme si gmovia/algo1 existen en el servidor, ya que en este caso, ambos repositorios son iguales. Luego
tendria que fijarme si ambas ramas (main y new_branch) existen dentro de ese repositorio. Pero guarda, este es un caso particular.
Si ambos fueran de distinto repositorio, que es la logica que debemos implementar, tenemos que fijarnos si base_repo existe y luego
si base existe dentro del repositorio, y lo mismo con el head_repo. 

En el caso de existir algun error de validacion deberiamos devolverlo. Si el procedimiento es exitoso y los datos son validos, entonces
podemos crear el PR en la base de datos y devolverle al usuario un identificador.
*/

impl PullRequest { 
    pub fn create(server: &Path, pr: CreatePullRequest) -> Result<String, std::io::Error>{
        if Validator::validate_create_pull_request(server, &pr).is_err(){
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "The pull request is not valid",
            ));
        }

        Ok(Query::create_pull_request(server,pr)?.to_string())
    }
}