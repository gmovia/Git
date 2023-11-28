use std::{fs::{OpenOptions, self}, io::{Read, Write, self, BufRead}, collections::HashMap};

use crate::{vcs::{files::{current_repository::CurrentRepository, commits_table::CommitsTable, repository::Repository, current_commit::CurrentCommit}, entities::commit_table_entry::CommitTableEntry, commands::checkout::Checkout}, constants::constant::{STATE_CREATED, STATE_MODIFIED}, utils::randoms::random::Random};

use super::{branch::Branch, diff::Diff};

pub struct Rebase;

impl Rebase{
    pub fn rebase(branch: &str) -> Result<(), std::io::Error>{ // git rebase rama destino
        let current = CurrentRepository::read()?;
        let current_branch = Branch::get_current_branch(&current)?;

        let old_current_commits_table = CommitsTable::read(current.clone(), &current_branch)?;
        let branch_commits_table = CommitsTable::read(current.clone(), branch)?;
        
        // ESCRIBIMOS TODOS LOS COMMITS DE BRANCH (REBASE)
        let mut current_file = OpenOptions::new().write(true).create(true).append(true).open(current.join(".rust_git").join("logs").join(&current_branch))?;

        let content = fs::read_to_string(current.join(".rust_git").join("logs").join(branch))?;
        current_file.set_len(0)?;
        current_file.write_all(content.as_bytes())?;
        CurrentCommit::write(CurrentCommit::read_for_branch(&current, branch)?)?;

        //let _ = CommitsTable::read(current.clone(), &current_branch)?;
        println!("CONTENT - CURRENT FILE {:?}\n",content);
        //let branch_last_hash = CurrentCommit::read_for_branch(&current, branch)?;

        let mut commits_rebase = Vec::new();
        for commit in old_current_commits_table{
            println!("{:?}", commit);
            if !CommitsTable::contains(&branch_commits_table, &commit){
                commits_rebase.push(commit);                  
            }
        }
        println!("COMMITS_REBASE VEC {:?}\n",commits_rebase);

        // MASTER       C1 (f1, f2)     C2 (f1, f2, f3)                                                 C5(f1', f2, f3, f6)
        // NEW BRANCH                                      C3(f1, f2, f3, f4)   C4(f1, f2, f3, f4, f5)
        for commit in commits_rebase{
            let mut repository_last_commit: HashMap<String, String> =  Repository::read_repository_of_branch(current.clone(), branch)?;
            println!("commit hash {:?}\n",commit.hash);
            let repository_commit: HashMap<String, String> = Repository::get_repository(current.clone(),&commit.hash)?;
            println!("REPOSITORY_LAST_COMMIT {:?}\n",repository_last_commit);
            println!("REPOSITORY_COMMIT {:?}\n",repository_commit);
            repository_last_commit.extend(repository_commit);
            CommitsTable::write(&commit.message, &repository_last_commit)?; 
        }

        /*
        - ARCHIVO TEMPORAL PARA ESCRIBIR
        - QUEDARNOS CON EL COMMIT EN COMUN (LA PRIMERA LINEA DE LA TABLA DE COMMITS DE LA RAMA BASE)
        - TRAER LOS COMMITS SALVO LA PRIMERA LINEA DE LA TABLA DE LA RAMA REBASE
        - GENERAR NUEVOS COMMITS PARA MASTER DESDE EL ULTIMO COMMIT DE LA RAMA REBASE --> ENTRAR A LOS BLOBS DEL COMMIT, LEERLOS Y ESCRIBIRLOS
        EN UN NUEVO COMMIT JUNTO CON LOS BLOBS DE LOS COMMITS DE MASTER
        - LUEGO DE TENER TODO 

        */
        Checkout::update_cd(&current)?;

        Ok(())
    }
}

// MASTER
// f1
// f2
// f3

// NEW BRANCH
// f1
// f4

// git rebase new branch

// f1
// f4


// MASTER (c1, c2, c3)
// NEW_BRANCH (c1, c4, c5)
// git rebase new_branch
// MASTER (c1, c4, c5, c2, c3)
// c2 c3 en vez de aplicarse sobre c1 se aplican sobre c5



/* 



MASTER          C1  C2          C5
NEW_BRANCH      C1  C2  C3  C4      C6
INTERMEDIO   METER TODO NEW BRANCH + LOS COMMITS DE MASTER NUEVOS


git rebase new_branch 

MASTER C1 C2 "C3 C4 C6" C5


MASTER C1 C2 
NEW_BRANCH C1 C3

// MASTER
C1
file1.txt

C2
file1.txt
file2.txt

// NEW_BRANCH

C3 
file1.txt 
file3.txt


// git rebase new_branch C1 C3 C2'

C1 
file1.txt

C3 
file1.txt 
file3.txt 

C2' 
file1.txt
file2.txt
file3.txt


MASTER C1 C2                     C4 C5
            NEW BRANCH C1 C2 C3         C6


git rebase new_branch

MASTER C1 C2 C3 C6 C4' C5'
*/