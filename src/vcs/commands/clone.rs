use super::{
    cat_file::CatFile,
    init::Init,
    remote::{Remote, RemoteOption},
};
use crate::{
    constants::constant::{
        BLOB_CODE_NUMBER, COMMIT_CODE_NUMBER, COMMIT_INIT_HASH, OBJ_REF_DELTA_CODE_NUMBER,
        TAG_CODE_NUMBER, TREE_CODE_NUMBER,
    },
    packfiles::{
        packfile::{decompress_data, read_packet, send_done_msg, to_pkt_line},
        tag_file::{create_tag_files, create_tag_folder, exclude_tag_ref},
    },
    proxies::proxy::Proxy,
    utils::randoms::random::Random,
    vcs::{
        commands::{branch::Branch, checkout::Checkout},
        entities::{commit_entity::CommitEntity, ref_delta_entity::RefDeltaEntity},
        files::current_repository::CurrentRepository,
    },
};
use std::fmt::Write as FmtWrite;
use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::{self, Read, Write},
    net::TcpStream,
    path::Path,
    str::from_utf8,
};

/// Este struct implementa el comando clone de git.
pub struct Clone;

impl Clone {
    /// Esta funcion sirve como inicialiazdor del struct Clone.
    pub fn git_clone(stream: &mut TcpStream, repo: &Path, owner_repo: &Path) -> Result<(), std::io::Error> {
        Self::receive_pack(stream, repo, owner_repo)?;
        Ok(())
    }

    /// Esta funcion se encarga de llevar a cabo la logica central del comando. Recibe la respuesta del servidor al upload_pack, parsea y envia los mensajes want y have, recibe los objetos y delega la creacion de las diferentes carpetas a clonar.
    pub fn receive_pack(socket: &mut TcpStream, repo: &Path, owner_repo: &Path) -> Result<(), std::io::Error> {
        let mut packets = Vec::new();

        loop {
            let mut len_buf = [0; 4];
            if socket.read_exact(&mut len_buf).is_ok() {
                if let Ok(len_str) = from_utf8(&len_buf) {
                    if let Ok(len) = usize::from_str_radix(len_str, 16) {
                        if len == 0 {
                            break;
                        }

                        let packet = read_packet(socket, len);
                        if packet.contains("fatal error") {
                            return Err(std::io::Error::new(
                                std::io::ErrorKind::Other,
                                "fatal error: the path is not correct",
                            ));
                        }
                        packets.push(packet);
                    }
                }
            }
        }

        for want in Self::get_want_msgs(&packets) {
            let _ = socket.write_all(want.as_bytes());
        }

        send_done_msg(socket)?;
        let objects = Self::get_socket_response(socket)?;
        let (list_tags, list_refs) = exclude_tag_ref(packets.clone())?;

        let _ = create_tag_files(list_tags, &CurrentRepository::read()?);
        Self::init_commits(&list_refs, &objects, repo)?;
        Remote::remote(
            &CurrentRepository::read()?,
            RemoteOption::Add("origin", owner_repo.to_string_lossy().to_string().as_str()),
        )?;
        Ok(())
    }

    /// Esta funcion se encarga de recorrer todos los objetos recibidos por parte del servidor, procesarlos y crear todas las carpeta necesarias para llevar a cabo el desarrollo del comando correctamente.
    fn init_commits(
        list_refs: &Vec<String>,
        objects: &[(u8, Vec<u8>)],
        repo: &Path,
    ) -> Result<(), std::io::Error> {
        let mut branchs: HashMap<String, String> = HashMap::new();

        let objects_processed = Self::process_folder(objects.to_vec());

        let mut commits_created = Self::create_folders(objects_processed.clone(), repo);

        let delta_objects: Vec<(u8, Vec<u8>)> = objects
            .iter()
            .filter(|&&(first, _)| first == 7)
            .cloned()
            .collect();
        let mut blob_objects: Vec<(u8, Vec<u8>)> = objects
            .iter()
            .filter(|&&(first, _)| first == 2)
            .cloned()
            .collect();
        for (_, inner_vec) in delta_objects {
            if let Ok(commits) = Self::process_delta_object(&inner_vec, repo, &mut blob_objects) {
                if !commits.is_empty() {
                    for commit in commits {
                        commits_created.insert(commit.0, commit.1);
                    }
                }
            }
        }

        for item in list_refs {
            if item.contains("HEAD") {
                continue;
            }
            let parts: Vec<&str> = item.splitn(2, ' ').collect();
            if parts.len() == 2 {
                let commit = parts[0];
                let ref_part = parts[1];
                if ref_part.starts_with("refs/") {
                    let branch_name = ref_part.trim_start_matches("refs/heads/").to_string();
                    let _ = Branch::create_new_branch_with_hash(
                        repo,
                        branch_name.trim_end_matches('\n'),
                        commit,
                    );
                    branchs.insert(branch_name, commit.to_owned());
                }
            }
        }
        let _ = Self::write_commit_log(
            repo,
            branchs.clone(),
            &commits_created,
            objects_processed.clone(),
        );
        Checkout::update_cd(repo)?;
        Ok(())
    }

