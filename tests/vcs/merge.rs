// #[cfg(test)]
// mod tests {

//     use rust_git::vcs::commands::checkout::CheckoutOptions;

//     use crate::tests_functions::{create_file, set_up};

//     #[test]
//     pub fn test_01_merge() -> Result<(), std::io::Error> {
//         let (temp_dir, mut vcs) = set_up();
//         let file_1 = create_file(&temp_dir, "file1.txt");
//         let file_2 = create_file(&temp_dir, "file2.txt");
//         let file_3 = create_file(&temp_dir, "file3.txt");
//         vcs.add(&file_1)?;
//         vcs.add(&file_2)?;
//         vcs.add(&file_3)?;
//         vcs.commit("first commit".to_string())?;

//         vcs.checkout(CheckoutOptions::CreateAndChangeBranch("new_branch"))?;
//         vcs.checkout(CheckoutOptions::ChangeBranch("master"))?;

//         let result = vcs.merge("new_branch")?;
//         assert_eq!("The branch is already up to date", result);
//         Ok(())
//     }

//     #[test]
//     pub fn test_02_merge() -> Result<(), std::io::Error> {
//         let (temp_dir, mut vcs) = set_up();
//         let file_1 = create_file(&temp_dir, "file1.txt");
//         let file_2 = create_file(&temp_dir, "file2.txt");
//         let file_3 = create_file(&temp_dir, "file3.txt");
//         vcs.add(&file_1)?;
//         vcs.add(&file_2)?;
//         vcs.add(&file_3)?;
//         vcs.commit("first commit".to_string())?;

//         vcs.checkout(CheckoutOptions::CreateAndChangeBranch("new_branch"))?;
//         vcs.checkout(CheckoutOptions::ChangeBranch("master"))?;
        
//         let file_4 = create_file(&temp_dir, "file4.txt");
//         vcs.add(&file_4)?;
//         vcs.commit("second commit".to_string())?;

//         let result = vcs.merge("new_branch")?;
//         assert_eq!("The branch is already up to date", result);
//         Ok(())
//     }

// }