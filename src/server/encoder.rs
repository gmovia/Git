use std::fs::File;
use std::path::Path;
use std::{path::PathBuf, io, fs};

extern crate flate2;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::{Write, Read};

pub struct Encoder {
    pub path: PathBuf
}

impl Encoder {
    
    pub fn init_encoder(path: PathBuf) -> Result<Encoder,std::io::Error> {
        let encoder = Encoder { path: path };
        let packfile = Self::create_packfile(&encoder.path)?;
        println!("PACKFILE BYTES: {:?}", packfile);
        println!("PACKFILE STRING: {:?}", String::from_utf8_lossy(&packfile));
        Ok(encoder)
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
            //let object_type = Self::my_function(objects.1 as u8, objects.2);
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
        
        let mut second_byte = (object_len >> 4) & 0b0111111;
        
        if ((second_byte<<4)+less_significative_len_bits as usize) < object_len && second_byte > 0 {
            second_byte = second_byte + 128;
        }

        if second_byte > 0 {
            bytes.push(second_byte as u8);
        }

        let mut third_byte = (object_len >> 8) & 0b01111111;

        if (third_byte<<8+second_byte<<4+less_significative_len_bits as usize) < object_len && second_byte > 0 && third_byte > 0 {
            third_byte = third_byte + 128;
        }

        if third_byte > 0 {
            bytes.push(third_byte as u8);
        }

        Ok(bytes)
    }

    fn get_4_bits_less_significatives(number: usize) -> u8 {
        let mask: usize = 0b00001111;
        let retun = number & mask;
        retun as u8
    }

    fn create_header(mut packfile: &mut Vec<u8>, path: &PathBuf) -> Result<usize,std::io::Error>{
        for &byte in b"PACK" {
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