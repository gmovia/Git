use std::{path::{Path, PathBuf}, fs::{self, File}, io::{Write, Read}};
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
        self.create_git_refs_folder(&path, branch_name)?;
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
        let commits_path = logs_path.join("master");
        fs::create_dir_all(logs_path)?;
        fs::OpenOptions::new().create(true).append(true).open(&commits_path)?;
        Ok(())
    }

    /// Crea el directorio objects al inicializar un nuevo repositorio
    fn create_git_objects_folder(&self, git_path: &Path) -> Result<(),std::io::Error> {
        let objects_path = git_path.join("objects");
        fs::create_dir_all(objects_path)?;
        Ok(())
    }

    /// Crea el directorio refs al inicializar un nuevo repositorio
    fn create_git_refs_folder(&self, git_path: &Path, branch_name: &str) -> Result<(), std::io::Error> {
        let refs_path = git_path.join("refs");
        let heads_path = refs_path.join("heads");
        let branch_path = heads_path.join(branch_name);
        
        fs::create_dir_all(&refs_path)?;
        fs::create_dir_all(&heads_path)?;
        
        let mut branch_file = File::create(&branch_path)?;
        
        branch_file.write_all(b"logs/")?;
        branch_file.write_all(branch_name.as_bytes())?;
            
        fs::create_dir_all(refs_path.join("tags"))?;
            
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
            file.write_all(format!("refs/heads/{}", branch_name).as_bytes())?;
        }            
        Ok(())
    }

    pub fn get_object_path(path: &String) -> Result<PathBuf,std::io::Error>{
        let p = Path::new(path);
        let objects_path = p.join(".rust_git").join("objects");
        Ok(Path::new(&objects_path).to_path_buf())
    }

    pub fn get_commits_path(path: &String) -> Result<PathBuf,std::io::Error>{
        let p = Path::new(path); // OJO LAS BARRAS EN WINDOWS NO VAN A ANDAR! FIJATE QUE EN HEAD ESTAN ASI / Y NO ASI \
        // VER COMO RESOLVERLO DSP! EN LINUX TODO OK!

        let head_path = p.join(".rust_git").join("HEAD");
        let mut head_file = File::open(head_path)?;
        
        let mut content = String::new();
        head_file.read_to_string(&mut content)?;
        
        let refs_path = p.join(".rust_git").join(content);
        let mut refs_file = File::open(refs_path)?;
        
        let mut content = String::new();
        refs_file.read_to_string(&mut content)?;
        
        let commits_path = p.join(".rust_git").join(content);
        Ok(Path::new(&commits_path).to_path_buf())
    }

}