use std::{cell::RefCell, rc::Rc};

use crate::vcs::{commands::branch::BranchOptions, version_control_system::VersionControlSystem};

use super::{interface::RustInterface, draw::changes_and_staging_area};
use gtk::prelude::*;

pub fn handle_commit(interface: &RustInterface, vcs: &VersionControlSystem){
    let box_window = interface.box_window.clone();
    let dialog = interface.commit_dialog.clone();
    let version = Rc::new(RefCell::new(vcs.clone()));
    let rc_entry = Rc::new(RefCell::new(interface.message.clone()));
    let rc_ok = Rc::new(RefCell::new(interface.message_ok.clone()));
    
    interface.message_ok.set_sensitive(false);

    interface.message.connect_changed({
        let rc_ok = rc_ok.clone();
        move |e| {
        let rc_ok = rc_ok.borrow_mut();
        rc_ok.set_sensitive(!e.text().is_empty());
    }});

    interface.commit_button.connect_clicked({
        move |_| {        
            dialog.run();
            dialog.hide();
            box_window.foreach(|child| {
                box_window.remove(child);
            });
        }
    });
    interface.message_ok.connect_clicked({
        let rc_entry = rc_entry.clone();
        let version = version.clone();
        move |button| {
            let mut version = version.borrow_mut();
            let rc_entry = rc_entry.borrow_mut();

            let _ = version.commit(rc_entry.text().to_string());

            rc_entry.set_text("");
            button.set_sensitive(false);
        }
    });
    
}

pub fn handle_branch(interface: &RustInterface, vcs: &VersionControlSystem) {
    let dialog = interface.branch_dialog.clone();

    let version = Rc::new(RefCell::new(vcs.clone()));
    let rc_entry = Rc::new(RefCell::new(interface.dialog_entry.clone()));
    let rc_branch = Rc::new(RefCell::new(interface.select_branch.clone()));
    let rc_create = Rc::new(RefCell::new(interface.create_branch.clone()));

    interface.create_branch.set_sensitive(false);

    interface.dialog_entry.connect_changed({  
        let rc_create = rc_create.clone();
        move |e| {
        let rc_create = rc_create.borrow_mut();
        rc_create.set_sensitive(!e.text().is_empty());
    }});

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
        move |button|{
            let version = version.borrow_mut();
            let rc_entry = rc_entry.borrow_mut();
            let rc_branch = rc_branch.borrow_mut();

            let _ = version.branch(BranchOptions::NewBranch(&rc_entry.text()));

            rc_branch.append_text(&rc_entry.text());

            rc_entry.set_text("");
            button.set_sensitive(false);
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