    /// Esta funcion se encarga de procesar los tipos de objetos que no son tree ni delta.
    fn process_non_tree_object(number: u8, inner_vec: &[u8]) -> (u8, String) {
        (number, String::from_utf8_lossy(inner_vec).to_string())
    }

    /// Esta funcion se encarga de procesar los objetos tree
    fn process_tree_object(number: u8, inner_vec: &Vec<u8>) -> (u8, String) {
        if std::str::from_utf8(inner_vec).is_ok() {
            let blobs: Vec<String> = String::from_utf8_lossy(inner_vec)
                .split('\n')
                .map(String::from)
                .collect();
            let mut string_to_send = String::new();
            for blob in &blobs {
                let blob_parts: Vec<&str> = blob.split(' ').collect();
                if blob_parts.len() == 3 {
                    let path = Path::new(blob_parts[1]);
                    if let Some(file_name) = path.file_name() {
                        string_to_send = format!(
                            "{}{}-   {}-{}\n",
                            string_to_send,
                            blob_parts[0],
                            file_name.to_string_lossy(),
                            blob_parts[2]
                        );
                    }
                }
            }
            (number, string_to_send)
        } else {
            let mut reader = inner_vec.as_slice();

            if let Ok(entries) = Self::read_tree_sha1(&mut reader) {
                let entry_string: String = entries
                    .iter()
                    .map(|(mode, name, sha1)| {
                        let hex_string: String =
                            sha1.iter().fold(String::new(), |mut acc, byte| {
                                write!(&mut acc, "{:02x}", byte)
                                    .expect("Failed to write to String");
                                acc
                            });
                        format!("{}-  {}-{}", mode, name, hex_string)
                    })
                    .collect::<Vec<String>>()
                    .join("\n");
                (number, entry_string)
            } else {
                eprintln!("Error decoding the tree object");
                (number, String::new())
            }
        }
    }

    /// Esta funcion se encarga de delegar el procesado de las carpetas, separandolas segun su tipo de objeto.
    pub fn process_folder(objects: Vec<(u8, Vec<u8>)>) -> Vec<(u8, String)> {
        let mut objects_processed: Vec<(u8, String)> = Vec::new();
        for (number, inner_vec) in &objects {
            if *number == OBJ_REF_DELTA_CODE_NUMBER {
            } else if *number == TREE_CODE_NUMBER {
                objects_processed.push(Self::process_tree_object(*number, inner_vec));
            } else {
                objects_processed.push(Self::process_non_tree_object(*number, inner_vec));
            }
        }
        objects_processed
    }

    /// Esta funcion se encarga de procesar los objetos delta.
    fn process_delta_object(
        inner_vec: &[u8],
        repo_path: &Path,
        blobs: &mut Vec<(u8, Vec<u8>)>,
    ) -> Result<Vec<(String, CommitEntity)>, std::io::Error> {
        let hash_base_object: String =
            (inner_vec[..20]).iter().fold(String::new(), |mut acc, b| {
                write!(&mut acc, "{:02x}", b).expect("Failed to write to String");
                acc
            });
        let decompres_data = &inner_vec[20..];

        let delta_entity = RefDeltaEntity {
            base_object_hash: hash_base_object.clone(),
            data: decompres_data.to_vec(),
        };
        let commit = Proxy::write_ref_delta(repo_path, delta_entity, blobs)?;
        Ok(commit)
    }

    /// Esta funcion se encarga de crear y delegar la creacion de las diferentes carpetas para cada tipo de objeto.
    pub fn create_folders(
        objects: Vec<(u8, String)>,
        repo: &Path,
    ) -> HashMap<String, CommitEntity> {
        let mut commits_created: HashMap<String, CommitEntity> = HashMap::new();

        for (index, content) in objects.iter() {
            match *index {
                COMMIT_CODE_NUMBER => match Self::create_commit_folder(content, repo) {
                    Ok((hash, commit_entity)) => {
                        commits_created.insert(hash.clone(), commit_entity);
                    }
                    Err(e) => {
                        println!("Error creating commit: {}", e);
                    }
                },
                TREE_CODE_NUMBER => {
                    if let Err(e) = Self::create_tree_folder(content, repo) {
                        println!("Error creating tree {}", e);
                    }
                }
                BLOB_CODE_NUMBER => Self::create_blob_folder(content, repo),
                TAG_CODE_NUMBER => {
                    if let Err(e) = create_tag_folder(content, repo) {
                        println!("Error creating tag {}", e);
                    }
                }
                OBJ_REF_DELTA_CODE_NUMBER => {}
                _ => println!("Type not identify {}", index),
            }
        }
        commits_created
    }

