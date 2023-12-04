use std::collections::HashSet;
use std::fs::{DirEntry, File};
use std::path::Path;
use std::{fs, io, path::PathBuf};
use tempdir::TempDir;

extern crate flate2;
use crate::constants::constant::COMMIT_INIT_HASH;
use crate::packfiles::tag_file::process_tag_directory;
use crate::vcs::commands::branch::Branch;
use crate::vcs::commands::cat_file::CatFile;
use crate::vcs::commands::init::Init;
use crate::vcs::entities::commit_entity::CommitEntity;
use crate::vcs::entities::entity::Entity;
use crate::vcs::entities::tree_entity::TreeEntity;
use crate::vcs::files::commits_table::CommitsTable;
use crate::vcs::files::current_repository::CurrentRepository;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::{Read, Write};

pub struct Encoder {
    pub path: PathBuf,
}

impl Encoder {
    pub fn init_encoder(
        path: &Path,
        messages: (Vec<String>, Vec<String>),
    ) -> Result<Vec<u8>, std::io::Error> {
        let encoder = Encoder {
            path: path.to_path_buf(),
        };
        if messages.1.is_empty() || messages.1[0] == "0" {
            Ok(Self::create_packfile(&encoder.path)?)
        } else {
            Ok(Self::create_fetch_packfile(&encoder.path, &messages)?)
        }
    }

    fn get_objects_number(path: &Path) -> Result<usize, std::io::Error> {
        let objects_path = path.join(".rust_git").join("objects");
        let mut total_files = 0;

        if let Ok(entries) = fs::read_dir(objects_path) {
            for entry in entries.flatten() {
                if entry.file_type()?.is_dir() {
                    if let Ok(subdir_entries) = fs::read_dir(entry.path()) {
                        let mut files = 0;
                        for subdir_entry in subdir_entries.flatten() {
                            if subdir_entry.file_type()?.is_file() {
                                files += 1;
                            }
                        }
                        total_files += files;
                    }
                }
            }
        }
        Ok(total_files)
    }

    pub fn process_tag_content(path: &Path, entry: DirEntry) -> Result<bool, std::io::Error> {
        let metadata = entry.metadata()?;
        let mut hash = String::new();

        if metadata.is_file() {
            let mut file: File = File::open(entry.path())?;
            file.read_to_string(&mut hash)?;
        }

        let content = CatFile::cat_file(&hash, Init::get_object_path(path)?)?;
        if content.contains("tag") {
            return Ok(true);
        }
        Ok(false)
    }

    fn create_packfile(path: &Path) -> Result<Vec<u8>, std::io::Error> {
        let mut packfile: Vec<u8> = Vec::new();
        Self::create_header(&mut packfile, path)?;

        let mut objects_data: Vec<(String, usize, usize)> = Vec::new();
        Self::process_directory(&path.join(".rust_git").join("objects"), &mut objects_data)?;

        for objects in objects_data.iter().rev() {
            let object_type = Self::set_bits(objects.1 as u8, objects.2)?;
            for object in object_type {
                packfile.push(object);
            }
            let path_object = Path::new(&objects.0);

            let compress_data = Self::compress_object(path_object, objects.1)?;
            for byte in compress_data {
                packfile.push(byte);
            }
        }
        Ok(packfile)
    }

    fn create_fetch_packfile(
        server_path: &Path,
        messages: &(Vec<String>, Vec<String>),
    ) -> Result<Vec<u8>, std::io::Error> {
        let mut packfile = Vec::new();

        let mut objects_data: Vec<(String, usize, usize)> = Vec::new();
        for want in &messages.0 {
            let parts: Vec<&str> = want.split(' ').collect();
            let commit_hash = parts[1];
            if !want.contains("tag") {
                if !Self::have_object(commit_hash, &messages.1) {
                    Self::fetch_process_directory(
                        server_path,
                        &mut objects_data,
                        commit_hash,
                        &messages.1,
                    )?;
                }
            } else {
                process_tag_directory(
                    &server_path.join(".rust_git").join("refs").join("tags"),
                    &mut objects_data,
                    server_path,
                    want.to_string(),
                )?;
            }
        }
        objects_data.sort_by(|a, b| a.1.cmp(&b.1));

        let mut unique_set = HashSet::new();

        let unique_objects_data: Vec<_> = objects_data
            .clone()
            .into_iter()
            .filter(|obj| unique_set.insert(obj.clone()))
            .collect();

        Self::create_size_header(&mut packfile, unique_objects_data.len())?;
        for objects in unique_objects_data.iter().rev() {
            let object_type = Self::set_bits(objects.1 as u8, objects.2)?;
            for object in object_type {
                packfile.push(object);
            }

            let path = Path::new(&objects.0);

            let compress_data = Self::compress_object(path, objects.1)?;
            for byte in compress_data {
                packfile.push(byte);
            }
        }

        Ok(packfile)
    }

