#[cfg(test)]
mod tests {

    use crate::tests_functions::{create_file, set_up};
    use rust_git::vcs::{
        commands::{branch::BranchOptions, show_ref::ShowRefOptions, tag::TagOptions},
        version_control_system::VersionControlSystem,
    };

    #[test]
    pub fn test_01_show_ref() -> Result<(), std::io::Error> {
        let temp_dir = set_up();
        let file_1 = create_file(&temp_dir, "file1.txt");

        VersionControlSystem::add(&file_1)?;
        VersionControlSystem::commit("first_commit".to_string())?;
        VersionControlSystem::tag(TagOptions::CreateLight("v1"))?;

        let refs_all = VersionControlSystem::show_ref(ShowRefOptions::GetAll)?;
        let tags = VersionControlSystem::show_ref(ShowRefOptions::GetRefTags)?;
        let heads = VersionControlSystem::show_ref(ShowRefOptions::GetRefHeads)?;

        assert_eq!(refs_all.len(), 2);
        assert_eq!(tags.len(), 1);
        assert_eq!(heads.len(), 1);

        Ok(())
    }

    #[test]
    pub fn test_02_show_ref() -> Result<(), std::io::Error> {
        let temp_dir = set_up();
        let file_1 = create_file(&temp_dir, "file1.txt");

        VersionControlSystem::add(&file_1)?;
        VersionControlSystem::commit("first_commit".to_string())?;

        VersionControlSystem::branch(BranchOptions::NewBranch("new_branch"))?;
        VersionControlSystem::tag(TagOptions::CreateLight("v1"))?;

        let refs_all = VersionControlSystem::show_ref(ShowRefOptions::GetAll)?;
        let tags = VersionControlSystem::show_ref(ShowRefOptions::GetRefTags)?;
        let heads = VersionControlSystem::show_ref(ShowRefOptions::GetRefHeads)?;

        assert_eq!(refs_all.len(), 3);
        assert_eq!(tags.len(), 1);
        assert_eq!(heads.len(), 2);

        Ok(())
    }
}
