pub mod constants {
    pub mod constant;
}

pub mod handlers {
    pub mod add;
    pub mod branch;
    pub mod cat_file;
    pub mod checkout;
    pub mod clone;
    pub mod commands;
    pub mod commit;
    pub mod fetch;
    pub mod hash_object;
    pub mod log;
    pub mod ls_files;
    pub mod ls_tree;
    pub mod merge;
    pub mod pull;
    pub mod push;
    pub mod rebase;
    pub mod remote;
    pub mod rm;
    pub mod show_ref;
    pub mod status;
    pub mod tag;
}

pub mod proxies {
    pub mod proxy;
}

pub mod clients {
    pub mod client;
}

pub mod servers {
    pub mod encoder;
    pub mod server;
    pub mod upload_pack;
}

pub mod server_http {
    pub mod web_server;
}

pub mod packfiles {
    pub mod packfile;
    pub mod tag_file;
}

pub mod interfaces {
    pub mod css;
    pub mod draw;
    pub mod handler;
    pub mod handler_button;
    pub mod interface;
    pub mod login;
}

pub mod protocol {
    pub mod receive_pack;
    pub mod send_pack;
}

pub mod types {
    pub mod set_type;
}

pub mod utils {
    pub mod files {
        pub mod file;
    }

    pub mod sets {
        pub mod set;
    }

    pub mod hashers {
        pub mod hasher;
    }
    pub mod randoms {
        pub mod random;
    }
}

pub mod vcs {
    pub mod files {
        pub mod commits_table;
        pub mod config;
        pub mod current_commit;
        pub mod current_repository;
        pub mod index;
        pub mod log;
        pub mod repositories;
        pub mod repository;
        pub mod vcs_file;
    }

    pub mod commands {
        pub mod add;
        pub mod branch;
        pub mod cat_file;
        pub mod check_ignore;
        pub mod checkout;
        pub mod clone;
        pub mod commit;
        pub mod diff;
        pub mod fetch;
        pub mod hash_object;
        pub mod init;
        pub mod log;
        pub mod ls_files;
        pub mod ls_tree;
        pub mod merge;
        pub mod pull;
        pub mod push;
        pub mod rebase;
        pub mod remote;
        pub mod reset;
        pub mod rm;
        pub mod show_ref;
        pub mod status;
        pub mod tag;
    }

    pub mod sets {
        pub mod set;
    }

    pub mod entities {
        pub mod blob_entity;
        pub mod change;
        pub mod commit_entity;
        pub mod commit_table_entry;
        pub mod conflict;
        pub mod entity;
        pub mod ref_delta_entity;
        pub mod tag_entity;
        pub mod tree_entity;
    }

    pub mod version_control_system;
}