    fn have_object(commit_hash: &str, haves: &Vec<String>) -> bool {
        for have in haves {
            if have.contains(commit_hash) {
                return true;
            }
        }
        false
    }

    fn fetch_process_directory(
        server_path: &Path,
        objects_data: &mut Vec<(String, usize, usize)>,
        commit_hash: &str,
        haves: &Vec<String>,
    ) -> Result<Vec<(String, usize, usize)>, std::io::Error> {
        let objects_path = server_path.join(".rust_git").join("objects");
        let want_path = objects_path.join(&commit_hash[..2]).join(&commit_hash[2..]);
        if want_path.exists() {
            let commit_entity = CommitEntity::read(server_path, commit_hash)?;
            if let Ok(metadata) = fs::metadata(&want_path) {
                objects_data.push((
                    want_path.to_string_lossy().to_string(),
                    1,
                    metadata.len() as usize,
                ));
                Self::process_fetch_tree(server_path, objects_data, commit_entity, haves)?;
            }
        }
        Ok(objects_data.to_vec())
    }

    fn process_fetch_tree(
        server_path: &Path,
        objects_data: &mut Vec<(String, usize, usize)>,
        commit_entity: CommitEntity,
        haves: &Vec<String>,
    ) -> Result<(), std::io::Error> {
        let tree_path = server_path
            .join(".rust_git")
            .join("objects")
            .join(&commit_entity.tree_hash[..2])
            .join(&commit_entity.tree_hash[2..]);
        let tree_entity = TreeEntity::read(server_path, commit_entity.tree_hash)?;
        if let Ok(metadata) = fs::metadata(&tree_path) {
            objects_data.push((
                tree_path.to_string_lossy().to_string(),
                2,
                metadata.len() as usize,
            ));
            Self::process_fetch_blobs(server_path, objects_data, tree_entity)?;
            if commit_entity.parent_hash != COMMIT_INIT_HASH
                && !Self::have_object(&commit_entity.parent_hash, haves)
            {
                Self::fetch_process_directory(
                    server_path,
                    objects_data,
                    &commit_entity.parent_hash,
                    haves,
                )?;
            }
        } else {
            std::io::Error::new(io::ErrorKind::NotFound, "Directory no found");
        }
        Ok(())
    }

    fn process_fetch_blobs(
        server_path: &Path,
        objects_data: &mut Vec<(String, usize, usize)>,
        entities: Vec<Entity>,
    ) -> Result<(), std::io::Error> {
        for entity in &entities {
            match entity {
                Entity::Blob(blob) => {
                    let blob_path = server_path
                        .join(".rust_git")
                        .join("objects")
                        .join(&blob.blob_hash[..2])
                        .join(&blob.blob_hash[2..]);
                    if let Ok(metadata) = fs::metadata(&blob_path) {
                        objects_data.push((
                            blob_path.to_string_lossy().to_string(),
                            3,
                            metadata.len() as usize,
                        ));
                    }
                }
                Entity::Tree(tree) => {
                    let tree_path = server_path
                        .join(".rust_git")
                        .join("objects")
                        .join(&tree.tree_hash[..2])
                        .join(&tree.tree_hash[2..]);
                    if let Ok(metadata) = fs::metadata(&tree_path) {
                        objects_data.push((
                            tree_path.to_string_lossy().to_string(),
                            2,
                            metadata.len() as usize,
                        ));
                    } else {
                        std::io::Error::new(io::ErrorKind::NotFound, "Directory no found");
                    }
                    Self::process_fetch_blobs(server_path, objects_data, tree.entities.clone())?;
                }
            };
        }
        Ok(())
    }

