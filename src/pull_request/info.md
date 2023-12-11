### Endpoints

En la fase inicial del desarrollo, los endpoints a implementar son:

1. **Create a pull request**
   - **Método HTTP:** POST
   - **Ruta:** `/repos/{owner}/{repo}/pull`

- **Request:**
  - **Atributos:**
    - `title` (String, Opcional): Representa el título del pull request.
    - `body` (String, Opcional): Descripción del pull request.
    - `maintainer_can_modify` (Bool, Opcional): Indica si los mantenedores pueden modificar el pull request.
    - `head_repo` (String, Opcional): Representa el nombre del repositorio donde se realizaron los cambios en el pull request.
    - `head` (String, Requerido): Representa el nombre de la rama donde están implementados los cambios, con el nombre de usuario al inicio.
    - `base` (String, Requerido): Representa el nombre de la rama en la que se desean integrar los cambios.



- **Ejemplo: POST /repos/gmovia/algo1/pull:**
  ```json
  {
      "title": "first pull request",
      "head": "gmovia:new_branch",
      "base": "main",
      "head_repo": "gmovia/algo1"
  }
  ```

- **Respuesta:**
  - **Códigos de Estado:**
    - `201 - OK`: La solicitud se completó con éxito.
    - `403 - Forbidden`: No se permite la acción realizada por el usuario.
    - `422 - Validation failed, or the endpoint has been spammed`: La validación falló.

