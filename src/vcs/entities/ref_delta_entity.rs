use std::{
    fs::{self, File, OpenOptions},
    io::{self, Read, Write},
    path::Path,
};

use super::commit_entity::CommitEntity;
use crate::{
    constants::constant::{BLOB_CODE, COMMIT_CODE, TREE_CODE},
    proxies::proxy::Proxy,
    utils::randoms::random::Random,
    vcs::{
        commands::{
            hash_object::{HashObject, WriteOption},
            init::Init,
        },
        version_control_system::VersionControlSystem,
    },
};
use std::fmt::Write as FmtWrite;

pub enum DeltaOptions {
    Copy,
    Append,
}

#[derive(Debug, Clone)]
pub struct RefDeltaEntity {
    pub base_object_hash: String,
    pub data: Vec<u8>,
}

impl RefDeltaEntity {
    pub fn write(
        repo_path: &Path,
        ref_delta: RefDeltaEntity,
        blobs: &mut Vec<(u8, Vec<u8>)>,
    ) -> Result<Vec<(String, CommitEntity)>, std::io::Error> {
        let mut commit: Vec<(String, CommitEntity)> = Vec::new();
        let delta_path = Path::new(&repo_path).join(Random::random());
        let mut delta_file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&delta_path)?;

        let base_object_content = VersionControlSystem::cat_file(&ref_delta.base_object_hash)?;

        let delta_len = ref_delta.data.len();
        let mut position = ref_delta.set_initial_position()?;

        while position < delta_len {
            if Self::is_bit_set(ref_delta.data[position]) {
                position = ref_delta.copy_option(
                    position,
                    &mut delta_file,
                    &base_object_content,
                    blobs,
                    repo_path,
                    &delta_path,
                )?;
            } else {
                position = ref_delta.append_option(position, &mut delta_file)?;
            }
        }

