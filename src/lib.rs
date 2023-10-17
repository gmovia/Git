pub mod handlers{
    pub mod status;
    pub mod hash_object;
    pub mod add;
    pub mod rm;
    pub mod cat_file;
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
    }

    pub mod commands{
        pub mod init;
        pub mod hash_object;
        pub mod cat_file;
        pub mod status;
        pub mod add;
        pub mod rm;
        pub mod commit;
    }

    pub mod sets{
        pub mod sets;
    }
  
    pub mod version_control_system;
}
