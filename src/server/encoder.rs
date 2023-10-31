use std::fs::File;
use std::{path::PathBuf, io, fs};

extern crate flate2;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::Write;

pub struct Encoder {
    pub path: PathBuf
}

impl Encoder {
    
    pub fn init_encoder(path: PathBuf) -> Result<Encoder,std::io::Error> {
        let encoder = Encoder { path: path };
        encoder.read_files();
        let packfile = Self::create_packfile(&encoder.path)?;
        println!("PACKFILE BYTES: {:?}", packfile);
        println!("PACKFILE STRING: {:?}", String::from_utf8_lossy(&packfile));
        Ok(encoder)
    }

    fn read_files(&self) {
        println!("{}", self.path.to_string_lossy());
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
        let objects_number = Self::create_header(&mut packfile, path)?;
        
        let object_type = Self::set_bits(3);
        packfile.push(object_type);
        println!("{:?}", format!("{:08b}", packfile.clone().pop().unwrap()));
        
        // ACA ESTOY AHORA - HAY QUE DECODIFICAR LOS FILES PARA SABER EL TAMAÑO Y GUARDARLO EN LOS BITS COMO EN EL PROTOCOLO
        for object in 0..objects_number {
            //let file_compresed = Self::compress_object(path)?;
            //println!("file {} compresed: {:?}", object, file_compresed);
            //println!("{}", object);
        } 
        //Self::compress();
        Ok(packfile)
    }

    fn set_bits(object_type: u8) -> u8 {
        if object_type > 7 {
            panic!("El número debe estar en el rango de 0 a 7");
        }
        let resultado = object_type << 4;
        let mascara = 0b01110000;
        resultado & mascara
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


    fn compress_object(archivo_entrada: &PathBuf) -> Result<Vec<u8>, std::io::Error> {
        let mut entrada = File::open(archivo_entrada)?;
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        io::copy(&mut entrada, &mut encoder)?;
        let datos_comprimidos = encoder.finish()?;
        
        Ok(datos_comprimidos)
    }



    fn compress() {
        let data = b"Esto es un ejemplo de datos que deseas comprimir.";
        let mut comprimido = Vec::new();
        let mut encoder = ZlibEncoder::new(comprimido, Compression::default());
        encoder.write_all(data).unwrap();
        let comprimido = encoder.finish().unwrap();
        println!("Datos comprimidos: {:?}", comprimido);
    }

}