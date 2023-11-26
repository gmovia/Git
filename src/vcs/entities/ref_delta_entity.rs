use std::{path::Path, fs::{OpenOptions, self}, io::Write};

use crate::{utils::randoms::random::Random, vcs::{commands::{hash_object::{HashObject, WriteOption}, init::Init, cat_file::CatFile}, version_control_system::VersionControlSystem}, constants::constant::{TAG_CODE, OBJ_REF_DELTA_CODE}};

#[derive(Debug, Clone)]
pub struct RefDeltaEntity{
    pub base_object_hash: String,
    pub data_to_chage: String,
    pub position_to_change: usize,
}

impl RefDeltaEntity{
    pub fn write(repo_path: &Path, ref_delta: RefDeltaEntity) -> Result<String, std::io::Error>{
        let delta_path = Path::new(&repo_path).join(Random::random());
        let mut delta_file = OpenOptions::new().write(true).create(true).append(true).open(&delta_path)?;
        
        let mut base_object_content = VersionControlSystem::cat_file(&ref_delta.base_object_hash)?;

        if ref_delta.data_to_chage == "".to_string() {
            base_object_content.drain(ref_delta.position_to_change..);
            delta_file.write_all(base_object_content.as_bytes())?;
        }
        let ref_delta_hash = HashObject::hash_object(&delta_path, Init::get_object_path(repo_path)?, WriteOption::Write, OBJ_REF_DELTA_CODE)?;

        let _ = fs::remove_file(delta_path);
        Ok(ref_delta_hash)
    }

    pub fn read(repo_path: &Path, tag_hash: String) -> Result<RefDeltaEntity, std::io::Error>{
        let content = CatFile::cat_file(&tag_hash, Init::get_object_path(repo_path)?)?;

        let lines: Vec<&str> = content.split('\n').collect();

        let commit_hash: Vec<&str> = lines[0].split_whitespace().collect();
        let typef: Vec<&str> = lines[1].split_whitespace().collect();
        let tag: Vec<&str> = lines[2].split_whitespace().collect();
        let tagger = lines[3];
        let message = lines[5];

        return Ok(RefDeltaEntity { base_object_hash: "".to_string(), data_to_chage: "".to_string(), position_to_change: 1 })
    }
}