        if base_object_content.contains("tree") && base_object_content.contains("author") {
            let ref_delta_hash = HashObject::hash_object(
                &delta_path,
                Init::get_object_path(repo_path)?,
                WriteOption::Write,
                COMMIT_CODE,
            )?;
            let commit_entity = CommitEntity::read(repo_path, &ref_delta_hash)?;
            commit.push((ref_delta_hash, commit_entity));
        } else if !base_object_content.contains("100644") && !base_object_content.contains("40000")
        {
            HashObject::hash_object(
                &delta_path,
                Init::get_object_path(repo_path)?,
                WriteOption::Write,
                BLOB_CODE,
            )?;
        }
        let _ = fs::remove_file(delta_path);
        Ok(commit)
    }

    fn is_bit_set(byte: u8) -> bool {
        let mask = 0b10000000;
        (byte & mask) == mask
    }

    fn append_option(
        &self,
        mut position: usize,
        delta_file: &mut File,
    ) -> Result<usize, std::io::Error> {
        let size = self.data[position];
        let bytes = &self.data[(position + 1_usize)..(position + 1 + (size as usize))];
        position += 1 + size as usize;
        delta_file.write_all(bytes)?;
        Ok(position)
    }

    fn set_initial_position(&self) -> Result<usize, std::io::Error> {
        let mut position = 0;
        while Self::is_bit_set(self.data[position]) {
            position += 1;
        }
        position += 1;

        while Self::is_bit_set(self.data[position]) {
            position += 1;
        }
        position += 1;
        Ok(position)
    }

    fn copy_option(
        &self,
        mut position: usize,
        delta_file: &mut File,
        base_object_content: &str,
        blobs: &mut Vec<(u8, Vec<u8>)>,
        repo_path: &Path,
        delta_path: &Path,
    ) -> Result<usize, std::io::Error> {
        let positions = Self::positions(&self.data[position..])?;
        let mut copy_content: &str = "";
        if base_object_content.contains("100644") || base_object_content.contains("40000") {
            for (iter, blob) in blobs.clone().iter().enumerate() {
                let new_blob = Self::process_tree_object(blob.0, &blob.1);
                let hash = Proxy::write_tree(repo_path, &new_blob.1)?;
                if self.base_object_hash == hash {
                    let content_bytes = &blob.1[..positions.1 as usize];
                    let content = Self::process_tree_object(blob.0, &content_bytes.to_vec());
                    Proxy::write_tree(repo_path, &content.1)?;
                    break;
                } else if iter == blobs.len() - 1 {
                    if let Some(text) =
                        Self::get_blob_content(base_object_content, positions.1 as usize)
                    {
                        copy_content = text;
                        delta_file.write_all(copy_content.as_bytes())?;
                        HashObject::hash_object(
                            delta_path,
                            Init::get_object_path(repo_path)?,
                            WriteOption::Write,
                            TREE_CODE,
                        )?;
                    } else {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "Error parsing delta blob",
                        ));
                    }
                }
            }
        } else {
            copy_content = &base_object_content[positions.0 as usize..positions.1 as usize];
        }
        delta_file.write_all(copy_content.as_bytes())?;
        position += positions.2;
        Ok(position)
    }

    fn process_tree_object(number: u8, inner_vec: &Vec<u8>) -> (u8, String) {
        let mut reader = inner_vec.as_slice();

        if let Ok(entries) = Self::read_tree_sha1(&mut reader) {
            let entry_string: String = entries
                .iter()
                .map(|(mode, name, sha1)| {
                    let hex_string: String = sha1.iter().fold(String::new(), |mut acc, byte| {
                        let _ = FmtWrite::write_fmt(&mut acc, format_args!("{:02x}", byte));
                        acc
                    });
                    format!("{}-  {}-{}", mode, name, hex_string)
                })
                .collect::<Vec<String>>()
                .join("\n");
            (number, entry_string)
        } else {
            eprintln!("Error decoding the tree object");
            (number, String::new())
        }
    }

    fn read_tree_sha1<R: Read>(reader: &mut R) -> io::Result<Vec<(String, String, Vec<u8>)>> {
        let mut entries = Vec::new();
        while let Ok(entry) = Self::read_tree_entry(reader) {
            entries.push(entry);
        }

        Ok(entries)
    }

    fn read_tree_entry<R: Read>(reader: &mut R) -> io::Result<(String, String, Vec<u8>)> {
        let mut mode_bytes = [0; 6];
        reader.read_exact(&mut mode_bytes)?;

        let binding = String::from_utf8_lossy(&mode_bytes[..]);
        let mode_str = binding.trim();
        let name = Self::read_cstring(reader)?;

        let mut sha1 = vec![0; 20];
        reader.read_exact(&mut sha1)?;

        Ok((mode_str.to_string(), name, sha1))
    }

    fn read_cstring<R: Read>(reader: &mut R) -> io::Result<String> {
        let mut buffer = Vec::new();
        loop {
            let mut byte = [0];
            reader.read_exact(&mut byte)?;
            if byte[0] == 0 {
                break;
            }
            buffer.push(byte[0]);
        }
        Ok(String::from_utf8_lossy(&buffer).to_string())
    }

    fn positions(bytes: &[u8]) -> Result<(u32, u32, usize), std::io::Error> {
        let initial_position: u32;
        let finish_position: u32;
        let mut bytes_used: usize = 0;
        let bits_position = Self::get_bits_positions(bytes[0]);
        if bits_position.0.is_empty() {
            initial_position = 0;
            let bytes_to_use = bits_position.1.len();
            let finish_bytes = &bytes[1..bytes_to_use + 1];
            bytes_used += bytes_to_use + 1;
            finish_position = Self::get_hexadecimal(bits_position.1, finish_bytes, 2)?;
        } else {
            let in_numeber_bytes_to_use = bits_position.0.len();
            let in_bytes_to_use = &bytes[1..in_numeber_bytes_to_use + 1];
            initial_position = Self::get_hexadecimal(bits_position.0, in_bytes_to_use, 1)?;
            let fin_numeber_bytes_to_use = bits_position.1.len();
            let fin_bytes_to_use = &bytes[(in_numeber_bytes_to_use + 1)
                ..in_numeber_bytes_to_use + fin_numeber_bytes_to_use + 1];
            bytes_used += in_numeber_bytes_to_use + fin_numeber_bytes_to_use + 1;
            finish_position =
                Self::get_hexadecimal(bits_position.1, fin_bytes_to_use, 2)? + initial_position;
        }
        Ok((initial_position, finish_position, bytes_used))
    }

    fn get_blob_content(text: &str, position: usize) -> Option<&str> {
        if let Some(mut index) = text[position..].find('\n') {
            index += position;
            let content = &text[..index + 1];
            Some(content)
        } else {
            None
        }
    }

    fn get_hexadecimal(
        positions: Vec<usize>,
        bytes: &[u8],
        option: u8,
    ) -> Result<u32, std::io::Error> {
        let hexa_number = if option == 2 {
            Self::size_bytes(positions, bytes)
        } else {
            Self::offset_bytes(positions, bytes)
        };

        if let Ok(decimal) = u32::from_str_radix(&hexa_number, 16) {
            Ok(decimal)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Cast delta hexa failed",
            ))
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
            } else if pos == 1 {
                if aux.contains('0') {
                    let number = format!("{:X}", bytes[1]);
                    hexa_number = format!("{}{}", number, hexa_number);
                } else {
                    let number = format!("{:X}", bytes[0]);
                    hexa_number = format!("{}00", number);
                }
                aux.push('1');
            } else if pos == 2 {
                if aux.contains('1') && !aux.contains('0') {
                    let number = format!("{:X}", bytes[1]);
                    hexa_number = format!("{}{}", number, hexa_number);
                } else if aux.contains('0') && !aux.contains('1') {
                    let number = format!("{:X}", bytes[1]);
                    hexa_number = format!("{}00{}", number, hexa_number);
                } else if aux.contains('1') && aux.contains('0') {
                    let number = format!("{:X}", bytes[2]);
                    hexa_number = format!("{}{}", number, hexa_number);
                } else {
                    let number = format!("{:X}", bytes[0]);
                    hexa_number = format!("{}0000", number);
                }
                aux.push('2')
            } else if aux.contains('0') && !aux.contains('1') && !aux.contains('2') {
                let number = format!("{:X}", bytes[1]);
                hexa_number = format!("{}0000{}", number, hexa_number);
            } else if aux.contains('1') && !aux.contains('0') && !aux.contains('2') {
                let number = format!("{:X}", bytes[1]);
                hexa_number = format!("{}00{}", number, hexa_number);
            } else if aux.contains('2') && !aux.contains('1') && !aux.contains('0') {
                let number = format!("{:X}", bytes[1]);
                hexa_number = format!("{}{}", number, hexa_number);
            } else if aux.contains('0') && aux.contains('1') && !aux.contains('2') {
                let number = format!("{:X}", bytes[2]);
                hexa_number = format!("{}00{}", number, hexa_number);
            } else if (aux.contains('0') && aux.contains('2') && !aux.contains('1'))
                || (aux.contains('2') && aux.contains('1') && !aux.contains('0'))
            {
                let number = format!("{:X}", bytes[2]);
                hexa_number = format!("{}{}", number, hexa_number);
            } else if aux.contains('0') && aux.contains('1') && aux.contains('2') {
                let number = format!("{:X}", bytes[3]);
                hexa_number = format!("{}{}", number, hexa_number);
            } else {
                let number = format!("{:X}", bytes[0]);
                hexa_number = format!("{}000000", number);
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
            } else if pos == 5 {
                if aux == 4 {
                    let number = format!("{:X}", bytes[1]);
                    hexa_number = format!("{}{}", number, hexa_number);
                    aux = 9;
                } else {
                    let number = format!("{:X}", bytes[0]);
                    hexa_number = format!("{}00", number);
                    aux = 5;
                }
            } else if aux == 5 {
                let number = format!("{:X}", bytes[1]);
                hexa_number = format!("{}{}", number, hexa_number);
            } else if aux == 4 {
                let number = format!("{:X}", bytes[1]);
                hexa_number = format!("{}00{}", number, hexa_number);
            } else if aux == 9 {
                let number = format!("{:X}", bytes[2]);
                hexa_number = format!("{}{}", number, hexa_number);
            } else {
                let number = format!("{:X}", bytes[0]);
                hexa_number = format!("{}0000", number);
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
