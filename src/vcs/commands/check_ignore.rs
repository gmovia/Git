use std::{path::Path, fs::{self, OpenOptions}, io::{Read, self, Write}};

pub struct CheckIgnore;

impl CheckIgnore {

    pub fn check_ignore(current_path: &Path, path: &Path) -> Result<bool,std::io::Error>{
        let ignore_path = current_path.join(".gitignore");
        
        let content = fs::read_to_string(&ignore_path)?;
        let lines: Vec<&str> = content.lines().collect();
                
        if Self::rule_full_path(path, &lines)? == true{
            return Ok(true);
        }
        
        if Self::rule_dir(path, &lines)? == true{
            return Ok(true);
        }

        Ok(false)
    }

    

    /// Ver si matcha con una ruta especifica
    pub fn rule_full_path(path: &Path, ignore_paths: &Vec<&str>) -> Result<bool, std::io::Error>{
        let path = path.to_str();
        for ignore_path in ignore_paths{
            if path == Some(*ignore_path){
                return Ok(true);
            }
        }
        Ok(false)
    }
    // qeu es ignore_paths?
    /// Ver si matchea con un dir especifico
    pub fn rule_dir(path: &Path, ignore_paths: &Vec<&str>) -> Result<bool, std::io::Error>{
        for ignore_path in ignore_paths{
            let ignore_path = Path::new(&ignore_path);
             if path.starts_with(ignore_path) == true{
                return Ok(true);
             }
        }
        Ok(false)
    }

    // Ver si matchea con un comodin
    //pub fn rule_wildcard(path: String, ignore_paths: &Vec<String>) -> Result<bool, std::io::Error>{ // PENSAR MAS
        /*
        mira eso
No, el asterisco (*) no tiene que estar al inicio del patrón. Puedes colocar el asterisco en cualquier parte del patrón para hacer coincidir caracteres dentro de un directorio o en una cadena de texto. Aquí hay algunos ejemplos:
        // perdon es solo un asterisco
        // si el asterisco esta al inicio entonces se ignoran todo lo que contiene el final, ej *.txt todo lo q tenga .txt
        // si el asterisco esta al final entonces se ignora todo lo que tiene al principio file* se ignora file.txt file2.txt etc
        // el problema es si esta en el medio, se ignoran los que empiecen con lo de la izq y terminen con lo de la derecha
        // ademas puede haber muchos asteriscos pero olvidate, no tiene relevancia meterse en ese caso, vamos a los 3 de arriba!
*.txt: Coincidirá con todos los archivos que tengan una extensión .txt en el directorio actual.
file*: Coincidirá con archivos que comiencen con "file" seguido por cualquier cantidad de caracteres.
abc*xyz: Coincidirá con archivos que comiencen con "abc" y terminen con "xyz", con cualquier cantidad de caracteres en el medio.
        */
        //let path = Path::new(&path);
        //for ignore_path in ignore_paths{
        //     if path.to_string_lossy().to_string().contains("**") == true{
        //        return Ok(true);
        //     }
        //}
    //    Ok(false)
    //}
}