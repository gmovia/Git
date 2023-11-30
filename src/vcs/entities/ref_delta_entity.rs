use std::{path::Path, fs::{OpenOptions, self, File}, io::{Write, Read}};

use crate::{utils::randoms::random::Random, vcs::{commands::{hash_object::{HashObject, WriteOption}, init::Init}, version_control_system::VersionControlSystem}, constants::constant::{OBJ_REF_DELTA_CODE, TREE_CODE, COMMIT_CODE}};

use super::commit_entity::CommitEntity;


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
    
    pub fn write(repo_path: &Path, ref_delta: RefDeltaEntity) -> Result<Vec<(String, CommitEntity)>, std::io::Error>{
        let mut commit: Vec<(String, CommitEntity)> = Vec::new();
        let delta_path = Path::new(&repo_path).join(Random::random());
        let mut delta_file = OpenOptions::new().write(true).create(true).append(true).open(&delta_path)?;
        
        let base_object_content = VersionControlSystem::cat_file(&ref_delta.base_object_hash)?;

        let delta_len = ref_delta.data.len();
        let mut position: usize = 0;
        let base_size = ref_delta.data[0];
        position += 1;
        let new_size: u8;
        if ref_delta.data[1] == 1 {
            new_size = ref_delta.data[2];
            position += 3;
        }
        else {
            new_size = ref_delta.data[1];
            position += 1;
        }
        println!("DATA - {:?}", ref_delta.data);
        println!("BASE: {} - NEW: {}", base_size, new_size);
        while position < delta_len {
            println!("POSITION: {}, BYTE: {}", position, ref_delta.data[position]);
            if Self::is_bit_set(ref_delta.data[position]) {
                let bits = Self::contar_bits(ref_delta.data[position]);
                let initial_position: u8;
                let finish_position: u8;
                if bits.0 == 1 {
                    initial_position = ref_delta.data[position+1];
                    finish_position = ref_delta.data[position+2] + initial_position;
                    position += 3;
                } else {
                    initial_position = 0;
                    finish_position = ref_delta.data[position+1];
                    position += 2;
                }
                println!("INIT: {} - FINISH: {}", initial_position, finish_position);
                let copy_content = &base_object_content[initial_position as usize..finish_position as usize];
                delta_file.write_all(copy_content.as_bytes())?;
                println!("COPY: {}", copy_content);
            } 
            else {
                let size = ref_delta.data[position];
                let bytes = &ref_delta.data[(position+1 as usize)..(position+1+(size as usize))];
                position += 1 + size as usize;
                delta_file.write_all(bytes)?;
                println!("APPEND: {}", String::from_utf8_lossy(bytes))
            }
        }
        if base_object_content.contains("tree") && base_object_content.contains("author"){
            let ref_delta_hash = HashObject::hash_object(&delta_path, Init::get_object_path(repo_path)?, WriteOption::Write, COMMIT_CODE)?;
            let commit_entity = CommitEntity::read(repo_path, &ref_delta_hash)?;
            commit.push((ref_delta_hash, commit_entity));
        }
        else {
            HashObject::hash_object(&delta_path, Init::get_object_path(repo_path)?, WriteOption::Write, OBJ_REF_DELTA_CODE)?;    
        }
        let _ = fs::remove_file(delta_path);
        Ok(commit)
    }

    fn is_bit_set(byte: u8) -> bool {
        let mask = 0b10000000;
        (byte & mask) == mask
     }

     fn contar_bits(byte: u8) -> (u32, u32) {
        let bits_primer_grupo = (byte & 0b00001111).count_ones();
    
        let bits_segundo_grupo = (byte & 0b01110000).count_ones();
    
        (bits_primer_grupo, bits_segundo_grupo)
    }

}