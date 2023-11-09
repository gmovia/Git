use std::path::PathBuf;
use crate::vcs::entities::{blob_entity::BlobEntity, tree_entity::TreeEntity, commit_entity::CommitEntity};

pub struct Proxy;

impl Proxy{

    /// COMMITS

    pub fn write_commit(repo_path: PathBuf, tree_hash: String) -> Result<String, std::io::Error>{
        Ok(CommitEntity::write(&repo_path, &tree_hash, )?)
    } // hash commit

    pub fn read_commit(repo_path: PathBuf, commit_hash: String) -> Result<CommitEntity, std::io::Error>{
        Ok(CommitEntity::read(&repo_path, &commit_hash)?)
    } // tree + hash del tree
    
    
    /// TREES

    pub fn write_tree(repo_path: PathBuf, blobs: Vec<BlobEntity>) -> Result<String, std::io::Error>{
        Ok(TreeEntity::write(&repo_path, &blobs)?)
    } // hash tree


    pub fn read_tree(repo_path: PathBuf, tree_hash: String) -> Result<Vec<BlobEntity>, std::io::Error>{
        Ok(TreeEntity::read(&repo_path, tree_hash)?)
    }   // los blobs => blob + path + hash del blob
    
    
    /// BLOBS

    pub fn write_blob(repo_path: PathBuf, content: &String) -> Result<String, std::io::Error>{
        Ok(BlobEntity::write(repo_path, content)?)
    } // hash blob

    pub fn read_blob(repo_path: PathBuf, blob_hash: String) -> Result<String, std::io::Error>{
        Ok(BlobEntity::read(repo_path, blob_hash)?)
    }   // contenido
}

// el mismo nombre cliente-servidor


/* 
CLIENTE
    repo_1

SERVIDOR 
test_folder/
    repo_1/
            file1.txt
            file2.txt

GIT CLONE .

### SERVIDOR

LEO LA TABLA DE COMMIT Y ME QUEDO CON EL ULTIMO => hash_commit

Proxy::read_commit(test_folder/repo_1, hash_commit) => Aca te traes "tree hash_tree"
Proxy::read_tree(test_folder/repo_1, hash_tree) => Aca te traes los blobs => "blob file_path hash_blob"
Proxy::read_blob(test_folder/repo1, hash_blob) => Aca te traes el contenido del blob

### CLIENTE

Lo primero que le llega al cliente es "hash_commit:(tree hash_tree mensaje_commit + otros datos)"
Proxy::write_commit(repo_1, hash_tree) => te da nuestro hash_commit distinto al que genera git, ojota! este lo pasas despues!

Despues te llega el contenido del tree => (blobs, file_paths, hash_blob)
Vos te armas el vector de blob! ("blob", file_blob, hash_blob) entonces te armas un Vec<BlobEntity>
Proxy::write_tree(repo_1, blobs) => te da nuestro hash_tree distinto al que te genera git(? es una pregunta)

Ahora lo que hay que hacer es recorrer la lista de los blobs con sus contenidos!
Proxy::write_blob(repo_1, contenido) => te crea el blob 

SOLO FUNCIONA SI LOS TREE TIENEN BLOBS! SI TIENEN TREE HAY QUE HACER CAMBIOS!

*/

