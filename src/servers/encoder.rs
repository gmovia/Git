use std::fs::File;
use tempdir::TempDir;
use std::path::Path;
use std::{path::PathBuf, io, fs};

extern crate flate2;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::{Read, Write};

pub struct Encoder {
    pub path: PathBuf
}

impl Encoder {
    
    pub fn init_encoder(path: PathBuf, messages: (Vec<String>,Vec<String>)) -> Result<Vec<u8>,std::io::Error> {
        let encoder = Encoder { path };
        let mut packfile= Vec::new();
        if messages.1.is_empty() || messages.1[0] == "0" {
            packfile = Self::create_packfile(&encoder.path)?;        
        }
        
        Ok(packfile)
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

    fn create_packfile(path: &Path) -> Result<Vec<u8>,std::io::Error> {
        let mut packfile: Vec<u8> = Vec::new();
        Self::create_header(&mut packfile, path)?;
        
        let mut objects_data: Vec<(String,usize,usize)> = Vec::new();
        Self::process_directory(&path.join(".rust_git").join("objects"), &mut objects_data)?;

        for objects in objects_data.iter().rev() {
            let object_type = Self::set_bits(objects.1 as u8, objects.2)?;
            for object in object_type {
                packfile.push(object);
            }
            let path = Path::new(&objects.0);
            
            let compress_data = Self::compress_object(path.to_path_buf(), objects.1)?;
            for byte in compress_data {
                packfile.push(byte);    
            }
        }
        Ok(packfile)
    }

    
    fn set_bits(object_type: u8, object_len: usize) -> Result<Vec<u8>, std::io::Error> {
        if object_type > 7 {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid object type"));
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

    fn create_header(packfile: &mut Vec<u8>, path: &Path) -> Result<usize,std::io::Error>{
        for &byte in b"0008NAK\nPACK" {
            packfile.push(byte);
        }
        Self::add_number_to_packfile(2, packfile);
        let objects = Self::get_objects_number(path)?;
        Self::add_number_to_packfile(objects as u32, packfile);
        Ok(objects)
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


    fn process_file(file_path: &PathBuf) -> Result<(String,usize,usize),std::io::Error> {
        let metadata = fs::metadata(file_path)?;
        let mut content = String::new();
        let mut file = fs::File::open(file_path)?;
        
        file.read_to_string(&mut content)?;

        if !(content.contains("100644") || content.contains("40000")) && content.contains("tree") {
            return Ok((file_path.to_string_lossy().to_string(), 1_usize,metadata.len() as usize))
        } else if content.contains("100644") || content.contains("40000"){
            return Ok((file_path.to_string_lossy().to_string(), 2_usize,metadata.len() as usize))
        }
        else {
            return Ok((file_path.to_string_lossy().to_string(),3_usize,metadata.len() as usize))
        }
    }
    
    fn process_directory(path: &PathBuf, objects_data: &mut Vec<(String,usize,usize)>) -> Result<Vec<(String,usize,usize)>, std::io::Error> {
        for entrada in fs::read_dir(path)? {
            let entrada = entrada?;
            let entry_path = entrada.path();
            if entry_path.is_file() {
                let data = Self::process_file(&entry_path)?;
                objects_data.push(data);
            }
            else {
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
                output.push_str(&format!("{} {} {}\n", elements[0], elements[3], elements[2]));
            }
        }
        output
    }


    fn compress_object(archivo_entrada: PathBuf, object_type: usize) -> Result<Vec<u8>, std::io::Error> {
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
}