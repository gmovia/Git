use std::{path::PathBuf, io};



pub struct Encoder {
    pub path: PathBuf
}

impl Encoder {
    
    pub fn init_encoder(path: PathBuf) -> Result<Encoder,std::io::Error> {
        let encoder = Encoder { path: path };
        encoder.read_files();
        Ok(encoder)
    }

    fn read_files(&self) {
        println!("{}", self.path.to_string_lossy());
    }


}