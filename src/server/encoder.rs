use std::fs::File;
use std::path::Path;
use std::{path::PathBuf, io, fs};

extern crate flate2;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::{Read, BufRead};
use crate::vcs::commands::cat_file::CatFile;
use crate::vcs::version_control_system::VersionControlSystem;

pub struct Encoder {
    pub path: PathBuf
}

impl Encoder {
    
    pub fn init_encoder(path: PathBuf, messages: (Vec<String>,Vec<String>)) -> Result<Vec<u8>,std::io::Error> {
        let encoder = Encoder { path: path };
        let mut packfile= Vec::new();
        if messages.1.is_empty() || messages.1[0] == "0" {
            packfile = Self::create_packfile(&encoder.path)?;        
        }
        else {
            packfile = Self::create_fetch_packfile(&encoder.path, &messages)?;
        }
        Ok(packfile)
    }
    


    fn get_objects_number(path: &PathBuf) -> Result<usize, std::io::Error> {
        let objects_path = path.join(".rust_git").join("objects");
        let mut objects = 0;
        let entries = fs::read_dir(objects_path)?;
    
        for entry in entries {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                objects += 1;
            }
        }
        Ok(objects)
    }

    fn create_packfile(path: &PathBuf) -> Result<Vec<u8>,std::io::Error> {
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
            let compress_data = Self::compress_object((&path).to_path_buf())?;
            for byte in compress_data {
                packfile.push(byte);    
            }
        }
        Ok(packfile)
    }


    fn create_fetch_packfile(path: &PathBuf, messages: &(Vec<String>,Vec<String>)) -> Result<Vec<u8>,std::io::Error> {
        let mut packfile = Vec::new();
        Self::create_header(&mut packfile, path)?;        
        let current = VersionControlSystem::read_current_repository()?;
        let formatted_path = format!("test_folder/{}", current.display().to_string()); // Puede ser aca el error?
        let path_server = Path::new(&formatted_path);

        
        let mut objects_to_send: Vec<(String,String)> = Vec::new();
        objects_to_send = Self::process_logs(&path.join(".rust_git").join("logs"), &messages, &mut objects_to_send)?;
        println!("DATA TO SEND: {:?}", objects_to_send);


        let mut objects_to_encode: Vec<(String,usize,usize)> = Vec::new();
        for objects in objects_to_send {
            let path = format!("{}/.rust_git/objects", path_server.display());
            let content = CatFile::cat_file(&objects.1, (&path).into())?;
            let tree_path = format!("{}/{}/{}",path,  &objects.1[0..2], &objects.1[2..]);
            objects_to_encode.push((tree_path, 2, content.len() as usize));
            Self::get_blobs(content, &mut objects_to_encode, path_server)?;
        }
        println!("OBJECTS TO ENCODE: {:?}", objects_to_encode);

        for objects in objects_to_encode.iter().rev() {
            let object_type = Self::set_bits(objects.1 as u8, objects.2)?;
            for object in object_type {
                packfile.push(object);
            }
            let path = Path::new(&objects.0);
            let compress_data = Self::compress_object((&path).to_path_buf())?;
            for byte in compress_data {
                packfile.push(byte);    
            }
        }

        Ok(packfile)
    }

    fn get_blobs(content: String, objects_to_encode: &mut Vec<(String,usize,usize)>, path: &Path) -> Result<(),std::io::Error> {
        let mut blobs: Vec<&str> = content.split("\n").collect();
        blobs.pop();
        for blob in blobs {
            let hash_blob: Vec<&str> = blob.split("-").collect();
            let path_server = format!("{}/.rust_git/objects", path.display());
            let blob_content = CatFile::cat_file(&hash_blob[1], (&path_server).into())?;
            let blob_path = format!("{}/{}/{}",path_server,  &hash_blob[1][0..2], &hash_blob[1][2..]);
            objects_to_encode.push((blob_path,3,blob_content.len() as usize));
        }
        Ok(())
    }

    fn process_logs(path: &PathBuf, messages: &(Vec<String>,Vec<String>), object_to_send_list: &mut Vec<(String, String)>) -> Result<Vec<(String,String)>, std::io::Error> {
        let mut data_to_send = Vec::new();
        for entrada in fs::read_dir(path)? {
            let entrada = entrada?;
            let entry_path = entrada.path();
            if entry_path.is_file() {
                data_to_send = Self::process_log_file(&entry_path, &messages, object_to_send_list)?;
            }
        }
        Ok(data_to_send)        
    }

    fn process_log_file(file_path: &PathBuf, messages: &(Vec<String>,Vec<String>), object_to_send_list: &mut Vec<(String, String)>) -> Result<Vec<(String,String)>,std::io::Error> {
        println!("PROCESS LOG FILE: {:?}",file_path);
        println!("{:?}-{:?}", &messages.0,&messages.1);

        if let Some(branch) = file_path.file_name() {
            let branch_as_string: String = branch.to_string_lossy().to_string();
            let file = fs::File::open(file_path)?;
            
            let reader = io::BufReader::new(file);
            let mut last_line = String::new();
            for line in reader.lines() {
                last_line = line?;
            }
            
            let content_vec: Vec<&str> = last_line.split('-').collect();
            println!("CONTENT VEC: {:?}", content_vec);
            for message in &messages.0 {   
                println!("ACA: {}-{}-{:?}", message, branch_as_string,&content_vec);
                println!("COND 1: {}", message.contains(&branch_as_string));
                println!("COND 2: {}", message.contains(&content_vec[1]));
                if message.contains(&branch_as_string) && message.contains(&content_vec[1]) {
                    object_to_send_list.push((file_path.to_string_lossy().to_string(), content_vec[1].to_owned()));
                    println!("PUSH: {:?}", object_to_send_list);
                }
                else {
                    println!("NO ENTRO");
                }
            }
            println!("VECTOR: {:?}", object_to_send_list);
        } else {
            return Err(std::io::Error::new(io::ErrorKind::NotFound, "Branch name not found"));
        }
        println!("OBJECT TO SEND: {:?}", object_to_send_list);
        Ok(object_to_send_list.to_vec())
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
            first_byte = 128 + first_byte;
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

    fn create_header(mut packfile: &mut Vec<u8>, path: &PathBuf) -> Result<usize,std::io::Error>{
        for &byte in b"0008NAK\nPACK" {
            packfile.push(byte);
        }
        Self::add_number_to_packfile(2, &mut packfile);
        let objects = Self::get_objects_number(path)?;
        Self::add_number_to_packfile(objects as u32, &mut packfile);
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


        if content.contains("tree") {
            return Ok((file_path.to_string_lossy().to_string(),1 as usize,metadata.len() as usize))
        } else if content.contains(".txt-"){
            return Ok((file_path.to_string_lossy().to_string(),2 as usize,metadata.len() as usize))
        }
        else {
            return Ok((file_path.to_string_lossy().to_string(),3 as usize,metadata.len() as usize))
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

    fn compress_object(archivo_entrada: PathBuf) -> Result<Vec<u8>, std::io::Error> {
        let mut entrada = File::open(archivo_entrada)?;
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        io::copy(&mut entrada, &mut encoder)?;
        let datos_comprimidos = encoder.finish()?;
        
        Ok(datos_comprimidos)
    }

}