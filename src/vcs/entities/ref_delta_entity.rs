use std::{path::Path, fs::{OpenOptions, self, File}, io::Write};

use crate::{utils::randoms::random::Random, vcs::{commands::{hash_object::{HashObject, WriteOption}, init::Init}, version_control_system::VersionControlSystem}, constants::constant::OBJ_REF_DELTA_CODE};


pub enum DeltaOptions{
    Copy,
    Append,
}

#[derive(Debug, Clone)]
pub struct RefDeltaEntity{
    pub base_object_hash: String,
    pub data: Vec<u8>,
}

impl RefDeltaEntity{
    pub fn write(repo_path: &Path, ref_delta: RefDeltaEntity, action: DeltaOptions) -> Result<String, std::io::Error>{
        let delta_path = Path::new(&repo_path).join(Random::random());
        let mut delta_file = OpenOptions::new().write(true).create(true).append(true).open(&delta_path)?;
        
        let mut base_object_content = VersionControlSystem::cat_file(&ref_delta.base_object_hash)?;

        if ref_delta.data.len() == 4 {
            base_object_content.drain((ref_delta.data[3] as usize)..);
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
        base_object_content.drain((self.data[3] as usize)..);
        println!("BASE CORTADO 2: {}", base_object_content);
        let format = format!("{}{}",base_object_content, String::from_utf8_lossy(&self.data[5..]).to_string());
        println!("FORMAT: {}", format);
        delta_file.write_all(format.as_bytes())?;
        Ok(())
    }

    fn write_copy(&self, mut base_object_content: String, mut delta_file: File) -> Result<(),std::io::Error> {
        base_object_content.drain((self.data[5] as usize)..(self.data[7] as usize));
        println!("BASE CORTADO 2: {}", base_object_content);
        let format = format!("{}{}",base_object_content, String::from_utf8_lossy(&self.data[8..]).to_string());
        println!("FORMAT: {}", format);
        delta_file.write_all(format.as_bytes())?;
        Ok(())
    }

}