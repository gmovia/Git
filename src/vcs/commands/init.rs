use std::{path::{Path, PathBuf}, fs::{self, File, OpenOptions}, io::{Write, Read, self}};

use crate::{constants::constant::{RUST_PATH, COMMIT_INIT_HASH}, vcs::files::{current_repository::CurrentRepository, config::Config}};

/// Este Struct representa el comando git init. El cual se encarga de inicializar un repostorio.
pub struct Init {
    pub example_text: String,
}

impl Init {
    
    /// Esta funcion es el constructor de init. Se crean los directorios y archivos necesarios.
    pub fn git_init(path: &Path, args: Vec<String>){
        let init = { Init { example_text: "hola".to_string() } };

        if args.len() < 4 {
            if let Err(e) = init.create_initial_folders(path, "master") {
                println!("Error: {}",e);
            }   
        }
        else if args.contains(&"-b".to_string()) {
            if let Some(index) = args.iter().position(|s| s == "-b"){
                if let Err(e) = init.create_initial_folders(path, args[index+1].as_str()) {
                    println!("Error: {}",e);
                }
            }
            else {
                println!("Error creating git folder")
            }
        }
        else if let Err(e) = init.create_initial_folders(path, "master") {
            println!("Error: {}",e);
        }
    }

    /// Esta funcion es la encargada de crear todsas las carpetas y archivos necesarios luego de ejecutar git init.
    fn create_initial_folders(&self, path: &Path, branch_name: &str) -> Result<(),std::io::Error> {
        let path = path.join(".rust_git");
        fs::create_dir_all(&path)?;

        self.create_git_hooks_folder(&path)?;
        self.create_git_info_folder(&path)?;
        self.create_git_logs_folder(&path, branch_name)?;
        self.create_git_objects_folder(&path)?;
        self.create_git_refs_folder(&path, branch_name)?;
        self.create_git_config_file(&path)?;
        self.create_head_file(&path, branch_name)?;
        self.create_index(&path)?;    
        self.create_git_ignore()?;   
        Ok(())
    }

    // Crea el archivo index
    fn create_index(&self, git_path: &Path) -> Result<(),std::io::Error>{
        let index_path = git_path.join("index");
        fs::OpenOptions::new().create(true).append(true).open(index_path)?;
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
    fn create_git_logs_folder(&self, git_path: &Path, branch_name: &str) -> Result<(),std::io::Error> {
        let logs_path = git_path.join("logs");
        let commits_path = logs_path.join(branch_name);
        fs::create_dir_all(logs_path)?;
        fs::OpenOptions::new().create(true).append(true).open(commits_path)?;
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
        
        if !branch_path.exists() {
            let mut branch_file = OpenOptions::new()
                .write(true)
                .create(true)
                .append(false)
                .open(&branch_path)?;
    
            branch_file.write_all(COMMIT_INIT_HASH.as_bytes())?;
        }

        fs::create_dir_all(refs_path.join("tags"))?;
            
        Ok(())
    } 


    /// Crea el archivo config al inicializar un nuevo repositorio
    fn create_git_config_file(&self, git_path: &Path) -> Result<(),std::io::Error> {
        let config_path = git_path.join("config");
        let configuration = Config::read_config()?;
        if fs::File::open(&config_path).is_ok() {

        } else {
            let mut file = File::create(config_path)?;
            file.write_all("[core]\n".to_string().as_bytes())?;
            file.write_all("    repostiryformatversion = 0\n".to_string().as_bytes())?;
            file.write_all("    filemode = false\n".to_string().as_bytes())?;
            file.write_all("    bare = false\n".to_string().as_bytes())?;
            file.write_all("[user]\n".to_string().as_bytes())?;
            file.write_all(format!("    user.name= {}\n    user.email= {}",configuration.0,configuration.1).as_bytes())?;
        }

        Ok(())
    } 

    /// Crea el archivo HEAD al inicializar un nuevo repositorio
    fn create_head_file(&self, git_path: &Path, branch_name: &str) -> Result<(),std::io::Error> {
        let head_path = git_path.join("HEAD");

        if fs::File::open(&head_path).is_ok() {
            println!("warning: re-init: ignored --initial-branch={}", branch_name);
        } else {
            let mut file = File::create(head_path)?;
            file.write_all(format!("refs/heads/{}", branch_name).as_bytes())?;
        }            
        Ok(())
    }

    fn create_git_ignore(&self) -> Result<(),std::io::Error> {
        let current = CurrentRepository::read()?;
        let ignore_path = current.join(".gitignore");

        fs::OpenOptions::new().create(true).append(true).open(ignore_path)?;
        Ok(())
    }

    pub fn get_object_path(path: &Path) -> Result<PathBuf,std::io::Error>{
        let p = Path::new(path);
        let objects_path = p.join(RUST_PATH).join("objects");
        Ok(Path::new(&objects_path).to_path_buf())
    }

    pub fn get_current_log(path: &Path) -> Result<PathBuf,std::io::Error>{
        let branch = Self::get_current_branch(path)?;
        let log_path = path.join(".rust_git").join("logs").join(branch);
        Ok(log_path)
    }

    pub fn get_current_head(path: &Path) -> Result<PathBuf,std::io::Error>{
        let branch = Self::get_current_branch(path)?;
        let head_path = path.join(".rust_git").join("refs").join("heads").join(branch);
        Ok(head_path)
    }

    pub fn get_current_branch(path: &Path) -> Result<String, std::io::Error>{
        let p = Path::new(path);

        let head_path = p.join(".rust_git").join("HEAD");
        let mut head_file = File::open(head_path)?;
        
        let mut content = String::new();
        head_file.read_to_string(&mut content)?;
        if let Some(actual_branch) = content.clone().split('/').last(){
            return Ok(actual_branch.to_string());
        }
        Err(io::Error::new(io::ErrorKind::InvalidInput, "Can't find the branch"))
    }
    pub fn get_current_config(path: &Path) -> Result<PathBuf, std::io::Error>{
        let config_path = path.join(".rust_git").join("config");
        Ok(Path::new(&config_path).to_path_buf())
    }

}
