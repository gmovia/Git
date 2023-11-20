use flate2::read::ZlibDecoder;
use std::{io::{Read, Write}, net::TcpStream, str::from_utf8};


pub fn decompress_data(compressed_data: &[u8]) -> Result<(Vec<u8>,u64), std::io::Error> {
    let mut decompressed_data = Vec::new();
    let mut decoder = ZlibDecoder::new(compressed_data);
    decoder.read_to_end(&mut decompressed_data)?;
    Ok((decompressed_data, decoder.total_in()))
}

pub fn to_pkt_line(msg: &str) -> String {
    let len = msg.len() + 4; 
    let hex = format!("{:04x}", len); 
    hex + msg 
}

pub fn read_packet(stream: &mut TcpStream, len: usize) -> String {
    if len == 0 {
        return "0".to_string();
    }
    let mut packet_buf = vec![0; len - 4];
    let _ = stream.read_exact(&mut packet_buf);
    String::from_utf8_lossy(&packet_buf).to_string()
}


/// Esta linea prcesa lo recibido por el servidor y lo devuelve.
pub fn process_line(stream: &mut TcpStream) -> Result<String, std::io::Error> {

    let mut result = String::new();
        let mut len_buf = [0; 4];
        if stream.read_exact(&mut len_buf).is_ok() {
            let len_str = from_utf8(&len_buf).unwrap();
            let len = usize::from_str_radix(len_str, 16).unwrap();
            let packet = read_packet(stream, len);
            result = packet;
        }
    
    Ok(result)
}

pub fn send_done_msg(socket: &mut TcpStream) -> Result<(), std::io::Error> {
    let msg_done = "0000";
    let _ = socket.write(msg_done.as_bytes());

    let msg_done2 = "0009done\n";
    let _ =socket.write(msg_done2.as_bytes());
    Ok(())
}