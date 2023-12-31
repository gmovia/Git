use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    path::Path,
};

use crate::vcs::{
    entities::tag_entity::TagEntity,
    files::{config::Config, current_commit::CurrentCommit},
};

pub struct Tag;

pub enum TagOptions<'a> {
    Get,
    Create(&'a str, &'a str),
    CreateLight(&'a str),
    Delete(&'a str),
}

impl Tag {
    /// Comando tag.
    /// Recibe el current path y una option y matchea las opciones de tag
    pub fn tag(path: &Path, option: TagOptions) -> Result<Vec<String>, std::io::Error> {
        match option {
            TagOptions::Get => Self::get(path),
            TagOptions::CreateLight(tag) => Self::create_light_tag(path, tag),
            TagOptions::Create(tag, message) => Self::create_tag(path, tag, message),
            TagOptions::Delete(tag) => Self::delete(path, tag),
        }
    }

    /// Genera un tag ligero
    pub fn create_light_tag(path: &Path, tag: &str) -> Result<Vec<String>, std::io::Error> {
        let tags_path = path.join(".rust_git").join("refs").join("tags").join(tag);
        let mut tag_file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(tags_path)?;
        let commit_hash = CurrentCommit::read()?;
        tag_file.write_all(commit_hash.as_bytes())?;

        Self::get(path)
    }

    /// Genera un tag con etiqueta y mensaje
    pub fn create_tag(
        path: &Path,
        tag: &str,
        message: &str,
    ) -> Result<Vec<String>, std::io::Error> {
        let commit_hash = CurrentCommit::read()?;
        let config = Config::read_config()?;
        let typef = "commit";
        let format = format!("tagger {} <{}> 1700522965 -0300", config.0, config.1);
        let tagger = format.as_str();

        let tag_entity = TagEntity {
            commit_hash,
            typef: typef.to_string(),
            tagger: tagger.to_string(),
            tag: tag.to_string(),
            message: message.to_string(),
        };
        let hash_tag = TagEntity::write(path, tag_entity)?;
        let tags_path = path.join(".rust_git").join("refs").join("tags").join(tag);

        let mut tag_file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(tags_path)?;
        tag_file.write_all(hash_tag.as_bytes())?;

        Self::get(path)
    }

    /// Elimina un tag
    pub fn delete(path: &Path, tag: &str) -> Result<Vec<String>, std::io::Error> {
        if let Ok(tags) = Self::get(path) {
            if !tags.contains(&tag.to_string()) {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "Can't find the tag",
                ));
            }
        }
        let tags_path = path.join(".rust_git").join("refs").join("tags").join(tag);
        let _ = fs::remove_file(tags_path);
        Self::get(path)
    }

    /// Obtiene todos los tags generados
    pub fn get(path: &Path) -> Result<Vec<String>, std::io::Error> {
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
