pub mod utils{
    pub mod files{
        pub mod file;
    }

    pub mod sets{
        pub mod set;
    }    
}

pub mod vcs{
    pub mod hash_object;
    pub mod cat_file;
    pub mod status;
    pub mod add;
    pub mod rm;
    pub mod branch;
    pub mod checkout;
    pub mod merge;
    pub mod reset;
    pub mod repository;
    pub mod clone;
    pub mod ls_files;
    pub mod ls_tree;
    pub mod fetch;
    pub mod pull;
    pub mod show_ref;
    pub mod tag;
    pub mod check_ignore;
}

pub mod tests_functions;