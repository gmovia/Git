use std::{net::TcpStream, io::{Read, Write, self, BufRead}, str::from_utf8, fs::File, path::Path, collections::HashMap};
use crate::{vcs::version_control_system::VersionControlSystem, packfile::packfile::{read_packet, to_pkt_line, decompress_data, send_done_msg}};

pub struct Fetch;

impl Fetch {

    pub fn fetch(socket: &mut TcpStream, vcs: &VersionControlSystem) -> Result<(), std::io::Error> {
        
        Self::get_upload_pack_response(socket, vcs)?;
        
        Ok(())
    }

    pub fn get_upload_pack_response(socket: &mut TcpStream, vcs: &VersionControlSystem) -> Result<(), std::io::Error> {
        let mut packets = Vec::new();
        loop {
            let mut len_buf = [0; 4]; 
            if socket.read_exact(&mut len_buf).is_ok() {
                let len_str = from_utf8(&len_buf).unwrap();
                let len = usize::from_str_radix(len_str, 16).unwrap();
                if len == 0 {
                    break;
                }
                let packet = read_packet(socket, len);
                packets.push(packet);
            }
        }

        for packet in &packets {
            println!("Paquete: {:?}", packet);
        }

        let branch_commits_received = Self::format_packet(&packets)?;
        println!("PASA ----> {:?}", branch_commits_received);
        let message_to_send =Self::packet_manager(branch_commits_received, vcs)?;
        println!("MENSAJESS: {:?}, {:?}", message_to_send.0, message_to_send.1);
        
        
        for want in Self::get_want_msgs(&packets) {
            socket.write(want.as_bytes())?;
        }

        send_done_msg(socket)?;
        let objects = Self::get_socket_response(socket)?;

        Ok(()) 
    }

    fn format_packet(packets: &Vec<String>) -> Result<Vec<(String,String)>,std::io::Error> {
        let mut branch_commit: Vec<(String,String)> = Vec::new();
        for packet in packets {
            let parts: Vec<&str> = packet.splitn(2, ' ').collect();
            branch_commit.push((parts[1].trim_start_matches("refs/head/").to_owned(), parts[0].to_owned()));
        }

        let mut last_entries: HashMap<&String, &String> = std::collections::HashMap::new();
        let mut last_commits: Vec<(String, String)> = Vec::new();
    
        for (key, value) in branch_commit.iter().rev() {
            if !last_entries.contains_key(&key) {
                last_entries.insert(key, value);
                last_commits.push((key.clone(), value.clone()));
            }
        }
        last_commits.reverse();
        Ok(last_commits)
    }
    
    fn packet_manager(last_branch_commit_recieve: Vec<(String,String)>, vcs: &VersionControlSystem) -> Result<(Vec<String>,Vec<String>), std::io::Error>{
        let mut want_list: Vec<String> = Vec::new();
        let mut have_list: Vec<String> = Vec::new();
        for packet in last_branch_commit_recieve {
            let file = File::open(&vcs.path.join(".rust_git").join("logs").join(packet.0))?;
            let reader = io::BufReader::new(file);

            let mut last_line = String::new();
            for line in reader.lines() {
                last_line = line?;
            }
            println!("last line: {}", last_line);
            if &last_line[2..42] == packet.1 {
                //have_list.push(format!("have {} refs/head/{}", packet.1, &packet.0));
            }
            else {
                //want_list.push(format!("want {} refs/head/{}", packet.1, &packet.0));
            }
        }
        Ok((want_list,have_list))
    }

    fn get_socket_response(socket: &mut TcpStream) -> Result<Vec<(u8,Vec<u8>)>,std::io::Error> {
        let mut buffer = Vec::new();
            match socket.read_to_end(&mut buffer) {
                Ok(_) => {
                    todo!()
                    //return Self::manage_pack(&buffer[8..]);
                }
                Err(e) => {
                    println!("Failed to receive data: {}\n", e);
                    return Err(e)
                }
            } 
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
        want_msgs
    }


}