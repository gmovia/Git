use std::{net::TcpStream, io::{Read, Write}, str::from_utf8};

use crate::packfile::packfile::{read_packet, to_pkt_line, send_done_msg};

pub struct Clone;

impl Clone{
    pub fn git_clone(stream: &mut TcpStream) -> Result<(),std::io::Error> {
        Self::receive_pack(stream)?;
        Ok(())
    }
    
    pub fn receive_pack(socket: &mut TcpStream) -> Result<(), std::io::Error> {
        let mut packets = Vec::new();
        print!("Entro a receive packs ---------------\n");
        loop {
            let mut len_buf = [0; 4]; 
            if socket.read_exact(&mut len_buf).is_ok() {
                let len_str = from_utf8(&len_buf).unwrap();
                let len = usize::from_str_radix(len_str, 16).unwrap();
                if len == 0 {
                    break;
                }
                println!("ACA 1 PACKETTT ---> {:?} \n", len);

                let packet = read_packet(socket, len);
                println!("ACA PACKETTT ---> {:?} \n", packet);
                packets.push(packet);
            }
        }

        for packet in &packets {
            println!("Paquete: {:?}", packet);
        }
        for want in Self::get_want_msgs(&packets) {
            socket.write(want.as_bytes())?;
        }

        send_done_msg(socket)?;
        //let objects = Self::get_socket_response(socket)?;
        //println!("ANTES DE ENTRAR A REQUEST");
        //Self::request_branch(&packets, objects)?;
        Ok(()) 
    }

    fn get_want_msgs(commits_list: &Vec<String>) -> Vec<String> {
        let mut want_msgs = Vec::new();
    
        for commit in commits_list {
            let msg_commit = format!("want {}", commit);                
            let pkt_commit = to_pkt_line(&msg_commit);
            if commit.contains("HEAD"){
                continue;
            }
            want_msgs.push(pkt_commit);
        }
        println!("WANTS MESSAGE --> {:?} \n", want_msgs);
        want_msgs
    }


}