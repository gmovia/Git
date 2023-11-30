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
        /* 
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
        */

        while Self::is_bit_set(ref_delta.data[position]) {
            position += 1;
        }
        position += 1;

        while Self::is_bit_set(ref_delta.data[position]) {
            position += 1;
        }
        position += 1;


        //println!("DATA - {:?}", ref_delta.data);
        //println!("BASE: {} - NEW: {}", base_size, new_size);
        while position < delta_len {
            println!("POSITION: {}, BYTE: {}", position, ref_delta.data[position]);
            if Self::is_bit_set(ref_delta.data[position]) {
                println!("\nCOPY\n");
                println!("BASE OBJECT: {}", ref_delta.base_object_hash);                
                let positions = Self::positions(&ref_delta.data[position..])?;

                println!("POSICIONES: {} - {}", positions.0, positions.1);
                let copy_content = &base_object_content[positions.0 as usize..positions.1 as usize];
                delta_file.write_all(copy_content.as_bytes())?;
                println!("BYTES USADOS: {}  (VIEJOS {})", positions.2, position);
                position += positions.2;
                println!("COPY: {}", copy_content);
            } 
            else {
                println!("\nAPPEND\n");
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

    fn get_positions(byte: u8) -> (u32, u32) {
        let bits_primer_grupo = (byte & 0b00001111).count_ones();
    
        let bits_segundo_grupo = (byte & 0b01110000).count_ones();
    
       (bits_primer_grupo, bits_segundo_grupo)
    }

    fn positions(bytes: &[u8]) -> Result<(u32,u32,usize), std::io::Error> {
        let initial_position: u32;
        let finish_position: u32;
        let mut bytes_used: usize  = 0;
        let bits_position = Self::get_bits_positions(bytes[0]);
        println!("BYTES POSITION: {:?}", bits_position);
        if (&bits_position.0).is_empty() {
            println!("ENTRA ACA");
            initial_position = 0;
            let bytes_to_use = bits_position.1.len();
            let finish_bytes = &bytes[1..bytes_to_use+1];
            bytes_used += bytes_to_use+1;
            finish_position = Self::get_hexadecimal(bits_position.1, finish_bytes, 2)?;
        } else {
            let in_numeber_bytes_to_use = (&bits_position.0).len();
            let in_bytes_to_use = &bytes[1..in_numeber_bytes_to_use+1];
            initial_position = Self::get_hexadecimal(bits_position.0, in_bytes_to_use, 1)?;
            let fin_numeber_bytes_to_use = bits_position.1.len();
            let fin_bytes_to_use = &bytes[(in_numeber_bytes_to_use+1)..in_numeber_bytes_to_use+fin_numeber_bytes_to_use+1];
            println!("IMPRIMI: {:?} - CANTIDAD DE BYTES: {}", fin_bytes_to_use, fin_numeber_bytes_to_use);
            bytes_used += in_numeber_bytes_to_use + fin_numeber_bytes_to_use + 1;
            finish_position = Self::get_hexadecimal(bits_position.1, fin_bytes_to_use, 2)?+initial_position;
        }

        Ok((initial_position,finish_position,bytes_used))
    }

    
    fn get_hexadecimal(mut positions: Vec<usize>, bytes: &[u8], option: u8) -> Result<u32, std::io::Error> {
        println!("POSITIONS: {:?}", positions); 
        println!("BYTES IN HEXA: {:?}", bytes);   
        let mut hexa_number = String::new();
        if option == 2 {
            hexa_number = Self::size_bytes(positions, bytes);
        }
        else {
            hexa_number = Self::offset_bytes(positions, bytes);
        }       
        if let Ok(decimal) = u32::from_str_radix(&hexa_number, 16) {
            return Ok(decimal);
        }
        else {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Cast delta hexa failed"));
        }
    }

    fn offset_bytes(mut positions: Vec<usize>, bytes: &[u8]) -> String {
        let mut aux = String::new();
        let mut hexa_number = String::new();
        positions.sort();
        for pos in positions {
            if pos == 0 {
                aux.push('0');
                hexa_number = format!("{:X}", bytes[0]);
            }
            else if pos == 1 {
                if aux.contains('0') {
                    let number = format!("{:X}", bytes[1]);
                    hexa_number = format!("{}{}",number,hexa_number);
                }
                else {
                    let number = format!("{:X}", bytes[0]);
                    hexa_number = format!("{}00",number);
                }
                aux.push('1');
            }
            else if pos == 2 {
                if aux.contains('1') && !aux.contains('0') {
                    let number = format!("{:X}", bytes[1]);
                    hexa_number = format!("{}{}",number,hexa_number);
                }
                else if aux.contains('0') && !aux.contains('1') {
                    let number = format!("{:X}", bytes[1]);
                    hexa_number = format!("{}00{}",number,hexa_number);
                }
                else if aux.contains('1') && aux.contains('0') {
                    let number = format!("{:X}", bytes[2]);
                    hexa_number = format!("{}{}",number,hexa_number);
                }
                else {
                    let number = format!("{:X}", bytes[0]);
                    hexa_number = format!("{}0000",number);
                }
                aux.push('2')
            }
            else {
                if aux.contains('0') && !aux.contains('1') && !aux.contains('2') {
                    let number = format!("{:X}", bytes[1]);
                    hexa_number = format!("{}0000{}",number, hexa_number);
                }   
                else if aux.contains('1') && !aux.contains('0') && !aux.contains('2') {
                    let number = format!("{:X}", bytes[1]);
                    hexa_number = format!("{}00{}",number, hexa_number);
                }                
                else if aux.contains('2') && !aux.contains('1') && !aux.contains('0') {
                    let number = format!("{:X}", bytes[1]);
                    hexa_number = format!("{}{}",number, hexa_number);
                }
                else if aux.contains('0') && aux.contains('1') && !aux.contains('2') {
                    let number = format!("{:X}", bytes[2]);
                    hexa_number = format!("{}00{}",number, hexa_number);
                }
                else if aux.contains('0') && aux.contains('2') && !aux.contains('1') {
                    let number = format!("{:X}", bytes[2]);
                    hexa_number = format!("{}{}",number, hexa_number);
                }
                else if aux.contains('2') && aux.contains('1') && !aux.contains('0') {
                    let number = format!("{:X}", bytes[2]);
                    hexa_number = format!("{}{}",number, hexa_number);
                }
                else if aux.contains('0') && aux.contains('1') && aux.contains('2') {
                    let number = format!("{:X}", bytes[3]);
                    hexa_number = format!("{}{}",number, hexa_number);
                }
                else {
                    let number = format!("{:X}", bytes[0]);
                    hexa_number = format!("{}000000",number);
                }
            }
        }
        hexa_number
    }


    fn size_bytes(mut positions: Vec<usize>, bytes: &[u8]) -> String {
        let mut aux = 0;
        let mut hexa_number = String::new();
        positions.sort();
        for pos in positions {
            if pos == 4 {
                aux = 4;
                hexa_number = format!("{:X}", bytes[0]);
            }
            else if pos == 5 {
                if aux == 4 {
                    let number = format!("{:X}", bytes[1]);
                    hexa_number = format!("{}{}",number,hexa_number);
                    aux = 9;
                }
                else {
                    let number = format!("{:X}", bytes[0]);
                    hexa_number = format!("{}00",number);
                    aux = 5;
                }
            }
            else {
                if aux == 5 {
                    let number = format!("{:X}", bytes[1]);
                    hexa_number = format!("{}{}",number,hexa_number);
                }
                else if aux == 4 {
                    let number = format!("{:X}", bytes[1]);
                    hexa_number = format!("{}00{}",number,hexa_number);
                }
                else if aux == 9 {
                    let number = format!("{:X}", bytes[2]);
                    hexa_number = format!("{}{}",number,hexa_number);
                }
                else {
                    let number = format!("{:X}", bytes[0]);
                    hexa_number = format!("{}0000",number);
                }
            }
        }
        hexa_number
    }

    fn get_bits_positions(byte: u8) -> (Vec<usize>, Vec<usize>) {
        let mut firsts_bits = Vec::new();
        let mut second_bits = Vec::new();
        for i in 0..8 {
            if (byte & 0b00001111) & (1 << i) != 0 {
                firsts_bits.push(i);
            }
            if (byte & 0b01110000) & (1 << i) != 0 {
                second_bits.push(i);
            }
        }
        (firsts_bits, second_bits)
    }

}