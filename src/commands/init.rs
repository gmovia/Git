use std::{path::Path, fs::{self, File}};

/// Este Struct representa el comando git init. El cual se encarga de inicializar un repostorio.
pub struct Init {
    example_text: String,
}

impl Init {
    
    pub fn git_init() -> Init {
        let init = { Init { example_text: "hola".to_string() } };
        
        if let Err(e) = init.create_initial_folders() {
            println!("Error: {}",e);
        }        
        init
    }

    fn create_initial_folders(&self) -> Result<(),std::io::Error> {
        let path = Path::new(".git");
        fs::create_dir(path)?;

        self.create_git_hooks_folder(path)?;
        self.create_git_info_folder(path)?;
        self.create_git_logs_folder(path)?;
        self.create_git_objects_folder(path)?;
        self.create_git_refs_folder(path)?;
        self.create_git_config_file(path)?;
        self.create_head_file(path)?;
        
        Ok(())
    }

    /// Crea el directorio hooks al inicializar un nuevo repositorio
    fn create_git_hooks_folder(&self, git_path: &Path) -> Result<(),std::io::Error> {
        let hooks_path = git_path.join("hooks");
        fs::create_dir(hooks_path)?;
        Ok(())
    }
    
    /// Crea el directorio info al inicializar un nuevo repositorio
    fn create_git_info_folder(&self, git_path: &Path) -> Result<(),std::io::Error> {
        let info_path = git_path.join("info");
        fs::create_dir(info_path)?;
        Ok(())
    }

    /// Crea el directorio logs al inicializar un nuevo repositorio
    fn create_git_logs_folder(&self, git_path: &Path) -> Result<(),std::io::Error> {
        let logs_path = git_path.join("logs");
        fs::create_dir(logs_path)?;
        Ok(())
    }

    /// Crea el directorio objects al inicializar un nuevo repositorio
    fn create_git_objects_folder(&self, git_path: &Path) -> Result<(),std::io::Error> {
        let objects_path = git_path.join("objects");
        fs::create_dir(objects_path)?;
        Ok(())
    }

    /// Crea el directorio refs al inicializar un nuevo repositorio
    fn create_git_refs_folder(&self, git_path: &Path) -> Result<(),std::io::Error> {
        let refs_path = git_path.join("refs");
        fs::create_dir(&refs_path)?;
        fs::create_dir(&refs_path.join("heads"))?;
        fs::create_dir(&refs_path.join("tags"))?;
        Ok(())
    }   

    /// Crea el archivo config al inicializar un nuevo repositorio
    fn create_git_config_file(&self, git_path: &Path) -> Result<(),std::io::Error> {
        let config_path = git_path.join("config");
        File::create(config_path)?;
        Ok(())
    } 

    /// Crea el archivo HEAD al inicializar un nuevo repositorio
    fn create_head_file(&self, git_path: &Path) -> Result<(),std::io::Error> {
        let head_path = git_path.join("HEAD");
        File::create(head_path)?;
        Ok(())
    }

}