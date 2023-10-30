use std::{cell::RefCell, rc::Rc};

use crate::{vcs::version_control_system::VersionControlSystem, handlers::commands::handler_command};

use super::{interface::RustInterface, handler_button::{handle_buttons_branch, handle_button_select_branch, handle_commit_button,  handle_buttons_repository, handle_select_repository}, draw::changes_and_staging_area};
use gtk::prelude::*;

pub fn handle_commit(interface: &RustInterface, vcs: &VersionControlSystem){
    let box_window = interface.box_window.clone();
    let dialog = interface.commit_dialog.clone();
    
    let rc_ok = interface.message_ok.clone();

    interface.message_ok.set_sensitive(false);
    
    interface.message.connect_changed({
        move |e| {
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
    handle_commit_button(interface, vcs);
    
}

pub fn handle_branch(interface: &RustInterface, vcs: &VersionControlSystem) {
    let dialog = interface.branch_dialog.clone();

    let rc_create = interface.create_branch.clone();
    let rc_delete = interface.delete_branch.clone();

    interface.create_branch.set_sensitive(false);
    interface.delete_branch.set_sensitive(false);

    interface.dialog_entry.connect_changed({  
        move |e| {
        rc_create.set_sensitive(!e.text().is_empty());
        rc_delete.set_sensitive(!e.text().is_empty());
    }});

    interface.branch_button.connect_clicked(
        move |_| {
            dialog.run();
            dialog.hide();
        }
    );

    handle_buttons_branch(interface,&interface.create_branch,vcs);
    handle_buttons_branch(interface,&interface.delete_branch,vcs);
    handle_button_select_branch(interface, vcs);

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

pub fn handle_log(interface: &RustInterface, vcs: &VersionControlSystem) {

    let version = Rc::new(RefCell::new(vcs.clone()));

    interface.log_dialog.set_title("Log information");
    let dialog = interface.log_dialog.clone();
    let log_box = interface.log_box.clone();
    interface.log.connect_clicked({
        let version = version.clone();
        move |_| {
            let version = version.borrow_mut();

            log_box.foreach(|child| {
                log_box.remove(child);
            });
            if let Ok(log) = version.log() {
                let label = gtk::Label::new(Some(&log));
                label.set_visible(true);
                label.set_xalign(2.5);
                label.set_yalign(2.5);
                log_box.add(&label);
            }
            dialog.run();
            dialog.hide();
        }
        
    });

    interface.close_log.connect_clicked({
        let dialog_2 = interface.log_dialog.clone();
        move |_| {
            dialog_2.hide();
    }});

}


pub fn handle_repository(interface: &RustInterface, vcs: &VersionControlSystem) {
    let dialog = interface.repository_dialog.clone();

    
    let rc_create = interface.create_repository.clone();
    let rc_delete = interface.delete_repository.clone();

    interface.create_repository.set_sensitive(false);
    interface.delete_repository.set_sensitive(false);

    interface.repository_entry.connect_changed({  
        move |e| {
        rc_create.set_sensitive(!e.text().is_empty());
        rc_delete.set_sensitive(!e.text().is_empty());
    }});

    interface.repository_button.connect_clicked(
        move |_| {
            dialog.run();
            dialog.hide();
        }
    );

    handle_buttons_repository(interface, &interface.create_repository, vcs);
    handle_buttons_repository(interface, &interface.delete_repository, vcs);
    handle_select_repository(interface,vcs);
    

}


pub fn handle_command(interface: &RustInterface, vcs: &VersionControlSystem) {
    let dialog = interface.command_dialog.clone();

    let version: Rc<RefCell<VersionControlSystem>> = Rc::new(RefCell::new(vcs.clone()));
    let rc_box = interface.command_box.clone();
    let rc_enter = interface.enter.clone();
    let rc_entry = Rc::new(RefCell::new(interface.command_entry.clone()));
    interface.enter.set_sensitive(false);

    interface.command_entry.connect_changed({  
        move |e| {
        rc_enter.set_sensitive(!e.text().is_empty());
        
    }});

    interface.enter.connect_clicked({
        let version = version.clone();
        let rc_entry = rc_entry.clone();
        move |button| {
            let mut version = version.borrow_mut();
            let rc_entry = rc_entry.borrow_mut();
            rc_box.foreach(|child| {
                rc_box.remove(child);
            });
            let result = handler_command(&mut version, &rc_entry.text());
            let label = gtk::Label::new(Some(&result));
            label.set_visible(true);
            label.set_xalign(2.5);
            label.set_yalign(2.5);
            rc_box.add(&label);

            rc_entry.set_text("");
            button.set_sensitive(false);
            
            dialog.run();
            dialog.hide();
        }
    });

    interface.command_close.connect_clicked({
        let dialog_2 = interface.command_dialog.clone();
        move |_| {
            dialog_2.hide();
    }});

}