use std::{path::Path, fs::{self, OpenOptions}, io::{Read, Write}};

use crate::{vcs::{commands::{cat_file::CatFile, init::Init}, entities::tag_entity::TagEntity}, packfiles::packfile::to_pkt_line};

pub fn get_tags(path: &Path) -> Result<Vec<String>, std::io::Error>{
    let mut log_entries = Vec::new();
    let tags_path = path.join(".rust_git").join("refs").join("tags");
    let entries_tag = fs::read_dir(tags_path)?;

    for entry in entries_tag{
        let tag_file: fs::DirEntry = entry?;
        let _ = fs::File::open(tag_file.path())?;
        if let Some(tag_name) = tag_file.path().file_name() {
            let tag_hash  = fs::read_to_string(tag_file.path())?;
            let is_comun = process_tag_content(tag_hash.clone(), path)?;
            let format_tag = if is_comun{
                format!("{} refs/tags/{}^{}", tag_hash, tag_name.to_string_lossy(), "{}")
            }else {
                format!("{} refs/tags/{}", tag_hash, tag_name.to_string_lossy())
            };
            log_entries.push(format_tag);
        }
    }
    Ok(log_entries)
}

pub fn process_tag_content(hash: String, repo_server_path:&Path) -> Result<bool, std::io::Error>{
    let content = CatFile::cat_file(&hash, Init::get_object_path(repo_server_path)?)?; // commit or tag
    if content.contains("tag"){
        return Ok(true);
    }
    Ok(false)
}

//Procesa el vector recibido, y devuelve solo las referencias de tags nuevas 
pub fn process_refs_old_new(refs : Vec<String>, path: &Path)-> Result<Vec<String>, std::io::Error>{
    //si es un comun debe agregarse para ser sumado como objeto mas
    //si no no suma, y solo se envia referencia sin {}
    let mut new_tags = Vec::new();
    let old_tags: Vec<String> = get_tags(path)?;
    
    for tags in refs{
        if !old_tags.contains(&tags){
            let format_tag = format!("want {}", tags);
            new_tags.push(to_pkt_line(&format_tag));
        }
    }
    Ok(new_tags)
}

//Procesa el vector recibido, y devuelve solo las referencias de tags nuevas 
pub fn process_refs_tag(refs : Vec<String>, path: &Path)-> Result<Vec<String>, std::io::Error>{
    //si es un comun debe agregarse para ser sumado como objeto mas
    //si no no suma, y solo se envia referencia sin {}
    let mut new_tags = Vec::new();
    let old_tags: Vec<String> = get_tags(path)?;
    
    for tags in old_tags{
        if !refs.contains(&tags){
            new_tags.push(tags)
        }
    }

    Ok(new_tags)
}

    pub fn process_tag_directory(path: &Path, objects_data: &mut Vec<(String,usize,usize)>, path_to_read: &Path, tag_to_send : String) -> Result<Vec<(String,usize,usize)>, std::io::Error> {
        //Aca agregar objetos tags
        for entrada in fs::read_dir(path)? {
            let entrada = entrada?;
            let entry_path = entrada.path();
            if entry_path.is_file() {
                println!("ENTRE a recorrer mi file\n");
                let data: (String, usize, usize) = process_tag_file(&entry_path, path_to_read, tag_to_send.clone())?;
                if data.1 != 0{
                    objects_data.push(data);
                }
            }
        }
        Ok(objects_data.to_vec())
    }

    //Encodea el objeto tag en formato (path, 4, size_paquete)
    //Recorre el directorio, de tags tiene que formatear a 4 solo los que sean objetos tags normal 
    pub fn process_tag_file(file_path: &Path, path_to_read: &Path, tag_to_send : String) -> Result<(String,usize,usize),std::io::Error> {
        let mut content_hash = String::new();
        let mut file = fs::File::open(file_path)?;

        file.read_to_string(&mut content_hash)?;   
        if tag_to_send.contains(&content_hash){
            let content = CatFile::cat_file(&content_hash, Init::get_object_path(path_to_read)?)?;
            if content.contains("tag"){
                let folder_name = content_hash.chars().take(2).collect::<String>();
                let object_path = Init::get_object_path(path_to_read)?;

                let final_path: std::path::PathBuf  = object_path.join(format!("{}/{}", folder_name, &content_hash[2..]).as_str());
                return Ok((final_path.to_string_lossy().to_string(), 4_usize, content.len()));
            }
        }
        Ok(("NONE".to_string(), 0, 0))   
    }
    
    pub fn create_tag_files(list_tags: Vec<String>, path: &Path) -> Result<(), std::io::Error>{
        println!("LIST TAGS ---> {:?}", list_tags);
        for string_tag in list_tags{
            let tag: Vec<&str> = string_tag.split_whitespace().collect();
            let filename:&str;
            let hash:&str;

            if tag[0].contains("want"){
                hash = tag[1];
                filename = tag[2].trim_end_matches("^{}");
            }else{
                hash = tag[0];
                filename = tag[1].trim_end_matches("^{}");
            }

            let file_path = path.join(".rust_git").join(filename);
            let mut file = OpenOptions::new().create(true).write(true).append(true).open(&file_path)?;
            println!("filepath ---> {:?}", file_path.clone());
            file.write_all(hash.as_bytes())?;
        } 
        Ok(())
    }


    pub fn create_tag_folder(content: &str, repo: &Path) -> Result<String, std::io::Error>{
        let content_lines: Vec<&str> = content.split('\n').collect();
        println!("CONTENt de create_tag_folder \n {:?}", content);
        let commit_hash: Vec<&str> = content_lines[0].split_whitespace().collect();
        let typed: Vec<&str> = content_lines[1].split_whitespace().collect();
        let tag: Vec<&str> = content_lines[2].split_whitespace().collect();
        let tagger: &str = content_lines[3];
        let message: &str = content_lines[5];

        let tag_entity = TagEntity{commit_hash:commit_hash[1].to_string(),typef:typed[1].to_string(), tag: tag[1].to_string(), tagger: tagger.to_string(), message: message.to_string()};
            
        TagEntity::write(repo, tag_entity)    
    }

    pub fn exclude_tag_ref(packets: Vec<String>) -> Result<(Vec<String>, Vec<String>), std::io::Error> {
        let mut to_remove: Vec<usize> = Vec::new();
        let mut list_tags: Vec<String> = Vec::new();
        
        for (index, refs) in packets.iter().enumerate() {
            if refs.contains("^{}") {
                // Crear folder de tag aquí para esa respectiva refs
            }
            if refs.contains("tags") {
                list_tags.push(refs.clone());
                to_remove.push(index);
            }
        }
        let remaining_packets: Vec<String> = packets
            .into_iter()
            .enumerate()
            .filter(|(index, _)| !to_remove.contains(index))
            .map(|(_, packet)| packet)
            .collect();
    
        Ok((list_tags, remaining_packets))
    }