use crate::{
    vcs::files::vcs_file::VCSFile,
    utils::files::files::read,
    types::types::{ChangesNotStagedForCommit, ChangesToBeCommited, UntrackedFiles},
    vcs::{commands::{status::Status, add::Add, init::Init, hash_object::HashObject,cat_file::CatFile}, files::repository::Repository}, client::client::Client, constants::constants::{BDD_PATH, CURRENT_REPOSITORY_PATH},
};
use super::{commands::{hash_object::WriteOption, rm::{Rm, RemoveOption}, commit::Commit, log::Log, branch::{Branch, BranchOptions}, checkout::{Checkout, CheckoutOptions}, merge::Merge, reset::Reset}, entities::conflict::Conflict};
use std::{collections::HashMap, path::{Path, PathBuf}, fs::{OpenOptions, self}, io::{Write, self, BufRead}};
use super::files::index::Index;

#[derive(Debug, Clone)]
pub struct VersionControlSystem;

impl VersionControlSystem {


    pub fn write_bdd_of_repositories(path: &Path) -> Result<(), std::io::Error>{
        let repositories = Self::read_bdd_of_repositories()?;
        if !repositories.contains(&path.to_string_lossy().to_string()){
            let bdd_path = Path::new(BDD_PATH);
            let mut bdd = OpenOptions::new().write(true).append(true).open(bdd_path)?; 
            bdd.write_all(format!("{}\n",path.to_string_lossy()).as_bytes())?;
        }
        let current_repository_path = Path::new(CURRENT_REPOSITORY_PATH); // DEFINIR LA CONSTANTE = "current_repository.txt"
        let mut current_repository = OpenOptions::new().write(true).append(true).open(current_repository_path)?; 
        current_repository.set_len(0)?;
        current_repository.write_all(format!("{}",path.to_string_lossy()).as_bytes())?;

        Ok(())
    }

    pub fn read_bdd_of_repositories() -> Result<Vec<String>, std::io::Error>{
        let mut repositories = Vec::new();
        let bdd_path = Path::new(BDD_PATH);
        let repo_file = OpenOptions::new().read(true).open(&bdd_path)?;
        let reader = io::BufReader::new(repo_file);
        for line in reader.lines().filter_map(Result::ok) {
            repositories.push(line);
        }
        Ok(repositories)
    }

