use std::{path::Path, fs::{OpenOptions, self, File}, io::Write};

use crate::{utils::randoms::random::Random, vcs::{commands::{hash_object::{HashObject, WriteOption}, init::Init}, version_control_system::VersionControlSystem}, constants::constant::OBJ_REF_DELTA_CODE};


pub enum DeltaOptions{
    Copy,
    Append,
}

#[derive(Debug, Clone)]
pub struct RefDeltaEntity{
    pub base_object_hash: String,
    pub data_to_chage: String,
    pub position_to_change: usize,
    pub stop_position: usize,
}

impl RefDeltaEntity{
    pub fn write(repo_path: &Path, ref_delta: RefDeltaEntity, action: DeltaOptions) -> Result<String, std::io::Error>{
        let delta_path = Path::new(&repo_path).join(Random::random());
        let mut delta_file = OpenOptions::new().write(true).create(true).append(true).open(&delta_path)?;
        
        let mut base_object_content = VersionControlSystem::cat_file(&ref_delta.base_object_hash)?;

        if ref_delta.data_to_chage == "".to_string() {
            base_object_content.drain(ref_delta.position_to_change..);
            delta_file.write_all(base_object_content.as_bytes())?;
        }
        else {
            match action {
                DeltaOptions::Copy => Self::write_copy(&ref_delta, base_object_content, delta_file)?,
                DeltaOptions::Append => Self::write_append(&ref_delta, base_object_content, delta_file)?,
            }
        }
        let ref_delta_hash = HashObject::hash_object(&delta_path, Init::get_object_path(repo_path)?, WriteOption::Write, OBJ_REF_DELTA_CODE)?;

        let _ = fs::remove_file(delta_path);
        Ok(ref_delta_hash)
    }

    fn write_append(&self, mut base_object_content: String, mut delta_file: File) -> Result<(),std::io::Error> {
        base_object_content.drain(self.position_to_change..);
        println!("BASE CORTADO 2: {}", base_object_content);
        let format = format!("{}{}",base_object_content, self.data_to_chage);
        println!("FORMAT: {}", format);
        delta_file.write_all(format.as_bytes())?;
        Ok(())
    }

    fn write_copy(&self, mut base_object_content: String, mut delta_file: File) -> Result<(),std::io::Error> {
        base_object_content.drain(self.position_to_change..self.stop_position);
        println!("BASE CORTADO 2: {}", base_object_content);
        let format = format!("{}{}",base_object_content, self.data_to_chage);
        println!("FORMAT: {}", format);
        delta_file.write_all(format.as_bytes())?;
        Ok(())
    }

}