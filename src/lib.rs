pub mod constants{
    pub mod constant;
}

pub mod handlers{
    pub mod commands;
    pub mod status;
    pub mod hash_object;
    pub mod add;
    pub mod rm;
    pub mod cat_file;
    pub mod branch;
    pub mod checkout;
    pub mod log;
    pub mod commit;
    pub mod merge;
    pub mod clone;
    pub mod ls_files;
    pub mod fetch;
    pub mod ls_tree;
    pub mod pull;
    pub mod tag;
}

pub mod proxies{
    pub mod proxy;
}

pub mod clients{
    pub mod client;
}

pub mod servers{
    pub mod server;
    pub mod upload_pack;
    pub mod encoder;
}

pub mod packfiles{
    pub mod packfile;
}

pub mod interfaces{
    pub mod interface;
    pub mod draw;
    pub mod handler;
    pub mod css;
    pub mod handler_button;
}

pub mod types{
    pub mod set_type;
}

pub mod utils {
    pub mod files {
        pub mod file;
    }

    pub mod sets {
        pub mod set;
    }

    pub mod hashers{
        pub mod hasher;
    }
    pub mod randoms{
        pub mod random;
    }

}

pub mod vcs{
    pub mod files{
        pub mod vcs_file;
        pub mod index;
        pub mod repository;
        pub mod current_repository;
        pub mod current_commit;
        pub mod commits_table;
        pub mod repositories;
    }

    pub mod commands{
        pub mod init;
        pub mod hash_object;
        pub mod cat_file;
        pub mod status;
        pub mod add;
        pub mod rm;
        pub mod commit;
        pub mod log;
        pub mod branch;
        pub mod checkout;
        pub mod merge;
        pub mod diff;
        pub mod reset;
        pub mod clone;
        pub mod ls_files;
        pub mod fetch;
        pub mod ls_tree;
        pub mod check_ignore;
        pub mod tag;
    }

    pub mod sets{
        pub mod set;
    }

    pub mod entities{
        pub mod entity;
        pub mod commit_table_entry;
        pub mod commit_entity;
        pub mod tree_entity;
        pub mod blob_entity;
        pub mod change;
        pub mod conflict;
        pub mod tag_entity;
    }
  
    pub mod version_control_system;
}