    /// Esta funcion se encarga de crear las carpetas de los objetos de tipo folder.
    fn create_commit_folder(
        content: &str,
        repo: &Path,
    ) -> Result<(String, CommitEntity), std::io::Error> {
        let partes: Vec<&str> = content.split('\n').collect();

        let commit_entity: CommitEntity = if !content.contains("parent") {
            CommitEntity {
                content_type: "commit".trim_end_matches('\n').to_string(),
                tree_hash: partes[0]
                    .trim_end_matches('\n')
                    .trim_start_matches("tree ")
                    .to_string(),
                message: partes[4..]
                    .join("\n")
                    .trim_start_matches('\n')
                    .trim_end_matches('\n')
                    .to_string(),
                author: partes[1]
                    .trim_end_matches('\n')
                    .trim_start_matches('\n')
                    .to_string(),
                committer: partes[2].trim_end_matches('\n').to_string(),
                parent_hash: COMMIT_INIT_HASH.to_string(),
            }
        } else {
            CommitEntity {
                content_type: "commit".to_string(),
                tree_hash: partes[0]
                    .trim_end_matches('\n')
                    .trim_start_matches("tree ")
                    .to_string(),
                message: partes[5..]
                    .join("\n")
                    .trim_start_matches('\n')
                    .trim_end_matches('\n')
                    .to_string(),
                author: partes[2]
                    .trim_end_matches('\n')
                    .trim_start_matches('\n')
                    .to_string(),
                committer: partes[3].trim_end_matches('\n').to_string(),
                parent_hash: partes[1]
                    .trim_end_matches('\n')
                    .trim_start_matches('\n')
                    .trim_start_matches("parent ")
                    .to_string(),
            }
        };
        let hash_commit = Proxy::write_commit(repo, &commit_entity)?;

        Ok((hash_commit, commit_entity))
    }

    /// Esta funcion se encarga de delegar la creacion de un objeto blob.
    fn create_blob_folder(content: &String, repo: &Path) {
        let _ = Proxy::write_blob(repo, content);
    }

    /// Esta funcion se encarga de delegar la creacion de un objeto tree.
    fn create_tree_folder(content: &str, repo: &Path) -> Result<String, std::io::Error> {
        Proxy::write_tree(repo, content)
    }

    /// Esta funcion se encarga de escribir y delegar la escritura de la tabla de commits
    fn write_commit_log(
        repo: &Path,
        branchs: HashMap<String, String>,
        commits_created: &HashMap<String, CommitEntity>,
        _objects: Vec<(u8, String)>,
    ) -> Result<(), std::io::Error> {
        for (branch_name, hash_commit_branch) in &branchs {
            if commits_created.contains_key(hash_commit_branch) {
                let _ = Self::complete_commit_table(
                    repo,
                    &branch_name.to_string(),
                    &hash_commit_branch.to_string(),
                    commits_created,
                );
            }
        }
        Ok(())
    }

    /// Esta funcion completa el desarrollo correspondiente a la escritura de la tabla de commits.
    fn complete_commit_table(
        repo: &Path,
        branch_name: &String,
        hash_commit_branch: &String,
        commits_created: &HashMap<String, CommitEntity>,
    ) -> Result<(), std::io::Error> {
        let logs_path = repo
            .join(".rust_git")
            .join("logs")
            .join(branch_name.trim_end_matches('\n'));
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(logs_path)?;

        let content = CatFile::cat_file(hash_commit_branch, Init::get_object_path(repo)?)?;
        let id = Random::random();

        if content.contains("parent") {
            let part: Vec<&str> = content.split('\n').collect();
            let hash_parent = part[1].trim_start_matches("parent ");
            if let Some(commit_entity) = commits_created.get(hash_commit_branch) {
                let date = Self::get_date(&commit_entity.author);
                let format_commit = format!(
                    "{}-{}-{}-{}-{}\n",
                    id, commit_entity.parent_hash, hash_commit_branch, commit_entity.message, date
                );
                let _ = Self::complete_commit_table(
                    repo,
                    branch_name,
                    &hash_parent.to_string(),
                    commits_created,
                );
                file.write_all(format_commit.as_bytes())?;
            }
        } else if let Some(commit_entity) = commits_created.get(hash_commit_branch) {
            let date = Self::get_date(&commit_entity.author);
            let format_commit = format!(
                "{}-{}-{}-{}-{}\n",
                id, commit_entity.parent_hash, hash_commit_branch, commit_entity.message, date
            );
            file.write_all(format_commit.as_bytes())?;
        }
        Ok(())
    }