    pub fn set_bits(object_type: u8, object_len: usize) -> Result<Vec<u8>, std::io::Error> {
        if object_type > 7 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid object type",
            ));
        }

        let mut bytes = Vec::new();
        let resultado = object_type << 4;
        let mascara = 0b01110000;
        let res = resultado & mascara;
        let less_significative_len_bits = Self::get_4_bits_less_significatives(object_len);

        let mut first_byte = res + less_significative_len_bits;
        if (less_significative_len_bits as usize) < object_len {
            first_byte += 128;
        }
        bytes.push(first_byte);

        let mut remaining_len = object_len >> 4;

        while remaining_len > 0 {
            let mut next_byte = (remaining_len & 0b01111111) as u8;
            if remaining_len > 0b01111111 {
                next_byte |= 0b10000000;
            }
            bytes.push(next_byte);
            remaining_len >>= 7;
        }

        Ok(bytes)
    }

    fn get_4_bits_less_significatives(number: usize) -> u8 {
        let mask: usize = 0b00001111;
        let retun = number & mask;
        retun as u8
    }

    fn create_header(packfile: &mut Vec<u8>, path: &Path) -> Result<usize, std::io::Error> {
        for &byte in b"0008NAK\nPACK" {
            packfile.push(byte);
        }
        Self::add_number_to_packfile(2, packfile);
        let objects = Self::get_objects_number(path)?;
        Self::add_number_to_packfile(objects as u32, packfile);
        Ok(objects)
    }

    pub fn create_size_header(
        packfile: &mut Vec<u8>,
        objects: usize,
    ) -> Result<(), std::io::Error> {
        for &byte in b"0008NAK\nPACK" {
            packfile.push(byte);
        }
        Self::add_number_to_packfile(2, packfile);
        Self::add_number_to_packfile(objects as u32, packfile);
        Ok(())
    }

    fn add_number_to_packfile(number: u32, packfile: &mut Vec<u8>) {
        let number_str = number.to_string();
        let mut number_bytes = Vec::new();
        for digit in number_str.chars() {
            if let Some(digit_u8) = digit.to_digit(10) {
                number_bytes.push(digit_u8 as u8);
            }
        }
        while number_bytes.len() < 4 {
            number_bytes.insert(0, 0);
        }
        packfile.extend(number_bytes);
    }

    fn process_file(file_path: &Path) -> Result<(String, usize, usize), std::io::Error> {
        let metadata = fs::metadata(file_path)?;
        let mut content = String::new();
        let mut file = fs::File::open(file_path)?;

        file.read_to_string(&mut content)?;

        if !(content.contains("100644") || content.contains("40000")) && content.contains("tree") {
            return Ok((
                file_path.to_string_lossy().to_string(),
                1_usize,
                metadata.len() as usize,
            ));
        } else if content.contains("100644") || content.contains("40000") {
            return Ok((
                file_path.to_string_lossy().to_string(),
                2_usize,
                metadata.len() as usize,
            ));
        } else if content.contains("object") && content.contains("tag") {
            return Ok((
                file_path.to_string_lossy().to_string(),
                4_usize,
                content.len(),
            ));
        } else {
            return Ok((
                file_path.to_string_lossy().to_string(),
                3_usize,
                metadata.len() as usize,
            ));
        }
    }

    fn process_directory(
        path: &Path,
        objects_data: &mut Vec<(String, usize, usize)>,
    ) -> Result<Vec<(String, usize, usize)>, std::io::Error> {
        for entrada in fs::read_dir(path)? {
            let entrada = entrada?;
            let entry_path = entrada.path();
            if entry_path.is_file() {
                let data = Self::process_file(&entry_path)?;
                objects_data.push(data);
            } else {
                Self::process_directory(&entry_path, objects_data)?;
            }
        }
        Ok(objects_data.to_vec())
    }

    fn modify_entry_tree(input: &str) -> String {
        let mut output = String::new();
        for line in input.lines() {
            let elements: Vec<&str> = line.split_whitespace().collect();
            if elements.len() == 4 {
                output.push_str(&format!(
                    "{} {} {}\n",
                    elements[0], elements[3], elements[2]
                ));
            }
        }
        output
    }

    pub fn compress_object(
        archivo_entrada: &Path,
        object_type: usize,
    ) -> Result<Vec<u8>, std::io::Error> {
        let mut entrada = File::open(archivo_entrada)?;
        let temp_dir = TempDir::new("my_temp_dir")?;

        if object_type == 2 {
            let mut buf = String::new();
            let _ = entrada.read_to_string(&mut buf);

            buf = Self::modify_entry_tree(&buf.clone());

            let temp_dir = TempDir::new("my_temp_dir")?;
            let temp_file_path = temp_dir.path().join("temp_file.txt");
            let mut temp_file = File::create(&temp_file_path)?;
            temp_file.write_all(buf.as_bytes())?;
            entrada = File::open(&temp_file_path)?;
        }

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        io::copy(&mut entrada, &mut encoder)?;
        let datos_comprimidos = encoder.finish()?;
        let _ = fs::remove_file(temp_dir);

        Ok(datos_comprimidos)
    }

    pub fn get_object_for_commit(
        server_path: &Path,
        objects_data: &mut Vec<(String, usize, usize)>,
        commit_hash: &str,
        last_commit_server: &str,
    ) -> Result<Vec<(String, usize, usize)>, std::io::Error> {
        let objects_path = server_path.join(".rust_git").join("objects");
        let want_path = objects_path.join(&commit_hash[..2]).join(&commit_hash[2..]);

        if want_path.exists() {
            let commit_entity = CommitEntity::read(server_path, commit_hash)?;
            if let Ok(_metadata) = fs::metadata(&want_path) {
                Self::get_objects_tree(
                    server_path,
                    objects_data,
                    commit_entity,
                    last_commit_server,
                    &want_path,
                )?;
            }
        }
        objects_data.sort_by(|a, b| a.1.cmp(&b.1));

        let mut unique_set = HashSet::new();

        let unique_objects_data: Vec<_> = objects_data
            .clone()
            .into_iter()
            .filter(|obj| unique_set.insert(obj.clone()))
            .collect();
        Ok(unique_objects_data)
    }

    fn get_objects_tree(
        server_path: &Path,
        objects_data: &mut Vec<(String, usize, usize)>,
        commit_entity: CommitEntity,
        last_commit_server: &str,
        want_path: &Path,
    ) -> Result<(), std::io::Error> {
        let tree_path = server_path
            .join(".rust_git")
            .join("objects")
            .join(&commit_entity.tree_hash[..2])
            .join(&commit_entity.tree_hash[2..]);
        let tree_entity = TreeEntity::read(server_path, commit_entity.tree_hash)?;
        if let Ok(metadata) = fs::metadata(&tree_path) {
            if let Some(commit) = CommitsTable::get_commit(
                commit_entity.parent_hash.clone(),
                &Branch::get_current_branch(&CurrentRepository::read()?)?,
            )? {
                if commit.hash.as_str().trim_end_matches('\n')
                    == last_commit_server.trim_end_matches('\n')
                {
                    return Ok(());
                }
            }
            objects_data.push((
                tree_path.to_string_lossy().to_string(),
                2,
                metadata.len() as usize,
            ));
            Self::get_objects_blobs(server_path, objects_data, tree_entity)?;
            objects_data.push((
                want_path.to_string_lossy().to_string(),
                1,
                metadata.len() as usize,
            ));

            Self::get_object_for_commit(
                server_path,
                objects_data,
                &commit_entity.parent_hash,
                last_commit_server,
            )?;
        } else {
            std::io::Error::new(io::ErrorKind::NotFound, "Directory no found");
        }
        Ok(())
    }

    fn get_objects_blobs(
        server_path: &Path,
        objects_data: &mut Vec<(String, usize, usize)>,
        entities: Vec<Entity>,
    ) -> Result<(), std::io::Error> {
        for entity in &entities {
            match entity {
                Entity::Blob(blob) => {
                    let blob_path = server_path
                        .join(".rust_git")
                        .join("objects")
                        .join(&blob.blob_hash[..2])
                        .join(&blob.blob_hash[2..]);
                    if let Ok(metadata) = fs::metadata(&blob_path) {
                        objects_data.push((
                            blob_path.to_string_lossy().to_string(),
                            3,
                            metadata.len() as usize,
                        ));
                    }
                }
                Entity::Tree(tree) => {
                    let tree_path = server_path
                        .join(".rust_git")
                        .join("objects")
                        .join(&tree.tree_hash[..2])
                        .join(&tree.tree_hash[2..]);
                    if let Ok(metadata) = fs::metadata(&tree_path) {
                        objects_data.push((
                            tree_path.to_string_lossy().to_string(),
                            2,
                            metadata.len() as usize,
                        ));
                    } else {
                        std::io::Error::new(io::ErrorKind::NotFound, "Directory no found");
                    }
                    Self::get_objects_blobs(server_path, objects_data, tree.entities.clone())?;
                }
            };
        }
        Ok(())
    }
}
