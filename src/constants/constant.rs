// PATHS
pub const BDD_PATH: &str = "repositories.txt";
pub const NULL_PATH: &str = "null_path";
pub const CURRENT_REPOSITORY_PATH: &str = "current_repository.txt";
pub const RUST_PATH: &str = ".rust_git";
pub const  CONFIG_PATH: &str = "config.txt";

// CODES
pub const TREE_CODE: &str = "tree";
pub const TREE_CODE_NUMBER: u8 = 2;

// BLOB
pub const BLOB_CODE: &str = "blob";
pub const BLOB_CODE_NUMBER: u8 = 3;

// COMMIT
pub const COMMIT_CODE_NUMBER: u8 = 1;
pub const COMMIT_CODE: &str = "commit";

// TAG
pub const TAG_CODE: &str = "tag";
pub const TAG_CODE_NUMBER: u8 = 4;


pub const COMMIT_INIT_HASH: &str = "0000000000000000000000000000000000000000";

// SERVER
pub const PUERTO: &str = "9418";
pub const HOST: &str = "127.0.0.1";

// NULL
pub const NULL: &str = "NULL";
pub const END_OF_LINE: &str = "";

// MERGE
pub const MERGE: &str = "merge";
pub const NONE: &str = "N";
pub const CURRENT: &str = "C";
pub const INCOMING: &str = "I";
pub const BOTH: &str = "B";

// STATES
pub const STATE_CREATED: &str = "CREATED";
pub const STATE_MODIFIED: &str = "MODIFIED";
pub const STATE_DELETED: &str = "DELETED";

// FILES
pub const UNTRACKED_FILES: &str = "UNTRACKED";
pub const CHANGES_NOT_BE_COMMITED: &str = "NOT COMMITED";
pub const CHANGES_TO_BE_COMMITED: &str = "AREA";

// COMMANDS
pub const FULL_ADD: &str = ".";

// RESPONSES
pub const RESPONSE_OK_ADD: &str = "Added successfully.";
pub const RESPONSE_OK_RM: &str = "Removed successfully.";
pub const RESPONSE_OK_CREATE_BRANCH: &str = "Created successfully.";
pub const RESPONSE_OK_DELETED_BRANCH: &str = "Deleted successfully.";
pub const RESPONSE_OK_COMMIT: &str = "Commit created succesfully.";
pub const RESPONSE_OK_MERGE: &str = "Merged successfully";

pub const RESPONSE_OK_IGNORE: &str = "The path is ignored as per the rules in the .gitignore file.";
pub const RESPONSE_NOK_IGNORE: &str = "The path is not ignored or is not listed in the .gitignore file.";

pub const RESPONSE_OK_CLONE: &str = "Clone successfully";
pub const RESPONSE_OK_PUSH: &str = "Push successfully";

pub const RESPONSE_NOK_CLONE: &str = "The path is not correct";
pub const RESPONSE_OK_FETCH: &str = "Fetched successfully";
pub const RESPONSE_OK_PULL: &str = "Pull successfully";

// ERRORS
pub const ERR_INVALID_PARAMETERS: &str = "Invalid parameters.";
pub const ERR_GET_BRANCHES: &str = "Error getting the branches.";
pub const ERR_NO_SUCH_OR_DIRECTORY: &str = "No such file or directory";
pub const ERR_PATH_IS_NOT_DIRECTORY_OR_NO_SUCH_OR_DIRECTORY: &str = "The path is an directory or no such file or directory.";
pub const ERR_COMMIT_IS_NOT_EXIST: &str = "No commits exist.";
pub const ERR_STATUS: &str = "Failed to get status";
pub const ERR_MERGE: &str = "There are conflicts";