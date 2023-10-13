use std::{path::Path, fs::{self, File}, io::Write};
/// Este Struct representa el comando git init. El cual se encarga de inicializar un repostorio.
pub struct Init {
    pub example_text: String,
}

impl Init {
    
    /// Esta funcion es el constructor de init. Se crean los directorios y archivos necesarios.
    pub fn git_init(path: &str, args: Vec<String>){
        let init = { Init { example_text: "hola".to_string() } };

        if args.len() < 4 {
            if let Err(e) = init.create_initial_folders(path, "master") {
                println!("Error: {}",e);
            }   
        }
        else {
            if args.contains(&"-b".to_string()) {
                if let Some(index) = args.iter().position(|s| s == &"-b"){
                    if let Err(e) = init.create_initial_folders(path, args[index+1].as_str()) {
                        println!("Error: {}",e);
                    }
                }
                else {
                    println!("Error creating git folder")
                }
            }
            else {
                if let Err(e) = init.create_initial_folders(path, "master") {
                    println!("Error: {}",e);
                }
            }
        }
    }

    /// Esta funcion es la encargada de crear todsas las carpetas y archivos necesarios luego de ejecutar git init.
    fn create_initial_folders(&self, path: &str, branch_name: &str) -> Result<(),std::io::Error> {
        let path = Path::new(path).join(".rust_git");
        fs::create_dir_all(&path)?;

        self.create_git_hooks_folder(&path)?;
        self.create_git_info_folder(&path)?;
        self.create_git_logs_folder(&path)?;
        self.create_git_objects_folder(&path)?;
        self.create_git_refs_folder(&path)?;
        self.create_git_config_file(&path)?;
        self.create_head_file(&path, branch_name)?;
        self.create_index(&path)?;        
        Ok(())
    }

    // Crea el archivo index
    fn create_index(&self, git_path: &Path) -> Result<(),std::io::Error>{
        let index_path = git_path.join("index");
        fs::OpenOptions::new().create(true).append(true).open(&index_path)?;
        Ok(())
    }

    /// Crea el directorio hooks al inicializar un nuevo repositorio
    fn create_git_hooks_folder(&self, git_path: &Path) -> Result<(),std::io::Error> {
        let hooks_path = git_path.join("hooks");
        fs::create_dir_all(hooks_path)?;
        Ok(())
    }
    
    /// Crea el directorio info al inicializar un nuevo repositorio
    fn create_git_info_folder(&self, git_path: &Path) -> Result<(),std::io::Error> {
        let info_path = git_path.join("info");
        fs::create_dir_all(info_path)?;
        Ok(())
    }

    /// Crea el directorio logs al inicializar un nuevo repositorio
    fn create_git_logs_folder(&self, git_path: &Path) -> Result<(),std::io::Error> {
        let logs_path = git_path.join("logs");
        fs::create_dir_all(logs_path)?;
        Ok(())
    }

    /// Crea el directorio objects al inicializar un nuevo repositorio
    fn create_git_objects_folder(&self, git_path: &Path) -> Result<(),std::io::Error> {
        let objects_path = git_path.join("objects");
        fs::create_dir_all(objects_path)?;
        Ok(())
    }

    /// Crea el directorio refs al inicializar un nuevo repositorio
    fn create_git_refs_folder(&self, git_path: &Path) -> Result<(),std::io::Error> {
        let refs_path = git_path.join("refs");
        fs::create_dir_all(&refs_path)?;
        fs::create_dir_all(&refs_path.join("heads"))?;
        fs::create_dir_all(&refs_path.join("tags"))?;
        Ok(())
    }   

    /// Crea el archivo config al inicializar un nuevo repositorio
    fn create_git_config_file(&self, git_path: &Path) -> Result<(),std::io::Error> {
        let config_path = git_path.join("config");
        
        if let Ok(_) = fs::File::open(&config_path) {

        } else {
            let mut file = File::create(config_path)?;
            file.write_all(format!("[core]\n").as_bytes())?;
            file.write_all(format!("    repostiryformatversion = 0\n").as_bytes())?;
            file.write_all(format!("    filemode = false\n").as_bytes())?;
            file.write_all(format!("    bare = false\n").as_bytes())?;
        }

        Ok(())
    } 

    /// Crea el archivo HEAD al inicializar un nuevo repositorio
    fn create_head_file(&self, git_path: &Path, branch_name: &str) -> Result<(),std::io::Error> {
        let head_path = git_path.join("HEAD");

        if let Ok(_) = fs::File::open(&head_path) {
            println!("warning: re-init: ignored --initial-branch={}", branch_name);
        } else {
            let mut file = File::create(head_path)?;
            file.write_all(format!("ref: refs/heads/{}", branch_name).as_bytes())?;
        }            
        Ok(())
    }

}