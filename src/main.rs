use rust_git::version_control_system::VersionControlSystem;
use std::{io::{self, Write}, path::Path};

fn main() { // PARA VER COMO FUNCIONA! NO SUBIR ESTO A DEVELOPMENT!
    let mut vsc = VersionControlSystem::init("/Users/gmovia/Desktop/FIUBA/hola".to_string());
    //vsc.staging_area.insert("/Users/gmovia/Desktop/prueba/archivo1.txt".to_string(), VSCFile::new("/Users/gmovia/Desktop/prueba/archivo1.txt".to_string(), "".to_string(), "CREATED".to_string()));

    loop{
        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        let file1 = Path::new("/Users/gmovia/Desktop/FIUBA/hola/archivo1.txt");
        let file2 = Path::new("/Users/gmovia/Desktop/FIUBA/hola/archivo2.txt");
        let directory = &vsc.path.clone();

        vsc.local_repository.insert(file1.display().to_string(), "contenido".to_string());

        match input{
            "git add ." => {let _ =vsc.add(Path::new(&directory));},
            "git add archivo1.txt" => {let _ = vsc.add(file1);}, 
            "git add archivo2.txt" => {let _ = vsc.add(file2);},
            "git status" => {
                if let Ok((untracked, not_commited, commited)) = vsc.status(){
                    println!("UNTRACKED");
                    for (key, value) in untracked{
                        println!("{} {}", key, value);
                    }
                    println!("NOT COMMITED");
                    for (key, value) in not_commited{
                        println!("{} {}", key, value);
                    }
                    println!("AREA");
                    for (key, value) in commited{
                        println!("{} {}", key, value);
                    }
                }
            },
            _ => ()
        }
    }
}

// CASO A ANALIZAR, INTERESANTE, SI EL ARCHIVO ESTA ELIMINADO Y EN EL REPO ENTONCES GIT ADD NO LO AGREGA => mandas algo que NO existe => cazar el caso de que no exista y este en el repo
// si no esta en files y esta en repo local, entonces no hay otra que un DELETE