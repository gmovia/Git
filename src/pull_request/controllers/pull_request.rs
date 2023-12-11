use std::path::{Path, PathBuf};

use crate::{pull_request::{schemas::schemas::{CreatePullRequest, PullRequestEntry, FindPullRequests, FindPullRequest}, validator::validator::Validator, db::queries::Query}, vcs::commands::merge::Merge};

pub struct PullRequest{
    server: PathBuf
}

impl PullRequest { 

    pub fn init(server: &Path) -> PullRequest{
        PullRequest{ server: server.to_path_buf() }
    }
    
    /// POST
    pub fn create(&self, pr: &mut CreatePullRequest) -> Result<String, std::io::Error>{
        Validator::validate_create_pull_request(&self.server, &pr)?;

        let base_repo = self.server.join(&pr.base_repo);
        let head_repo = self.server.join(&pr.head_repo);
        
        if !Merge::are_conflicts(&pr.head, &pr.base, &head_repo.clone(), &base_repo.clone())?{
            pr.mergeable = true;
        }

        let id = Query::create_pull_request(&self.server, &pr)?;
    
        Ok(id)
    }

    /// GET ALL
    pub fn find_all(&self, query: FindPullRequests) -> Result<Vec<PullRequestEntry>, std::io::Error>{
        Validator::validate_find_pull_requests(&self.server, &query)?;
        let prs_path = self.server.join("pull_requests").join(&query.base_repo);
        Query::find_pull_requests(&prs_path, &query)
    }

    /// GET
    pub fn find_one(&self, query: FindPullRequest) -> Result<PullRequestEntry, std::io::Error> {
        Validator::validate_find_a_pull_request(&self.server, &query)?;
        let id = self.server.join("pull_requests").join(&query.base_repo).join(&query.id);
        Query::find_a_pull_request(&id)
    }

    //pub fn find_commits(server: &Path, query: FindPullRequest) -> Result<Vec<CommitsPullRequest>, std::io::Error> {
        //Validator::validate_find_a_pull_request(server, &query)?;
        //let path = server.join(&query.base_repo);
        //let id = server.join("pull_requests").join(&query.base_repo).join(&query.id);
        //Query::get_commits_pull_request(&path, &id)
    //  }

    // SE USA PARA ACTUALIZAR EL ESTADO MERGEABLE EN EL CASO DE QUE SE HAGAN NUEVOS COMMTIS!
    // vos escribis la primera vez y si no tenes conflictos te queda true
    // ahora bien, si alguien llega a hacer un commit y se generan conflictos, queda el valor true viejo! deberia ser false!
    // deberia llamarse a esta funcion para actualizar los pr! solo actualizamos los pr que esten abiertos
    pub fn refresh(prs_path: &Path) -> Result<(), std::io::Error>{ // PARA ACTUALIZAR! HAY QUE VER DONDE SE USA! DSP VEMO!
        let prs = Query::find_all_pull_requests(prs_path)?;

        for mut pr in prs{
            let pr_path = prs_path.join(&pr.id);
            let head_repo = Path::new(&pr.head_repo);
            let base_repo = Path::new(&pr.base_repo);

            if pr.status == "open"{
                if !Merge::are_conflicts(&pr.head, &pr.base, head_repo, base_repo)?{
                    pr.mergeable = true;
                    Query::write_pull_request(&pr_path, &pr)?;
                    continue;
                }
        
                pr.mergeable = false;
                Query::write_pull_request(&pr_path, &pr)?;
            }
        }
        Ok(())
    }
}