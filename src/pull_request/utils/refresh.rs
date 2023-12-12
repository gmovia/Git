use std::path::Path;

use crate::{pull_request::db::queries::Query, vcs::commands::merge::Merge};

pub fn refresh(prs_path: &Path) -> Result<(), std::io::Error>{
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