    /// Esta funcion se encarga de optener la fecha escrita en una linea de la tabl de commits.
    pub fn get_date(line: &str) -> &str {
        let start = match line.find('>') {
            Some(pos) => pos + 2,
            None => 0,
        };
        &line[start..]
    }

    /// Esta funcion de encarga de obtener y parsear los mensajes want que luego seran enviados al servidor.
    fn get_want_msgs(commits_list: &Vec<String>) -> Vec<String> {
        let mut want_msgs = Vec::new();

        for commit in commits_list {
            let msg_commit = format!("want {}", commit);
            let pkt_commit = to_pkt_line(&msg_commit);
            if commit.contains("HEAD") {
                continue;
            }
            want_msgs.push(pkt_commit);
        }
        want_msgs
    }

    /// Esta funcion se ecarga de recibir el packfile enviado por el servidor con todos los objetos.
    pub fn get_socket_response(
        socket: &mut TcpStream,
    ) -> Result<Vec<(u8, Vec<u8>)>, std::io::Error> {
        let mut buffer = Vec::new();
        match socket.read_to_end(&mut buffer) {
            Ok(_) => Self::manage_pack(&buffer[8..]),
            Err(e) => Err(e),
        }
    }

    /// Este archivo se encarga de parsear y descomprimir la data que respecta a los objetos que recibimos del servidor.
    fn manage_pack(pack: &[u8]) -> Result<Vec<(u8, Vec<u8>)>, std::io::Error> {
        let object_number = Self::parse_number(&pack[8..12])?;
        let mut position: usize = 12;
        let mut objects = Vec::new();
        for _object in 0..object_number {
            let objet_type = Self::get_object_type(pack[position]);
            while Self::is_bit_set(pack[position]) {
                position += 1;
            }
            position += 1;
            if objet_type == 7 {
                let mut base_object = pack[position..position + 20].to_vec();
                position += 20;
                if let Ok(data) = decompress_data(&pack[position..]) {
                    position += data.1 as usize;
                    base_object.extend_from_slice(&data.0);
                    objects.push((objet_type, base_object));
                }
            } else if let Ok(data) = decompress_data(&pack[position..]) {
                position += data.1 as usize;
                objects.push((objet_type, data.0))
            }
        }
        objects.sort_by(|a, b| a.0.cmp(&b.0));

        Ok(objects)
    }

    /// Esta funcion nos ayuda a obtener un contenido legible en caso de tener un objeto tree.
    fn read_cstring<R: Read>(reader: &mut R) -> io::Result<String> {
        let mut buffer = Vec::new();
        loop {
            let mut byte = [0];
            reader.read_exact(&mut byte)?;
            if byte[0] == 0 {
                break;
            }
            buffer.push(byte[0]);
        }
        Ok(String::from_utf8_lossy(&buffer).to_string())
    }

    /// Esta funcion se encarga de obtener el contenido de forma legible para el humano (UTF-8)
    fn read_tree_entry<R: Read>(reader: &mut R) -> io::Result<(String, String, Vec<u8>)> {
        let mut mode_bytes = [0; 6];
        reader.read_exact(&mut mode_bytes)?;

        let binding = String::from_utf8_lossy(&mode_bytes[..]);
        let mode_str = binding.trim();
        let name = Self::read_cstring(reader)?;

        let mut sha1 = vec![0; 20];
        reader.read_exact(&mut sha1)?;

        Ok((mode_str.to_string(), name, sha1))
    }

    /// Esta funcion se encarga de retornarme un numero u8 al recibir un byte.
    fn parse_number(bytes: &[u8]) -> Result<u8, std::io::Error> {
        let texto: String = bytes.iter().map(|&b| b.to_string()).collect();
        match texto.parse() {
            Ok(numero) => Ok(numero),
            Err(_) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Can not parse number",
            )),
        }
    }

    /// Esta funcion nos ayuda a leer el contenido de un tree.
    fn read_tree_sha1<R: Read>(reader: &mut R) -> io::Result<Vec<(String, String, Vec<u8>)>> {
        let mut entries = Vec::new();
        while let Ok(entry) = Self::read_tree_entry(reader) {
            entries.push(entry);
        }

        Ok(entries)
    }

    /// Esta funcio nos retorna el tipo de objeto en concreto recibiendo un byte.
    fn get_object_type(bytes: u8) -> u8 {
        let mut bits = Vec::new();
        for i in (0..8).rev() {
            if i == 4 || i == 5 || i == 6 {
                let bit = (bytes >> i) & 1;
                bits.push(bit);
            }
        }
        let mut numero = 0;
        for bit in &bits {
            numero = (numero << 1) | bit;
        }
        numero
    }

    /// Esta funcion determina si el bit de la izquierda se encuentra en 1 o no.
    fn is_bit_set(byte: u8) -> bool {
        let mask = 0b10000000;
        (byte & mask) == mask
    }
}
