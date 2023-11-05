
use std::path::Path;

use gtk::prelude::*;

use crate::vcs::version_control_system::VersionControlSystem;
use crate::interface::draw::{repositories, branches, changes_and_staging_area};
use super::css::{init_css, set_styles_css};
use super::handler::{handle_branch, handle_commit, handle_status, handle_log, handle_repository, handle_command, handle_rm, handle_merge, handle_clone};

pub struct RustInterface {
    pub window: gtk::Window,
    pub title: gtk::Label,
    pub _box: gtk::Box,
    pub commit_button: gtk::Button,
    pub grid: gtk::Grid,
    pub select_repository: gtk::ComboBoxText,
    pub repository_button: gtk::Button,
    pub repository_dialog: gtk::Dialog,
    pub repository_entry: gtk::Entry,
    pub delete_repository: gtk::Button,
    pub create_repository: gtk::Button,
    pub select_branch: gtk::ComboBoxText,
    pub grid_staging: gtk::Grid,
    pub branch_button: gtk::Button,
    pub branch_dialog: gtk::Dialog,
    pub dialog_entry: gtk::Entry,
    pub create_branch: gtk::Button,
    pub delete_branch: gtk::Button,
    pub status: gtk::Button,
    pub commit_dialog: gtk::Dialog,
    pub message: gtk::Entry,
    pub message_ok: gtk::Button,
    pub log: gtk::Button,
    pub log_dialog: gtk::Dialog,
    pub log_box: gtk::Box,
    pub close_log: gtk::Button,
    pub title_changes: gtk::Label,
    pub title_sa: gtk::Label,
    pub terminal_dialog: gtk::Dialog,
    pub terminal: gtk::Button,
    pub command_entry: gtk::Entry,
    pub command_box: gtk::Box,
    pub enter: gtk::Button,
    pub rm: gtk::Button,
    pub rm_dialog: gtk::Dialog,
    pub rm_entry: gtk::Entry,
    pub rm_enter: gtk::Button,
    pub merge: gtk::Button,
    pub merge_entry: gtk::Entry,
    pub merge_dialog: gtk::Dialog,
    pub merge_changes: gtk::Box,
    pub resolve: gtk::Button,
    pub merge_grid: gtk::Grid,
    pub apply_merge: gtk::Button,
    pub clone: gtk::Button,
    pub clone_entry: gtk::Entry,
    pub info_clone: gtk::Box,
    pub fix: gtk::Fixed,
    pub both_dialog: gtk::Dialog,
    pub both_ok: gtk::Button,
    pub both_box: gtk::Box,
    pub both_text: gtk::TextView,
    pub fetch: gtk::Button,
}

impl RustInterface {

    pub fn new() -> RustInterface {
        if gtk::init().is_err() {
            println!("Failed to initialize GTK.");
        }

        let glade_src = include_str!("interface.glade");
        let builder = gtk::Builder::from_string(glade_src);
        
        init_css();

        RustInterface {
            window: builder.object("window").unwrap(),
            title: builder.object("title").unwrap(),
            _box: builder.object("box").unwrap(),
            commit_button: builder.object("commit").unwrap(),
            grid: builder.object("grid").unwrap(),
            select_repository: builder.object("select-repository").unwrap(),
            repository_button: builder.object("repository").unwrap(),
            repository_dialog: builder.object("repository-dialog").unwrap(),
            repository_entry: builder.object("dialog-entry-repo").unwrap(),
            delete_repository: builder.object("delete-repo").unwrap(),
            create_repository: builder.object("create-repo").unwrap(),
            select_branch: builder.object("select-branch").unwrap(),
            grid_staging: builder.object("grid-staging").unwrap(),
            branch_button: builder.object("branch").unwrap(),
            branch_dialog: builder.object("branch-dialog").unwrap(),
            dialog_entry: builder.object("dialog-entry").unwrap(),
            create_branch: builder.object("create").unwrap(),
            delete_branch: builder.object("delete").unwrap(),
            status: builder.object("status").unwrap(),
            commit_dialog: builder.object("commit-dialog").unwrap(),
            message: builder.object("message-entry").unwrap(),
            message_ok: builder.object("message-ok").unwrap(),
            log: builder.object("log").unwrap(),
            log_dialog: builder.object("log-dialog").unwrap(),
            log_box: builder.object("log-box").unwrap(),
            close_log: builder.object("close-log").unwrap(),
            title_changes: builder.object("title-changes").unwrap(),
            title_sa: builder.object("title-sa").unwrap(),
            terminal_dialog: builder.object("terminal-dialog").unwrap(),
            terminal: builder.object("terminal").unwrap(),
            command_box: builder.object("command-box").unwrap(),
            command_entry: builder.object("command-entry").unwrap(),
            enter: builder.object("enter").unwrap(),
            rm: builder.object("rm").unwrap(),
            rm_dialog: builder.object("rm-dialog").unwrap(),
            rm_entry: builder.object("rm-entry").unwrap(),
            rm_enter: builder.object("rm-enter").unwrap(),
            merge: builder.object("merge").unwrap(),
            merge_dialog: builder.object("merge-dialog").unwrap(),
            merge_entry: builder.object("merge-entry").unwrap(),
            merge_changes: builder.object("merge-changes").unwrap(),
            resolve: builder.object("resolve").unwrap(),
            merge_grid: builder.object("merge-grid").unwrap(),
            apply_merge: builder.object("apply-merge").unwrap(),
            clone: builder.object("clone").unwrap(),
            clone_entry: builder.object("clone-entry").unwrap(),
            info_clone: builder.object("info-clone").unwrap(),
            fix: builder.object("fix").unwrap(),
            both_dialog: builder.object("both-dialog").unwrap(),
            both_ok: builder.object("both-ok").unwrap(),
            both_box: builder.object("both-box").unwrap(),
            both_text: builder.object("both-text").unwrap(),
            fetch: builder.object("fetch").unwrap(),
        }
    }
    
    pub fn impl_interface(&self) -> Result<(), std::io::Error>{    
        set_styles_css(self);
        
        VersionControlSystem::init(Path::new("clone_here"), Vec::new());

        let _ = changes_and_staging_area(&self.grid, &self.grid_staging);
        repositories( &self.select_repository)?;
        branches( &self.select_branch)?;
        handle_branch(&self);
        handle_commit(&self);
        handle_status(&self);
        handle_log(&self);
        handle_command(&self);
        handle_rm(&self);
        handle_merge(&self);
        handle_clone(&self);
        handle_repository(&self);

        self.window.show_all();   
        gtk::main();  // esto corta el ciclo de ejecucion

        Ok(())
    }
}

