use std::{cell::RefCell, rc::Rc};

use crate::vcs::{commands::branch::BranchOptions, version_control_system::VersionControlSystem};

use super::{interface::RustInterface, draw::changes_and_staging_area};
use gtk::prelude::*;

pub fn handle_commit(interface: &RustInterface, vcs: &VersionControlSystem){
    let box_window = interface.box_window.clone();
    let dialog = interface.commit_dialog.clone();
    let version = Rc::new(RefCell::new(vcs.clone()));
    let rc_entry = Rc::new(RefCell::new(interface.message.clone()));
    interface.commit_button.connect_clicked({
        move |_| {        
        dialog.run();
        box_window.foreach(|child|{
            box_window.remove(child);
        });
        dialog.hide();
    }});

    interface.message_ok.connect_clicked({
        let rc_entry = rc_entry.clone();
        let version = version.clone();
        move |_| {
            let mut version = version.borrow_mut();
            let rc_entry = rc_entry.borrow_mut();
            let _ = version.commit(rc_entry.text().to_string());
        }
    });
    
}

pub fn handle_branch(interface: &RustInterface, vcs: &VersionControlSystem) {
    let dialog = interface.branch_dialog.clone();

    let version = Rc::new(RefCell::new(vcs.clone()));
    let rc_entry = Rc::new(RefCell::new(interface.dialog_entry.clone()));
    let rc_branch = Rc::new(RefCell::new(interface.select_branch.clone()));

    interface.new_branch_button.connect_clicked(
        move |_| {
            dialog.run();
            dialog.hide();
        }
    );

    interface.create_branch.connect_clicked({
        let version = version.clone();
        let rc_entry = rc_entry.clone();
        let rc_branch = rc_branch.clone();
        move |_|{
            let version = version.borrow_mut();
            let rc_entry = rc_entry.borrow_mut();
            let rc_branch = rc_branch.borrow_mut();
            let _ = version.branch(BranchOptions::NewBranch(&rc_entry.text()));
            rc_branch.append_text(&rc_entry.text());
    }});
}

pub fn handle_status(interface: &RustInterface, vcs: &VersionControlSystem) {
    let version = Rc::new(RefCell::new(vcs.clone()));
        let rc_grid = Rc::new(RefCell::new(interface.grid.clone()));
        let rc_add = Rc::new(RefCell::new(interface.box_window.clone()));
        interface.status.connect_clicked({
            let version = version.clone();
            let rc_grid = rc_grid.clone();
            let rc_add = rc_add.clone();
            move |_| {
                let _ = changes_and_staging_area(&version.borrow_mut(), &rc_grid.borrow_mut(), &rc_add.borrow_mut());
            }
        });
}


