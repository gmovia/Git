
use std::{path::Path, rc::Rc, cell::RefCell, io::{self, BufRead}, fs::OpenOptions};

use crate::{vcs::{version_control_system::VersionControlSystem, commands::{branch::BranchOptions, checkout::CheckoutOptions}, files::repository}, handlers::{rm::handler_rm, commands::handler_command}};

use super::{interface::RustInterface, draw::{branches, repositories}};

use gtk::prelude::*;


pub fn handle_buttons_branch(interface: &RustInterface, button_branch: &gtk::Button) {
    let rc_branch = interface.select_branch.clone();
    let rc_entry = interface.dialog_entry.clone();
    button_branch.connect_clicked({
        //let version = version.clone();
        let rc_branch = rc_branch.clone();
        let rc_entry = rc_entry.clone();
        move |button| {
            if let Some(label) = button.label() {
                match label.as_str() {
                    "Create" => {let _ = VersionControlSystem::branch(BranchOptions::NewBranch(&rc_entry.text()));},
                    "Delete" => {let _ = VersionControlSystem::branch(BranchOptions::DeleteBranch(&rc_entry.text()));},
                    _ => {},
                }
            }

            rc_branch.remove_all();
            let _ = branches(&rc_branch);

            rc_entry.set_text("");
            button.set_sensitive(false);
            
        }
    });
}

pub fn handle_button_select_branch(interface: &RustInterface) {
    interface.select_branch.connect_changed({
        //let version = version.clone();
        move |combo_box|{
            if let Some(branch) = combo_box.active_text(){
                let _ = VersionControlSystem::checkout(CheckoutOptions::ChangeBranch(&branch.to_string()));
            }
        }
    });
}

pub fn handle_buttons_repository(interface: &RustInterface, button_repo: &gtk::Button) {
    let rc_repo = interface.select_repository.clone();
    let rc_entry = interface.repository_entry.clone();
    button_repo.connect_clicked({
        //let version = version.clone();
        let rc_repo = rc_repo.clone();
        let rc_entry = rc_entry.clone();
        move |button| {
            if let Some(label) = button.label() {
                match label.as_str() {
                    "Create" => {let _ = VersionControlSystem::write_bdd_of_repositories(Path::new(&rc_entry.text().to_string()));},
                    "Delete" => {},
                    _ => {},
                }
            }

            rc_repo.remove_all();
            let _ = repositories( &rc_repo);

            rc_entry.set_text("");
            button.set_sensitive(false);
            
        }
    });
}

pub fn handle_button_select_repository(interface: &RustInterface) {
    interface.select_repository.connect_changed({
        //let version = version.clone();
        move |combo_box|{
            //let mut version = version.borrow_mut();
            if let Some(repository) = combo_box.active_text(){
                let _ = VersionControlSystem::init(Path::new(&repository.to_string()), Vec::new());
            }
        }
    });
}

pub fn handle_commit_button(interface: &RustInterface) {
    //let version = vcs.clone();
    let rc_entry = interface.message.clone();
    interface.message_ok.connect_clicked({
        let rc_entry = rc_entry.clone();
        //let version = version.clone();
        move |button| {
            let _ = VersionControlSystem::commit(rc_entry.text().to_string());

            rc_entry.set_text("");
            button.set_sensitive(false);
        }
    });
}


pub fn handle_terminal(interface: &RustInterface) {
    let rc_entry = interface.command_entry.clone();
    let rc_box = interface.command_box.clone();
    
    interface.enter.connect_clicked({
        //let version = version.clone();
        let rc_entry = rc_entry.clone();
        move |button| {
            rc_box.foreach(|child| {
                rc_box.remove(child);
            });
            
            let result = handler_command(&rc_entry.text());
            let label = gtk::Label::new(Some(&result));
            label.set_visible(true);
            label.set_xalign(2.5);
            label.set_yalign(2.5);
            rc_box.add(&label);
            rc_box.set_visible(true);

            rc_entry.set_text("");
            button.set_sensitive(false);
            
        }
    });
}

pub fn handle_rm_button(interface: &RustInterface) {
    let rm_entry = interface.rm_entry.clone();
    interface.rm_enter.connect_clicked({
        let rm_entry1 = rm_entry.clone();
        //let version1 = version.clone();
        move |button| {
            
            let binding = rm_entry1.text();

            if binding.ends_with("/"){
                let _ = handler_rm(format!("git rm -r {}",rm_entry1.text()));
            }
            else{
                let _ = handler_rm(format!("git rm {}",rm_entry1.text()));
            }
            
            rm_entry1.set_text("");
            button.set_sensitive(false);
        }
    });
}