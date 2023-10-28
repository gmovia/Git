use flate2::read::ZlibDecoder;
use std::io::{Write, Read};


pub fn decompress_data(compressed_data: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    let mut decompressed_data = Vec::new();
    let mut decoder = ZlibDecoder::new(&compressed_data[..]);
    decoder.read_to_end(&mut decompressed_data)?;
    Ok(decompressed_data)
}


pub fn to_pkt_line(msg: &str) -> String {
    let len = msg.len() + 4; 
    let hex = format!("{:04x}", len); 
    hex + msg 
}