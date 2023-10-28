use std::{cell::RefCell, rc::Rc};

use crate::vcs::{commands::branch::BranchOptions, version_control_system::VersionControlSystem};

use super::interface::RustInterface;
use gtk::{prelude::*, Label};

pub fn handle_commit(interface: &RustInterface){
    let grid = interface.grid.clone();
    let box_window = interface.box_window.clone();

    interface.commit_button.connect_clicked(move |_| {
        grid.foreach(|child|{
            grid.remove(child);
        });
        
        box_window.foreach(|child|{
            box_window.remove(child);
        });
    });
}

pub fn handle_branch(interface: &RustInterface, vcs: &VersionControlSystem) {
    let dialog = interface.dialog.clone();

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
            rc_entry.set_text("Create new branch ...");
    }});
}


