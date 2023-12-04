use crate::{
    constants::constant::COMMIT_INIT_HASH,
    packfiles::{
        packfile::{process_line, to_pkt_line},
        tag_file::process_refs_tag,
    },
    servers::encoder::Encoder,
    vcs::{
        commands::{cat_file::CatFile, init::Init},
        files::current_commit::CurrentCommit,
    },
};
use std::{
    collections::HashSet,
    fs,
    io::{Read, Write},
    net::TcpStream,
    path::{Path, PathBuf},
};

/// Maneja el envío del paquete al servidor, que incluye el procesamiento de las referencias y la creación del packfile.
/// Recibe un mutable TcpStream `stream` para la comunicación, la ruta del repositorio actual `current_repo`,
/// y un slice de entradas de registro `log_entries`.
pub fn handle_send_pack(
    stream: &mut TcpStream,
    current_repo: &Path,
    log_entries: &[String],
) -> Result<(), std::io::Error> {
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
                return Err(e);
            }
        }
    }
    let last_commit_server = process_hash_server(&send_refs, (current_repo).to_path_buf())?;
    let last_commit_current = CurrentCommit::read()?;

    let send_new_tags = process_refs_tag(send_refs, current_repo)?;
    let packfile = init_packfile(
        last_commit_current,
        current_repo,
        &last_commit_server,
        &send_new_tags,
    )?;

    send_pack(packfile, stream, log_entries, send_new_tags)?;

    let msg_done = "0000";
    stream.write_all(msg_done.as_bytes())?;
    Ok(())
}
/// Procesa las referencias enviadas al servidor para determinar el último commit en el servidor
/// para la rama actual del repositorio local.
/// Recibe un vector de referencias `send_ref` y la ruta del repositorio local como `current
fn process_hash_server(
    send_ref: &Vec<String>,
    current_repo: PathBuf,
) -> Result<String, std::io::Error> {
    let mut exist_branch_in_server = false;
    let mut last_commit_server = String::new();

    for refs in send_ref {
        if refs.contains(&Init::get_current_branch(&current_repo)?) {
            exist_branch_in_server = true;
            let parts: Vec<&str> = refs.split_ascii_whitespace().collect();
            last_commit_server = parts[0].to_string();
            break;
        }
    }
    if !exist_branch_in_server {
        last_commit_server = COMMIT_INIT_HASH.to_string();
    }
    Ok(last_commit_server)
}

// Inicializa un packfile que contiene la información necesaria para enviar al servidor,
/// incluyendo objetos relacionados con el último commit local, el último commit en el servidor
/// y nuevas etiquetas a enviar.
/// Recibe el último commit local como `last_commit_current`, la ruta del repositorio local como `current_repo`,
/// el hash del último commit en el servidor como `last_commit_server` y un slice de nuevas etiquetas `send_new_tag`.
fn init_packfile(
    last_commit_current: String,
    current_repo: &Path,
    last_commit_server: &str,
    send_new_tag: &[String],
) -> Result<Vec<u8>, std::io::Error> {
    let mut packfile: Vec<u8> = Vec::new();

    let mut objects_data: Vec<(String, usize, usize)> = Vec::new();
    Encoder::get_object_for_commit(
        current_repo,
        &mut objects_data,
        &last_commit_current,
        last_commit_server,
    )?;

    process_directory_to_send_new_tag(
        &current_repo.join(".rust_git").join("refs").join("tags"),
        &mut objects_data,
        send_new_tag.to_vec(),
        current_repo,
    )?;

    Encoder::create_size_header(&mut packfile, objects_data.len())?;

    for objects in objects_data.iter().rev() {
        let object_type = Encoder::set_bits(objects.1 as u8, objects.2)?;
        for object in object_type {
            packfile.push(object);
        }
        let path = Path::new(&objects.0);

        let compress_data = Encoder::compress_object(path, objects.1)?;

        for byte in compress_data {
            packfile.push(byte);
        }
    }
    Ok(packfile)
}

/// Procesa el contenido de un directorio para incluir en objects_data las entradas
/// relacionadas con nuevas etiquetas que se enviarán al servidor.
/// Recibe la ruta del directorio como `path`, una referencia mutable a objects_data,
/// un vector de nuevas etiquetas a enviar `send_new_tag` y la ruta del repositorio local como `current_repo`.
fn process_directory_to_send_new_tag(
    path: &Path,
    objects_data: &mut Vec<(String, usize, usize)>,
    send_new_tag: Vec<String>,
    current_repo: &Path,
) -> Result<Vec<(String, usize, usize)>, std::io::Error> {
    let send_new_tag_set: HashSet<_> = send_new_tag
        .into_iter()
        .map(|s| match s.split_whitespace().next() {
            Some(hash) => hash.to_string(),
            None => String::new(),
        })
        .collect();

    for entrada in fs::read_dir(path)? {
        let entrada = entrada?;
        let entry_path = entrada.path();
        if entry_path.is_file() {
            let hash_bytes = fs::read(&entry_path)?;
            let hash = String::from_utf8_lossy(&hash_bytes).trim().to_string();

            if send_new_tag_set.contains(&hash) {
                let data = process_particular_tag_to_send(&entry_path, current_repo)?;
                if data.1 != 0 {
                    objects_data.push(data);
                }
            }
        }
    }
    Ok(objects_data.to_vec())
}

/// Procesa un archivo de etiqueta específico para incluir en el packfile,
/// obteniendo datos relevantes sobre la etiqueta.
/// Recibe la ruta del archivo como `file_path` y la ruta para leer objetos como `path_to_read`.
fn process_particular_tag_to_send(
    file_path: &Path,
    path_to_read: &Path,
) -> Result<(String, usize, usize), std::io::Error> {
    let mut content_hash = String::new();
    let mut file = fs::File::open(file_path)?;

    file.read_to_string(&mut content_hash)?;

    let content = CatFile::cat_file(&content_hash, Init::get_object_path(path_to_read)?)?;

    if content.contains("tag") {
        let folder_name = content_hash.chars().take(2).collect::<String>();
        let object_path = Init::get_object_path(path_to_read)?;

        let final_path =
            object_path.join(format!("{}/{}", folder_name, &content_hash[2..]).as_str());
        return Ok((
            final_path.to_string_lossy().to_string(),
            4_usize,
            content.len(),
        ));
    }
    Ok(("NONE".to_string(), 0, 0))
}

/// Envía un packfile al servidor a través de un flujo TCPStream, incluyendo información de registro y nuevas etiquetas.
/// Recibe el packfile como `packfile`, un mutable TcpStream `stream` para la comunicación,
/// entradas de registro como `log_entries` y un vector de nuevas etiquetas como `send_new_tag`.
fn send_pack(
    packfile: Vec<u8>,
    stream: &mut TcpStream,
    log_entries: &[String],
    send_new_tag: Vec<String>,
) -> Result<String, std::io::Error> {
    let entry_hash = format!("{}\n", log_entries[0]);
    stream.write_all(to_pkt_line(&entry_hash).as_bytes())?;

    for tag in send_new_tag {
        let tag_to_pkt_line = to_pkt_line(&tag);
        stream.write_all(tag_to_pkt_line.as_bytes())?;
    }

    let msg_done = "0000";
    stream.write_all(msg_done.as_bytes())?;

    stream.write_all(&packfile)?;
    Ok("0000".to_string())
}
