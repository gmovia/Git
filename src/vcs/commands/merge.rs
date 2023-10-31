// (1) B esta incluido totalmente en A => no hace nada
// Rama master      => [1, 2, 3]
// Rama new_branch  => [1, 2]

// master
// primer commit
// segundo commit
// checkout -b new_branch
// checkout master
// tercer commit

// (2) A esta incluido totalmente en B => FUSION AUTOMATICA => no hay conflictos
// Rama master      => [1, 2, 3]     
// Rama new_branch  => [1, 2, 3, 4]

// master
// primer commit
// segundo commit
// tercer commit
// checkout -b new_branch
// cuarto commmit

// (3) ni A contiene a B totalmente ni B contiene totalmente a A => puede haber FUSION AUTOMATICA o CONFLICTOS
// Rama master => [1, 2, 3, 4]
// Rama new_branch => [1, 2, 3, 5]

use crate::vcs::files::repository::Repository;

//
// si un archivo de repBase se modifico en repA y en repB => conflicto => el ultimo caso a analizar
#[derive(Debug, Clone)]
pub struct Merge;

impl Merge {
    pub fn merge(repository: &Repository, branch: &str) -> Result<String,std::io::Error> {
        //let actual_commit_table = repository.read_commits_hashes(&Init::get_current_branch(&repository.path)?)?;// get commits path => te trae e
        //let branch_commit_table = repository.read_commits_hashes(branch)?;// get commits path => te trae e

        // (1) Si B esta contenida totalmente en A => si actual_commit_table es mas largo que branch_commit_table ==> no hago nada
        //if actual_commit_table.iter().any(|cadena1| branch_commit_table.iter().all(|cadena2| cadena1 == cadena2)) {
        //    return Ok("The branch is already up to date".to_string());
       // }

        // (2) Si A esta contenido totalmente en B => me traigo todo lo de B
        //if branch_commit_table.iter().any(|cadena1| actual_commit_table.iter().all(|cadena2| cadena1 == cadena2)) {
            // nos traemos el commit hash ultimo que tienen en comun, en este caso, commit2hash
            // leemos la tabla de commits de branch_commit_table y nos traemos las filas a partir de commit2hash
                // en este caso, commit3hash y commit4has
                // y escribimos esto en la tabla de actual_commit_table
        
        //}
        Ok("Ok".to_string())
    }


    pub fn get_parent_commit(commits_a: Vec<String>, commits_b: Vec<String>) -> Option<String>{
        for (i, commit_a) in commits_a.iter().rev().enumerate() {
            if let Some(commit_b) = commits_b.iter().rev().nth(i) {
                if commit_a == commit_b {
                    return Some(commit_a.clone());
                } else {
                    break;
                }
            }
        }
        None
    }
}
