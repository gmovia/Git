use std::{path::Path, fs::{self, File}};

pub struct Init {
    text: String,
}

impl Init {
    
    pub fn git_init() -> Init {
        let init = { Init { text: "hola".to_string() } };
        
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
        self.create_head_config_file(path)?;
        
        Ok(())
    }

    fn create_git_hooks_folder(&self, git_path: &Path) -> Result<(),std::io::Error> {
        let hooks_path = git_path.join("hooks");
        fs::create_dir(hooks_path)?;
        Ok(())
    }
    
    fn create_git_info_folder(&self, git_path: &Path) -> Result<(),std::io::Error> {
        let info_path = git_path.join("info");
        fs::create_dir(info_path)?;
        Ok(())
    }

    fn create_git_logs_folder(&self, git_path: &Path) -> Result<(),std::io::Error> {
        let logs_path = git_path.join("logs");
        fs::create_dir(logs_path)?;
        Ok(())
    }

    fn create_git_objects_folder(&self, git_path: &Path) -> Result<(),std::io::Error> {
        let objects_path = git_path.join("objects");
        fs::create_dir(objects_path)?;
        Ok(())
    }

    fn create_git_refs_folder(&self, git_path: &Path) -> Result<(),std::io::Error> {
        let refs_path = git_path.join("refs");
        fs::create_dir(&refs_path)?;
        fs::create_dir(&refs_path.join("heads"))?;
        fs::create_dir(&refs_path.join("tags"))?;
        Ok(())
    }   

    fn create_git_config_file(&self, git_path: &Path) -> Result<(),std::io::Error> {
        let config_path = git_path.join("config");
        File::create(config_path)?;
        Ok(())
    } 

    fn create_head_config_file(&self, git_path: &Path) -> Result<(),std::io::Error> {
        let head_path = git_path.join("HEAD");
        File::create(head_path)?;
        Ok(())
    }

}