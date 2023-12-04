use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

use crate::{
    constants::constant::TAG_CODE,
    utils::randoms::random::Random,
    vcs::{
        commands::{
            cat_file::CatFile,
            hash_object::{HashObject, WriteOption},
            init::Init,
        },
        files::config::Config,
    },
};

#[derive(Debug, Clone)]
pub struct TagEntity {
    pub commit_hash: String,
    pub typef: String,
    pub tag: String,
    pub tagger: String,
    pub message: String,
}

impl TagEntity {
    /// Recibe un tag y crea su correspondiente archivo 
    pub fn write(repo_path: &Path, tag: TagEntity) -> Result<String, std::io::Error> {
        let tag_path = Path::new(&repo_path).join(Random::random());
        let mut tag_file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&tag_path)?;
        let config = Config::read_config()?;
        tag_file.write_all(format!("object {}\n", tag.commit_hash).as_bytes())?;
        tag_file.write_all(format!("type {}\n", tag.typef).as_bytes())?;
        tag_file.write_all(format!("tag {}\n", tag.tag).as_bytes())?;
        tag_file.write_all(
            format!("tagger {} <{}> 1700522965 -0300\n", config.0, config.1).as_bytes(),
        )?;
        tag_file.write_all(format!("\n{}", tag.message).as_bytes())?;

        let tag_hash = HashObject::hash_object(
            &tag_path,
            Init::get_object_path(repo_path)?,
            WriteOption::Write,
            TAG_CODE,
        )?;

        let _ = fs::remove_file(tag_path);
        Ok(tag_hash)
    }

    /// Recibe el hash del tag y devuelve su correspondiente entidad
    pub fn read(repo_path: &Path, tag_hash: String) -> Result<TagEntity, std::io::Error> {
        let content = CatFile::cat_file(&tag_hash, Init::get_object_path(repo_path)?)?;

        let lines: Vec<&str> = content.split('\n').collect();

        let commit_hash: Vec<&str> = lines[0].split_whitespace().collect();
        let typef: Vec<&str> = lines[1].split_whitespace().collect();
        let tag: Vec<&str> = lines[2].split_whitespace().collect();
        let tagger = lines[3];
        let message = lines[5];

        Ok(TagEntity {
            commit_hash: commit_hash[1].to_string(),
            typef: typef[1].to_string(),
            tag: tag[1].to_string(),
            tagger: tagger.to_string(),
            message: message.to_string(),
        })
    }
}
