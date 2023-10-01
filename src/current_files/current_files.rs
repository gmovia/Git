use std::{path::Path, fs, collections::HashMap};

pub fn get_files(path: String) -> Result<HashMap<String, String>, std::io::Error>{
    read(path, HashMap::new())
}

fn read(path: String,mut files: HashMap<String, String>) -> Result<HashMap<String,String>, std::io::Error>{

    files.insert(path.clone(), path.clone());

    let path = Path::new(&path);

    if let Ok(entrys) = fs::read_dir(path){
        for entry in entrys{
            if let Ok(entry) = entry{
                if entry.path().is_file(){
                    let value = fs::read_to_string(&entry.path())?;
                    if let Ok(path_name) = entry.path().into_os_string().into_string(){
                        files.insert(path_name, value);
                    }
                }
                if entry.metadata()?.is_dir(){
                    if let Ok(path_name) = entry.path().into_os_string().into_string(){
                        files = read(path_name, files.clone())?;
                    }
                }
                    
            }
        }
    }
    Ok(files)
}

