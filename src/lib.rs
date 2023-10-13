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
}

pub mod vcs{
    pub mod files{
        pub mod vcs_file;
        pub mod index;
    }

    pub mod commands{
        pub mod init;
        pub mod hash_object;
        pub mod cat_file;
        pub mod status;
        pub mod add;
    }

    pub mod sets{
        pub mod sets;
    }
  
    pub mod version_control_system;
}