    pub fn read_current_repository() -> Result<PathBuf, std::io::Error>{ // current_repository.txt
        let current_path = Path::new(CURRENT_REPOSITORY_PATH);
        let repo_file = OpenOptions::new().read(true).open(&current_path)?;
        let reader = io::BufReader::new(repo_file);
        if let Some(current) = reader.lines().filter_map(Result::ok).last() {
            return Ok(Path::new(&current).to_path_buf());
        }
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "El archivo no existe",))
    }

    /// Inicializacion del versionControlSystem --> posee el repositorio local y la ruta de la carpeta a informar.
    pub fn init(path: &Path, args: Vec<String>){
        let _ = Self::write_bdd_of_repositories(path);
        let _ = Init::git_init(&path.to_path_buf(), args);
    }

    pub fn remove_repository(path: &Path) -> Result<(), std::io::Error>{
        let _ = fs::remove_dir_all(&path);
        let mut repositories = Self::read_bdd_of_repositories()?;
        if let Some(index) = repositories.iter().position(|item| item == &path.display().to_string()) {
            repositories.remove(index);
        }
        let bdd_path = Path::new(BDD_PATH);
        let mut bdd = OpenOptions::new().write(true).append(true).open(bdd_path)?; 
        bdd.set_len(0)?;
        for repo in repositories {
            bdd.write_all(format!("{}\n",repo).as_bytes())?;
        }
        Ok(())
    }

    /// Devuelve la informacion de los archivos creados, modificados y eliminados recientemente, junto con el area de staging.
    pub fn status() -> Result<(UntrackedFiles, ChangesNotStagedForCommit, ChangesToBeCommited), std::io::Error> {
        let current = Self::read_current_repository()?;
        println!("CURRENT {:?}",current);
        let files = read(&current)?;
        let staging_area = Index::read_index()?;
        println!("STAGING {:?}",staging_area);
        let repository = Repository::read_repository()?;
        println!("REPOSITORY {:?}",repository);
        Ok(Status::status(&files, &staging_area, &repository))
    }

    /// Recibe un path
    /// Agrega los archivos que se encuentran dentro del path al area de staging
    /// Devuelve el area de staging
    pub fn add(path: &Path) -> Result<HashMap<String, VCSFile>, std::io::Error> {
        Add::add(path)        
    }

    pub fn reset(path: &Path) -> Result<HashMap<String, VCSFile>, std::io::Error>{
        Reset::reset(path.to_path_buf())
    }

    /// Calcula el hash object de un archivo. En el caso de que sea una carpeta, debe devolver un error.
    /// Si se añade el comando -w lo que sucede es que se guardan los datos en .git/objects (investigar bien) FALTA HACER
    pub fn hash_object(path: &Path, option: WriteOption) -> Result<String, std::io::Error>{
        let current = Self::read_current_repository()?;
        let object_path = Init::get_object_path(&current, ".rust_git")?;
        HashObject::hash_object(path, object_path, option)
    }
 
    /// Recibe un hash
    /// Obtiene el path del hash y devuelve el contenido que hay en el archivo del path
    pub fn cat_file(hash: &str, path: &str) -> Result<String, std::io::Error>{
        let current = Self::read_current_repository()?;
        let object_path = Init::get_object_path(&current, path)?;
        CatFile::cat_file(hash, object_path)
    }

    /// Recibe un hash
    /// Obtiene el path del hash y devuelve el contenido que hay en el archivo del path en bytes
    pub fn cat_file_bytes(hash: &str, path: &str) -> Result<Vec<u8>, std::io::Error>{
        let current = Self::read_current_repository()?;
        let object_path = Init::get_object_path(&current, path)?;
        CatFile::cat_file_bytes(hash, object_path)
    }

    /// Recibe un path
    /// Elimina los archivos del workspace y repositorio local dado el path
    /// Si el comando tiene un -r se eliminan los archivos de un directorio entero
    pub fn rm(path: &Path, option: RemoveOption) -> Result<HashMap<String, VCSFile>, std::io::Error> {
        Rm::rm(path, option)
    }

    /// Recibe un mensaje
    /// Crea una entrada en la tabla de commits con su correspondiente id, hash del repositorio y mensaje.
    pub fn commit(message: String) -> Result<HashMap<String, String>, std::io::Error>{
        Commit::commit(message)
    }

    ///Muestra el historial de commits
    pub fn log() -> Result<String, std::io::Error> {
        Log::log()
    }

    pub fn git_clone(input: String) -> Result<(), std::io::Error> {
        let args: Vec<&str> = input.split_whitespace().collect();
        println!("{:?}", args);
        let _ = Self::write_bdd_of_repositories(Path::new("clone_here"));
        let _ = Client::client_(input.clone());
        let _ = Client::client_(format!("git fetch {}", args[2].to_owned()));
        Ok(())
    }

    pub fn fetch(input: String) -> Result<(), std::io::Error> {
        let _ = Client::client_(input);
        Ok(())
    }
    
    /// Recibe una opcion de branch (crear, borrar, listar)
    /// Segun la opcion, el branch permite crear una rama, borrar una ya existente o listar todas las ramas
    pub fn branch(option: BranchOptions) -> Result<Vec<String>, std::io::Error>{
        let current = Self::read_current_repository()?;
        Branch::branch(&current, option)
    } 

    pub fn get_branches() -> Result<Vec<String>, std::io::Error>{
        let current = Self::read_current_repository()?;
        Branch::get_branches(&current)
    } 

    /// Recibe una opcion de checkout (cambiar rama, crear y cambiar rama, analizar commit)
    /// Segun la opcion, el checkout actua
    pub fn checkout(option: CheckoutOptions) -> Result<(), std::io::Error>{
        let current = Self::read_current_repository()?;
        Checkout::checkout(&current, option)
    }

    pub fn merge(branch: &str) -> Result<HashMap<String, Conflict>,std::io::Error> {
        Merge::merge(branch, HashMap::new())
    }

    pub fn resolve_conflicts(branch: &str, conflicts: HashMap<String, Conflict>) -> Result<HashMap<String, Conflict>,std::io::Error> {
        Merge::merge(branch, conflicts)
    }
}