
use std::{net::TcpStream, path::{Path, PathBuf}, io::{Write, Read}, fs, collections::HashSet};
use crate::{packfiles::{packfile::{process_line, to_pkt_line}, tag_file::{process_refs_tag, process_tag_directory, process_tag_file}}, servers::encoder::Encoder, vcs::{commands::{init::Init, cat_file::CatFile}, files::current_commit::CurrentCommit}, constants::constant::COMMIT_INIT_HASH};

pub fn handle_send_pack(stream:  &mut TcpStream, current_repo: &Path, log_entries: &[String]) -> Result<(), std::io::Error> {
    println!("Entro a handle--- send--pack \n");
    let mut send_refs = Vec::new(); 
    loop {
        let value = process_line(stream);
        match value {
            Ok(value) => {
                if value == "0" {
                    break;
                } else {
                    send_refs.push(value);
                }                
            }
            Err(e) => {
                println!("Error al procesar la línea: {:?}", e);
                return Err(e);
            }
        }
    }
    println!("Mi lista que recibo de refs a enviar es:  --->{:?}\n" , send_refs);
    let last_commit_server = process_hash_server(&send_refs, (current_repo).to_path_buf())?; //handlear despues para mas ramas
    let last_commit_current = CurrentCommit::read()?;

    let send_new_tags = process_refs_tag(send_refs,current_repo )?;
    println!("SEND NEW TAGS ----< {:?}\n\n", send_new_tags);
    let packfile = init_packfile(last_commit_current, current_repo, &last_commit_server, &send_new_tags)?;

    send_pack(packfile, stream, log_entries, send_new_tags)?;

    let msg_done = "0000";
    stream.write_all(msg_done.as_bytes())?;
    Ok(())
}


fn process_hash_server(send_ref: &Vec<String>, current_repo: PathBuf) -> Result<String, std::io::Error>{
    let mut exist_branch_in_server = false;
    let mut last_commit_server= String::new();

    for refs in send_ref{
        if refs.contains(&Init::get_current_branch(&current_repo)?){
            exist_branch_in_server = true;
            let parts: Vec<&str>  = refs.split_ascii_whitespace().collect();
            last_commit_server = parts[0].to_string();
            break;
        }
    }
    if !exist_branch_in_server{
        last_commit_server = COMMIT_INIT_HASH.to_string();
    }
    Ok(last_commit_server)
}


fn init_packfile(last_commit_current: String, current_repo: &Path, last_commit_server: &str, send_new_tag: &Vec<String>) -> Result<Vec<u8>,std::io::Error>{
    let mut packfile: Vec<u8> = Vec::new();

    let mut objects_data: Vec<(String,usize,usize)> = Vec::new();
    println!("CURREN REPO ---> {:?}\n", current_repo);
    println!("LAS COMMIT ---> {}\n", last_commit_current);
    Encoder::get_object_for_commit(current_repo, &mut objects_data, &last_commit_current, last_commit_server)?;
    
    process_directory_to_send_new_tag(&current_repo.join(".rust_git").join("refs").join("tags"), &mut objects_data, send_new_tag.to_vec(), current_repo)?;

    println!("LEN OBJECTS DESPUESSSS {:?}\n", objects_data.len());
    println!("OBJECTS DATA: {:?}\n", objects_data);

    Encoder::create_size_header(&mut packfile, objects_data.len())?;

    for objects in objects_data.iter().rev() {
        let object_type = Encoder::set_bits(objects.1 as u8, objects.2)?;
        for object in object_type {
            packfile.push(object);
        }
        let path = Path::new(&objects.0);
        
        let compress_data = Encoder::compress_object(path.clone(), objects.1, current_repo)?;
        for byte in compress_data {
            packfile.push(byte);    
        }
    }
    Ok(packfile)
}
fn process_directory_to_send_new_tag(path: &Path, objects_data: &mut Vec<(String, usize, usize)>, send_new_tag: Vec<String>, current_repo: &Path) -> Result<Vec<(String, usize, usize)>, std::io::Error> {
    // Convert send_new_tag to HashSet for efficient membership testing
    let send_new_tag_set: HashSet<_> = send_new_tag.into_iter().map(|s| match s.split_whitespace().next() {
        Some(hash) => hash.to_string(),
        None => String::new(),
    }).collect();

    for entrada in fs::read_dir(path)? {
        let entrada = entrada?;
        let entry_path = entrada.path();
        if entry_path.is_file() {
            let hash_bytes = fs::read(&entry_path)?;

            // Convert hash_bytes to a String for comparison
            let hash = String::from_utf8_lossy(&hash_bytes).trim().to_string();

            if send_new_tag_set.contains(&hash) {
                // aca para el tag l etiene que llegar el path del object y no el que esta en refs/tag
                let data = process_particular_tag_to_send(&entry_path, current_repo)?;
                if data.1 != 0 {
                    objects_data.push(data);
                }
            }
        }
    }
    println!("process_tag_directory  TAG FOLDER\n");
    Ok(objects_data.to_vec())
}


fn process_particular_tag_to_send(file_path: &Path, path_to_read: &Path) -> Result<(String,usize,usize),std::io::Error> {

    let metadata = fs::metadata(file_path)?;
    let mut content_hash = String::new();
    let mut file = fs::File::open(file_path)?;

    file.read_to_string(&mut content_hash)?;    

    let content = CatFile::cat_file(&content_hash, Init::get_object_path(&path_to_read)?)?;

    if content.contains("tag"){
        println!("CONTIENE TAGGG tag_file\n");

        let folder_name = content_hash.chars().take(2).collect::<String>();
        let object_path = Init::get_object_path(path_to_read)?;

        let final_path  = object_path.join(format!("{}/{}", folder_name, &content_hash[2..]).as_str());
        return Ok((final_path.to_string_lossy().to_string(), 4_usize, content.len() as usize));
    }
    return Ok(("NONE".to_string(), 0, 0))
}


fn send_pack(packfile: Vec<u8>, stream: &mut TcpStream, log_entries: &[String], send_new_tag: Vec<String>) -> Result<String, std::io::Error> {
    let entry_hash = format!("{}\n", log_entries[0]); //esto manda pos [0] pq siempre mandas cambio de la rama en la que estas parado.
    println!("LOG ENTIRES ---> {:?}", entry_hash);
    stream.write_all(to_pkt_line(&entry_hash).as_bytes())?;
    println!("el mensaje de old y new antes del packfile --> {}\n", to_pkt_line(&entry_hash));

    for tag in send_new_tag{
        let tag_to_pkt_line = to_pkt_line(&tag);
        println!("Mi pedido de los tags al cliente es: {:?}\n\n", tag_to_pkt_line);
        stream.write_all(tag_to_pkt_line.as_bytes())?;
    }

    let msg_done = "0000";
    stream.write_all(msg_done.as_bytes())?;

    stream.write_all(&packfile)?;
    println!("PAQUETE ENVIADO CON EXITO\n");
    Ok("0000".to_string())
}
