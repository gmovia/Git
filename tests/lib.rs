pub mod utils {
    pub mod files {
        pub mod file;
    }

    pub mod sets {
        pub mod set;
    }
}

pub mod server_http {
    pub mod web_server;
}

pub mod vcs {
    pub mod add;
    pub mod branch;
    pub mod cat_file;
    pub mod check_ignore;
    pub mod checkout;
    pub mod clone;
    pub mod fetch;
    pub mod hash_object;
    pub mod ls_files;
    pub mod ls_tree;
    pub mod merge;
    pub mod pull;
    pub mod push;
    pub mod rebase;
    pub mod remote;
    pub mod repository;
    pub mod reset;
    pub mod rm;
    pub mod show_ref;
    pub mod status;
    pub mod tag;
    pub mod pull_request;
}

pub mod tests_functions;
