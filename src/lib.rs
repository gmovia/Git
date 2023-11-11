pub mod constants{
    pub mod constants;
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
}

pub mod proxy{
    pub mod proxy;
}

pub mod client{
    pub mod client;
}

pub mod server{
    pub mod server;
    pub mod upload_pack;
    pub mod encoder;
}

pub mod packfile{
    pub mod packfile;
}

pub mod interface{
    pub mod interface;
    pub mod draw;
    pub mod handler;
    pub mod css;
    pub mod handler_button;
}

pub mod types{
    pub mod types;
}

pub mod utils {
    pub mod files {
        pub mod files;
    }

    pub mod sets {
        pub mod sets;
    }

    pub mod hasher{
        pub mod hasher;
    }
    pub mod random{
        pub mod random;
    }

}

pub mod vcs{
    pub mod files{
        pub mod vcs_file;
        pub mod index;
        pub mod repository;
        pub mod current_repository;
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
    }

    pub mod sets{
        pub mod sets;
    }

    pub mod entities{
        pub mod commit_table_entry;
        pub mod commit_entity;
        pub mod tree_entity;
        pub mod blob_entity;
        pub mod change;
        pub mod conflict;
    }
  
    pub mod version_control_system;
}
