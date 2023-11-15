use std::{path::{PathBuf, Path}, fs::OpenOptions, collections::HashMap, io::{Write, self, BufRead}};
use chrono::{Local, DateTime};
use crate::{vcs::{entities::{commit_table_entry::CommitTableEntry, commit_entity::CommitEntity, tree_entity::TreeEntity, entity::convert_to_entities}, commands::init::Init}, utils::random::random::Random, constants::constants::COMMIT_INIT_HASH};
use super::current_repository::CurrentRepository;

#[derive(Debug, Clone)]
pub struct CommitsTable;

impl CommitsTable{

    /// Recibe el path del repositorio y una branch
    /// Devuelve la tabla de commits
    pub fn read(repo_path: PathBuf, branch: &str) -> Result<Vec<CommitTableEntry>, std::io::Error>{
        let mut commits: Vec<CommitTableEntry> = Vec::new();
        let path = repo_path.join(".rust_git").join("logs").join(Path::new(branch));
        let commits_file = OpenOptions::new().read(true).open(path)?;

        let reader = io::BufReader::new(commits_file);
        for line in reader.lines().filter_map(Result::ok) {
            let parts: Vec<&str> = line.split("-").collect();
            let commit = CommitTableEntry{id: parts[0].to_string(), last_hash: parts[1].to_string(), hash: parts[2].to_string(), message: parts[3].to_string(), date: parts[4].to_string()};
            commits.push(commit);
        }
        Ok(commits)
    } 

    /// Recibe un mensaje y el conjunto de blobs que va a almacenar el commits
    /// Escribe la tabla de commits, crea el tree y los blobs relacionados al commit
    pub fn write(message: &String, repository: &HashMap<String, String>) -> Result<(),std::io::Error>{
        let id = Random::random();
        let current_time: DateTime<Local> = Local::now();
        let _ = current_time.to_rfc2822();
        
        let current = CurrentRepository::read()?;

        let mut commits_file = OpenOptions::new().write(true).append(true).open(Init::get_commits_path(&current)?)?; //abro la tabla de commits para escribir - si no existe, la creo
        
        let entities = convert_to_entities(repository, &format!("{}/", &current.display().to_string()));

        let tree_hash = TreeEntity::write(&current, &entities)?;

        // LEER DE ALGUN ARCHIVO LA DATA!
        let author = "author gmovia <gmovia@fi.uba.ar> 1699842870 -0300";
        let committer = "committer gmovia <gmovia@fi.uba.ar> 1699842870 -0300";
        
        let current_repository = CurrentRepository::read()?;
        let commits = CommitsTable::read(current_repository.clone(), &Init::get_current_branch(&current_repository.clone())?)?;

        let parent_hash = if let Some(last_commit) = commits.last() {
            last_commit.hash.clone()
        } else {
            COMMIT_INIT_HASH.to_string()
        };

        let commit_entity =  CommitEntity{content_type: "commit".to_string(), tree_hash: tree_hash.clone(), parent_hash: parent_hash.clone(), author: author.to_string(), committer: committer.to_string(), message: message.clone()};
        let commit_hash = CommitEntity::write(&current, &commit_entity)?;
        let commit = format!("{}-{}-{}-{}-{}\n", id, parent_hash, commit_hash, message, current_time); 
        commits_file.write_all(commit.as_bytes())?;

        Ok(())
    }

    /// Recibe dos listados de commits 
    /// Devuelve el ultimo commit comun
    pub fn get_parent_commit(current_commits: &Vec<CommitTableEntry>, branch_commits: &Vec<CommitTableEntry>) ->  Option<CommitTableEntry>{
        let size = if current_commits.len() >= branch_commits.len() { branch_commits.len() } else { current_commits.len() };
        for index in 0..size{
            if current_commits[index].id == branch_commits[index].id{
                if index == size - 1{
                    return Some(current_commits[index].clone())
                }
                continue;
            }
            return Some(current_commits[index-1].clone());
        }
        None
    }
}