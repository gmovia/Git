use std::{path::Path, fs::{self, OpenOptions}, io::Write};

use crate::vcs::{files::current_commit::CurrentCommit, entities::tag_entity::TagEntity};



pub struct Tag;

pub enum TagOptions<'a> {
    Get,
    Create(&'a str, &'a str),
    CreateLight(&'a str),
    Delete(&'a str),
}

impl Tag {

    pub fn tag(path: &Path, option: TagOptions) -> Result<Vec<String>, std::io::Error> {
        match option {
            TagOptions::Get => {Self::get(path)},
            TagOptions::CreateLight(tag) => {Self::create_light_tag(path, tag)},
            TagOptions::Create(tag, message) => {Self::create_tag(path, tag, message)}
            TagOptions::Delete(tag) => {Self::delete(path, tag)},
        }
        
    }

    pub fn create_light_tag(path: &Path, tag: &str) -> Result<Vec<String>, std::io::Error>{
        let tags_path = path.join(".rust_git").join("refs").join("tags").join(tag);
        let mut tag_file = OpenOptions::new().write(true).create(true).append(true).open(tags_path)?;
        
        let commit_hash = CurrentCommit::read()?;
        tag_file.write_all(commit_hash.as_bytes())?;
        
        Self::get(path)
    }
    
    pub fn create_tag(path: &Path, tag: &str, message: &str) -> Result<Vec<String>, std::io::Error>{
        let commit_hash = CurrentCommit::read()?;
        let typef = "commit";
        let tagger = "tagger gmovia <gmovia@fi.uba.ar> 1700522965 -0300";
        
        let tag_entity = TagEntity{commit_hash: commit_hash, typef: typef.to_string(), tagger: tagger.to_string(), tag: tag.to_string(), message: message.to_string()};
        println!("{:?}",tag_entity);
        let hash_tag = TagEntity::write(path, tag_entity)?;
        println!("{:?}",hash_tag);
        let tags_path = path.join(".rust_git").join("refs").join("tags").join(tag);
        let mut tag_file = OpenOptions::new().write(true).create(true).append(true).open(tags_path)?;
        tag_file.write_all(hash_tag.as_bytes())?;

        Self::get(path)
    }
    
    pub fn delete(path: &Path, tag: &str) -> Result<Vec<String>, std::io::Error>{
        let tags_path = path.join(".rust_git").join("refs").join("tags").join(tag);
        let _ = fs::remove_file(tags_path);
        Self::get(path)
    }

    
    pub fn get(path: &Path) -> Result<Vec<String>, std::io::Error>{
        let mut tags = Vec::new();
        let tags_path = path.join(".rust_git").join("refs").join("tags");
        if let Ok(entries) = fs::read_dir(tags_path) {
            for tag_entry in entries.flatten() {
                if let Some(tag_name) = tag_entry.path().file_name() {
                    tags.push(tag_name.to_string_lossy().to_string());
                }
            }
        }
        Ok(tags)
    }
}