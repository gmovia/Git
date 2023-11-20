
use std::path::Path;

use crate::{vcs::{version_control_system::VersionControlSystem, commands::{branch::BranchOptions, checkout::CheckoutOptions, ls_files::LsFilesOptions}, files::repositories::Repositories}, handlers::{rm::handler_rm, commands::handler_command}, constants::constants::{RESPONSE_OK_RM, ERR_NO_SUCH_OR_DIRECTORY}};

use super::{interface::RustInterface, draw::{branches, repositories, draw_message, draw_error}};

use gtk::prelude::*;


pub fn handle_buttons_branch(interface: &RustInterface, button_branch: &gtk::Button) {
    let rc_branch = interface.select_branch.clone();
    let rc_entry = interface.dialog_entry.clone();
    let rc_box = interface.branch_box.clone();

    let errors_tuple = (interface.error_dialog.clone(),interface.error_box.clone());
    let rc_tuple = errors_tuple.clone();

    button_branch.connect_clicked({
        let rc_branch = rc_branch.clone();
        let rc_entry = rc_entry.clone();
        move |button| {
            rc_box.foreach(|child| {
                rc_box.remove(child);
            });
            if let Some(label) = button.label() {
                match label.as_str() {
                    "Create" => {let _ = VersionControlSystem::branch(BranchOptions::NewBranch(&rc_entry.text()));
                                draw_message(&rc_box, &"     CREATED SUCCESSFULLY!    ".to_string(), 0.5);
                    },
                    "Delete" => {if let Ok(_) = VersionControlSystem::branch(BranchOptions::DeleteBranch(&rc_entry.text())){
                                    draw_message(&rc_box, &"     DELETED SUCCESSFULLY!    ".to_string(), 0.5);
                                }else{
                                    draw_error(rc_tuple.clone(), &"     CANNOT FOUND THE BRANCH...    ".to_string(), &rc_entry);
                                }
                    },
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
    let rc_branch = interface.select_branch.clone();
    button_repo.connect_clicked({
        let rc_repo = rc_repo.clone();
        let rc_entry = rc_entry.clone();
        move |button| {
            if let Some(label) = button.label() {
                match label.as_str() {
                    "Create" => {let _ = Repositories::write(Path::new(&rc_entry.text().to_string()));},
                    "Delete" => {let _ = Repositories::remove(Path::new(&rc_entry.text().to_string()));},
                    _ => {},
                }
            }
            rc_branch.remove_all();
            rc_repo.remove_all();
            let _ = repositories( &rc_repo);
            let _ = branches(&rc_branch);

            rc_entry.set_text("");
            button.set_sensitive(false);
            
        }
    });
}

pub fn handle_button_select_repository(interface: &RustInterface) {
    let rc_branch = interface.select_branch.clone();
    interface.select_repository.connect_changed({
        move |combo_box|{
            if let Some(repository) = combo_box.active_text(){
                let _ = VersionControlSystem::init(Path::new(&repository.to_string()), Vec::new());
            }
            rc_branch.remove_all();
            let _ = branches(&rc_branch);
        }
    });
}

pub fn handle_commit_button(interface: &RustInterface) {
    let rc_entry = interface.message.clone();
    interface.message_ok.connect_clicked({
        let rc_entry = rc_entry.clone();
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
    let rc_box = interface.rm_box.clone();

    let errors_tuple = (interface.error_dialog.clone(),interface.error_box.clone());
    let rc_tuple = errors_tuple.clone();

    interface.rm_enter.connect_clicked({
        let rm_entry1 = rm_entry.clone();
        move |button| {
            rc_box.foreach(|child| {
                rc_box.remove(child);
            });
            let binding = rm_entry1.text();

            if binding.ends_with("/"){
                let response = handler_rm(format!("git rm -r {}",rm_entry1.text()));
                if response == RESPONSE_OK_RM.to_string() {
                    draw_message(&rc_box, &"     DELETE SUCCESSFULLY!    ".to_string(), 0.5);
                    rm_entry.set_text("");
                    button.set_sensitive(false);
                }else if response == ERR_NO_SUCH_OR_DIRECTORY.to_string() {
                    draw_error(rc_tuple.clone(), &"      NO SUCH FILE OR DIRECTORY ...     ".to_string(), &rm_entry);
                }
            }
            else{
                let response = handler_rm(format!("git rm {}",rm_entry1.text()));
                if response == RESPONSE_OK_RM.to_string() {
                    draw_message(&rc_box, &"     DELETE SUCCESSFULLY!    ".to_string(), 0.5);
                    rm_entry.set_text("");
                    button.set_sensitive(false);
                }else if response == ERR_NO_SUCH_OR_DIRECTORY.to_string() {
                    draw_error(rc_tuple.clone(), &"      NO SUCH FILE OR DIRECTORY ...     ".to_string(), &rm_entry);
                }
            }
            rm_entry1.set_text("");
            button.set_sensitive(false);
        }
    });
}

pub fn handle_ls_files_buttons(interface: &RustInterface, button_file: &gtk::Button) {
    let rc_box = interface.selection_box.clone();
    button_file.connect_clicked({
        let rc_box = rc_box.clone();
       move |button| {
            rc_box.foreach(|child| {
                rc_box.remove(child);
            });
            if let Some(label) = button.label() {
                match label.as_str() {
                    "all" => {if let Ok(files) = VersionControlSystem::ls_files(LsFilesOptions::EverythingInVCS) {
                                for entry in files {
                                    let message = format!("{}\n",entry);
                                    draw_message(&rc_box, &message, 0.5);
                                }}},
                    "-o" => {if let Ok(files) = VersionControlSystem::ls_files(LsFilesOptions::OnlyUntracked) {
                                for entry in files {
                                    let message = format!("{}\n",entry);
                                    draw_message(&rc_box, &message, 0.5);
                                }}},
                    "-m" => {if let Ok(files) = VersionControlSystem::ls_files(LsFilesOptions::OnlyModified) {
                                for entry in files {
                                    let message = format!("{}\n",entry);
                                    draw_message(&rc_box, &message, 0.5);
                                }}},
                    "-d" => {if let Ok(files) = VersionControlSystem::ls_files(LsFilesOptions::OnlyDeleted) {
                                for entry in files {
                                    let message = format!("{}\n",entry);
                                    draw_message(&rc_box, &message, 0.5);
                                }}},
                    "-c" => {if let Ok(files) = VersionControlSystem::ls_files(LsFilesOptions::OnlyStaging) {
                                for entry in files {
                                    let message = format!("{}\n",entry);
                                    draw_message(&rc_box, &message, 0.5);
                                }}},
                    _ => {},
                }
            }
            rc_box.set_visible(true);
       } 
    });
}

pub fn handle_ls_tree_button(interface: &RustInterface) {
    let rc_box = interface.tree_box.clone();
    let rc_entry = interface.tree_branch_entry.clone();

    let errors_tuple = (interface.error_dialog.clone(),interface.error_box.clone());

    let rc_tuple = errors_tuple.clone();
    interface.apply_tree.connect_clicked({
        let rc_box = rc_box.clone();
        move |_| {
            rc_box.foreach(|child| {
                rc_box.remove(child);
            });
            if let Ok(information) = VersionControlSystem::ls_tree(&rc_entry.text().to_string()) {
                for entry in information {
                    let message = format!("{}\n",entry);
                    draw_message(&rc_box, &message, 0.0);
                }
                rc_box.set_visible(true);
                rc_entry.set_text("");
            }else {
                draw_error(rc_tuple.clone(), &"    ERROR! BRANCH NOT FOUND...  ".to_string(), &rc_entry);
            }
        }
    });

    interface.error_close.connect_clicked({
        let err_dialog_2 = errors_tuple.0.clone();
        move |_| {
            err_dialog_2.hide();
        }
    });
}

pub fn handle_check_ignore_button(interface: &RustInterface) {

    let ig_entry = interface.check_ignore_entry.clone();
    let ch_box = interface.check_ignore_box.clone();
    
    interface.check_button.connect_clicked({
        let ch_box = ch_box.clone();
        move |button| {
            ch_box.foreach(|child| {
                ch_box.remove(child);
            });
            if let Ok(response) = VersionControlSystem::check_ignore(Path::new(&ig_entry.text().to_string())){
                draw_message(&ch_box, &response, 0.5);
            }

            ig_entry.set_text("");
            button.set_sensitive(false);
        }
    });
}