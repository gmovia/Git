use std::thread;
use std::time::Duration;
use std::{path::Path, cell::RefCell, rc::Rc};

use gtk::{prelude::*, ComboBoxText, Button, Grid};

use crate::vcs::{version_control_system::VersionControlSystem, commands::branch::BranchOptions};
use crate::interface::draw::{repositories, branches, changes_and_staging_area};

use super::handler::{handle_branch, handle_commit};

pub struct RustInterface {
    pub window: gtk::Window,
    pub commit_button: gtk::Button,
    pub grid: gtk::Grid,
    pub select_repository: gtk::ComboBoxText,
    pub select_branch: gtk::ComboBoxText,
    pub box_window: gtk::Box,
    pub new_branch_button: gtk::Button,
    pub dialog: gtk::Dialog,
    pub dialog_entry: gtk::Entry,
    pub create_branch: gtk::Button,
}

impl RustInterface {

    pub fn new() -> RustInterface {
        if gtk::init().is_err() {
            println!("Failed to initialize GTK.");
        }

        let glade_src = include_str!("interface.glade");
        let builder = gtk::Builder::from_string(glade_src);
        
        RustInterface {
            window: builder.object("window").unwrap(),
            commit_button: builder.object("commit").unwrap(),
            grid: builder.object("grid").unwrap(),
            select_repository: builder.object("select-repository").unwrap(),
            select_branch: builder.object("select-branch").unwrap(),
            box_window: builder.object("box-add").unwrap(),
            new_branch_button: builder.object("new-branch").unwrap(),
            dialog: builder.object("dialog").unwrap(),
            dialog_entry: builder.object("dialog-entry").unwrap(),
            create_branch: builder.object("create").unwrap(),
        }
    }
    
    pub fn impl_interface(&self) -> Result<(), std::io::Error>{    
        let vcs = VersionControlSystem::init(Path::new("test_folder"), Vec::new());
        repositories(&vcs, &self.select_repository)?;
        branches(&vcs, &self.select_branch)?;
        changes_and_staging_area(&vcs, &self.grid, &self.box_window)?;
        
        
        let version = Rc::new(RefCell::new(vcs));
        let rc_entry = Rc::new(RefCell::new(self.dialog_entry.clone()));
        let rc_branch = Rc::new(RefCell::new(self.select_branch.clone()));

        handle_commit(&self);
        handle_branch(&self);

        self.create_branch.connect_clicked({
            let version = version.clone();
            let rc_entry = rc_entry.clone();
            let rc_branch = rc_branch.clone();
            move |_|{
                let version = version.borrow_mut();
                let rc_entry = rc_entry.borrow_mut();
                let rc_branch = rc_branch.borrow_mut();
                let _ = version.branch(BranchOptions::NewBranch(&rc_entry.text()));
                rc_branch.append_text(&rc_entry.text());
                rc_entry.set_text("Create new branch ...");
            }});
    
        self.window.show_all();
    
        gtk::main();
        Ok(())
    }
}