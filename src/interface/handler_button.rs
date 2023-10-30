use std::{cell::RefCell, rc::Rc};

use crate::vcs::{version_control_system::VersionControlSystem, commands::{branch::BranchOptions, checkout::CheckoutOptions}};

use super::{interface::RustInterface, draw::{branches, changes_and_staging_area}};

use gtk::prelude::*;


pub fn handle_buttons_branch(interface: &RustInterface, button_branch: &gtk::Button, vcs: &VersionControlSystem) {
    let version: Rc<RefCell<VersionControlSystem>> = Rc::new(RefCell::new(vcs.clone()));
    let rc_branch = Rc::new(RefCell::new(interface.select_branch.clone()));
    let rc_entry = Rc::new(RefCell::new(interface.dialog_entry.clone()));

    button_branch.connect_clicked({
        let version = version.clone();
        let rc_branch = rc_branch.clone();
        let rc_entry = rc_entry.clone();
        move |button| {
            let version = version.borrow_mut();
            let rc_branch = rc_branch.borrow_mut();
            let rc_entry = rc_entry.borrow_mut();
            if let Some(label) = button.label() {
                match label.as_str() {
                    "Create" => {let _ = version.branch(BranchOptions::NewBranch(&rc_entry.text()));},
                    "Delete" => {let _ = version.branch(BranchOptions::DeleteBranch(&rc_entry.text()));},
                    _ => {},
                }
            }
            rc_branch.foreach(|child| {
                if let Some(child_label) = child.downcast_ref::<gtk::Entry>() {
                    if child_label.text().to_string() != rc_entry.text().to_string() {
                        let _ = branches(&version, &rc_branch);
                    }
                }
            });
            rc_entry.set_text("");
            button.set_sensitive(false);
            
        }
    });
}

pub fn handle_button_select_branch(interface: &RustInterface, vcs: &VersionControlSystem) {
    let version: Rc<RefCell<VersionControlSystem>> = Rc::new(RefCell::new(vcs.clone()));
    interface.select_branch.connect_changed({
        let version = version.clone();
        move |combo_box|{
            let version = version.borrow_mut();
            if let Some(branch) = combo_box.active_text(){
                let _ = version.checkout(CheckoutOptions::ChangeBranch(&branch.to_string()));
            }
        }
    });
}

pub fn handle_commit_button(interface: &RustInterface, vcs: &VersionControlSystem) {
    let version = Rc::new(RefCell::new(vcs.clone()));
    let rc_entry = Rc::new(RefCell::new(interface.message.clone()));

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

pub fn handle_status_button(interface: &RustInterface, vcs: &VersionControlSystem) {
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