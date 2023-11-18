use std::{fs::{OpenOptions, self}, io::Write, path::Path};
use crate::{vcs::{version_control_system::VersionControlSystem, entities::{conflict::Conflict, change::{write_changes, read_changes, Change}}, commands::hash_object::WriteOption, files::current_repository::CurrentRepository}, constants::constants::{CURRENT, INCOMING, BOTH, BLOB_CODE}};

use super::{interface::RustInterface, handler_button::{handle_buttons_branch, handle_button_select_branch, handle_commit_button,  handle_buttons_repository, handle_rm_button, handle_terminal, handle_button_select_repository, handle_ls_files_buttons}, draw::{changes_and_staging_area, draw_message, draw_error}};
use gtk::{prelude::*, Button};

pub fn handle_other_commands(interface: &RustInterface) {

    let dialog = interface.others_dialog.clone();

    interface.other_commands.connect_clicked({
        move |_| {
            dialog.run();
            dialog.hide();
        }
    });

    handle_ls_files(interface);
    handle_ls_tree(interface);
    handle_check_ignore(interface);

    interface.others_close.connect_clicked({
       let dialog2 = interface.others_dialog.clone(); 
       move |_| {
            dialog2.hide();
       } 
    });
    
}

pub fn handle_repository(interface: &RustInterface) {
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

    handle_buttons_repository(interface,&interface.create_repository);
    handle_buttons_repository(interface,&interface.delete_repository);
    handle_button_select_repository(interface);
}

pub fn handle_commit(interface: &RustInterface){
    let box_window = interface.grid_staging.clone();
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
    handle_commit_button(interface);
    
}

pub fn handle_branch(interface: &RustInterface) {
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

    handle_buttons_branch(interface,&interface.create_branch);
    handle_buttons_branch(interface,&interface.delete_branch);
    handle_button_select_branch(interface);

}


pub fn handle_status(interface: &RustInterface) {

    let rc_grid = interface.grid.clone();
    let rc_add = interface.grid_staging.clone();
    interface.status.connect_clicked({

        let rc_grid = rc_grid.clone();
        let rc_add = rc_add.clone();
        move |_| {
            let _ = changes_and_staging_area(&rc_grid, &rc_add);
        }
    });
}

pub fn handle_log(interface: &RustInterface) {


    interface.log_dialog.set_title("Log information");
    let dialog = interface.log_dialog.clone();
    let log_box = interface.log_box.clone();

    interface.log.connect_clicked({

        move |_| {

            log_box.foreach(|child| {
                log_box.remove(child);
            });
            if let Ok(log) = VersionControlSystem::log() {
                draw_message(&log_box, &log, 0.5);
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


pub fn handle_command(interface: &RustInterface) {

    let terminal_dialog = interface.terminal_dialog.clone();
    let rc_enter = interface.enter.clone();

    interface.enter.set_sensitive(false);

    interface.command_entry.connect_changed({  
        move |e| {
        rc_enter.set_sensitive(!e.text().is_empty());
        
    }});

    interface.terminal.connect_clicked({
        move |_| {
            terminal_dialog.run();
            terminal_dialog.hide();
        }
    });

    handle_terminal(interface);
}

pub fn handle_rm(interface: &RustInterface) {
    let rm_dialog = interface.rm_dialog.clone();
    let rm_enter = interface.rm_enter.clone();

    let _err_dialog = interface.error_dialog.clone();
    let _err_box = interface.error_box.clone();

    interface.rm_enter.set_sensitive(false);

    interface.rm_entry.connect_changed({  
        move |e| {
        rm_enter.set_sensitive(!e.text().is_empty());
    }});

    interface.rm.connect_clicked({
        move |_| {
            rm_dialog.run();
            rm_dialog.hide();
        }
    });

    handle_rm_button(interface);

}

pub fn handle_merge(interface: &RustInterface) {

    let merge_dialog = interface.merge_dialog.clone();
    let button_merge = interface.merge.clone();
    let m_entry = interface.merge_entry.clone();
    let m_changes = interface.merge_changes.clone();
    let button_resolve = interface.resolve.clone();
    let merge_grid_clone = interface.merge_grid.clone();
    let apply_clone = interface.apply_merge.clone();
    let b_dialog = interface.both_dialog.clone();
    let ok = interface.both_ok.clone();
    let b_box = interface.both_box.clone();
    let b_text = interface.both_text.clone();

    let errors_tuple = (interface.error_dialog.clone(),interface.error_box.clone());

    let rc_tuple = errors_tuple.clone();
    //let err_dialog = interface.error_dialog.clone();
    //let err_box = interface.error_box.clone();

    interface.merge.set_sensitive(false);
    interface.apply_merge.set_visible(false);
    interface.apply_merge.set_sensitive(true);

    interface.merge_entry.connect_changed({
        move |e| {
            button_merge.set_sensitive(!e.text().is_empty());
        }
    });
    
    interface.merge.connect_clicked({

        let m_changes = m_changes.clone();
        let apply_c = apply_clone.clone();
        let m_entry = m_entry.clone();
        move |button| {
            m_changes.foreach(|child| {
                m_changes.remove(child);
            });
            if let Ok(conflicts) = VersionControlSystem::merge(&m_entry.text()){
                if conflicts.len() == 0 {
                    draw_message(&m_changes, &"Merged successfully".to_string(), 0.5);
                    button_resolve.set_sensitive(false);
                }
                else{
                    draw_message(&m_changes, &"Conflicts need to be resolve".to_string(), 0.5);
                    button_resolve.set_visible(true);
                    button_resolve.set_sensitive(true);
                    button_resolve.connect_clicked({
                        let m_box = m_changes.clone();
                        let m_grid = merge_grid_clone.clone();
                        let apply_c = apply_c.clone();
                        let b_dialog = b_dialog.clone();
                        let ok = ok.clone();
                        let b_box = b_box.clone();
                        let b_text = b_text.clone();
                        move |button| {
                            m_box.foreach(|child| {
                                m_box.remove(child);
                            });
                            let mut index = 0;
                            for (key, value) in &conflicts {
                                if let (Ok(cat_current), Ok(cat_incoming)) = (VersionControlSystem::cat_file(&value.change_current.hash),VersionControlSystem::cat_file(&value.change_branch.hash)) {
                                    let set_label_current = format!("{}\n             {}\n",key,cat_current);
                                    let set_label_incoming = format!("\n{}\n            {}\n",key,cat_incoming);
                                    let set_format = set_label_current + &set_label_incoming;
                                    let labels = gtk::Label::new(Some(&set_format));
                                    labels.set_visible(true);
                                    labels.set_xalign(2.0);
                                    labels.set_yalign(2.0);
                                    let both = Button::builder()
                                    .margin_start(10)
                                    .label("Accept both")
                                    .build();
                                    let current =Button::builder()
                                    .margin_start(10)
                                    .label("Accept current")
                                    .build();
                                    let incoming = Button::builder()
                                    .margin_start(10)
                                    .label("Accept incoming")
                                    .build();
                                    both.set_visible(true);
                                    current.set_visible(true);
                                    incoming.set_visible(true);
                                    m_grid.attach(&labels, 0, index as i32, 1, 1);
                                    m_grid.attach(&both, 1, index as i32, 1, 1);
                                    m_grid.attach(&current, 2, index as i32, 1, 1);
                                    m_grid.attach(&incoming, 3, index as i32, 1, 1);
                                    index += 1;
                                    add_current(&current, &value);
                                    add_incoming(&incoming, &value);
                                    add_both(&both, &value,&b_dialog,&b_box,&b_text,&ok);
                                } 
                                m_box.add(&m_grid);
                            }
                            button.set_sensitive(false);
                            button.set_visible(false);
                            apply_c.set_visible(true);
                            apply_c.set_sensitive(true);

                        }
                    });
                    apply_c.connect_clicked({
                        let m_entry1 = m_entry.clone();
                        let m_box = m_changes.clone();
                        move |button|{
                            if let Ok(list_conflicts) = read_changes(){
                                let _ = VersionControlSystem::resolve_conflicts(&m_entry1.text(), list_conflicts);
                            }
                            m_box.foreach(|child| {
                                m_box.remove(child);
                            });
                            draw_message(&m_box, &"Merged successfully".to_string(), 0.5);
                            button.set_sensitive(false);
                        }
                    });   
                }
                merge_dialog.run();
                merge_dialog.hide();

                m_entry.set_text("");
                button.set_sensitive(false);
            }else {
                draw_error(rc_tuple.clone(), &"    ERROR! BRANCH NOT FOUND...  ".to_string(), &m_entry);
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



pub fn add_current(button: &gtk::Button, conflict: &Conflict) {
    let conflict_c = conflict.clone();
    button.connect_clicked({
        let conflict = conflict_c.clone();
        move |_| {
            let conflict = Conflict { file: conflict.file.clone(), change_current: conflict.change_current.clone(), change_branch: conflict.change_branch.clone(), resolved: CURRENT.to_string() };
            let _ = write_changes( &conflict);

    }});
    
}

pub fn add_incoming( button: &gtk::Button, conflict: &Conflict) {
    let conflict_c = conflict.clone();
    button.connect_clicked({
        let conflict = conflict_c.clone();
        move |_| {
            let conflict = Conflict { file: conflict.file.clone(), change_current: conflict.change_current.clone(), change_branch: conflict.change_branch.clone(), resolved: INCOMING.to_string() };
            let _ = write_changes(&conflict);

    }});
    
}

pub fn add_both(button: &gtk::Button, conflict: &Conflict, dialog: &gtk::Dialog, both_box: &gtk::Box, both_text: &gtk::TextView, both_ok: &gtk::Button) {
    let conflict_c = conflict.clone();
    let dialog = dialog.clone();
    let both_box = both_box.clone();
    let both_text = both_text.clone();
    let both_ok = both_ok.clone();
    button.connect_clicked({
        let conflict_c = conflict_c.clone();
        let dialog = dialog.clone();
        let both_box = both_box.clone();
        let both_ok = both_ok.clone();
        move |_| {
            both_box.add(&both_text);
            both_ok.connect_clicked({
                let conflict_c = conflict_c.clone();
                let both_text = both_text.clone();
                move |_| {
                    if let Some(result) = both_text.buffer(){
                        if let Some(result) = result.text(&result.start_iter(), &result.end_iter(), false){
                            if let Ok(current) = CurrentRepository::read() {
                                let temp_path = current.join("temp2");
                                if let Ok(mut temp_file) = OpenOptions::new().write(true).create(true).append(true).open(&temp_path) {
                                    let _ = temp_file.write_all(result.as_bytes());
                                    if let Ok(hash) = VersionControlSystem::hash_object(&temp_path, WriteOption::Write, BLOB_CODE) {
                                        let _ = fs::remove_file(temp_path);
                                        let change_current = Change { file: conflict_c.file.to_string(), hash: hash.clone(), state: conflict_c.change_current.state.clone() };
                                        let change_branch = Change { file: conflict_c.file.to_string(), hash: hash.clone(), state: conflict_c.change_branch.state.clone() };
                                        let conflict = Conflict { file: conflict_c.file.clone(), change_current: change_current, change_branch: change_branch, resolved: BOTH.to_string() };
                                        let _ = write_changes(&conflict);
                                    }
                                }
                            }
                        }
                    }
                }
            });
            
            dialog.run();
            dialog.hide();
    }});
    
}

pub fn handle_ls_files(interface: &RustInterface) {
    let files_dialog = interface.ls_files_dialog.clone();
    let rc_box = interface.selection_box.clone();

    interface.selection_box.set_visible(false);

    interface.files.connect_clicked({
        let rc_box = rc_box.clone();
        move |_| {
            rc_box.foreach(|child| {
                rc_box.remove(child);
            });
            files_dialog.run();
            files_dialog.hide();
        }
    });

    handle_ls_files_buttons(interface,&interface.all);
    handle_ls_files_buttons(interface,&interface.o);
    handle_ls_files_buttons(interface,&interface.m);
    handle_ls_files_buttons(interface,&interface.c);
    handle_ls_files_buttons(interface,&interface.d);

    interface.close_files.connect_clicked({
        let dialog_2 = interface.ls_files_dialog.clone();
        move |_| {
            dialog_2.hide();
    }});

}

pub fn handle_ls_tree(interface: &RustInterface) {
    let tree_dialog = interface.ls_tree_dialog.clone();
    let rc_box = interface.tree_box.clone();
    let rc_entry = interface.tree_branch_entry.clone();

    let errors_tuple = (interface.error_dialog.clone(),interface.error_box.clone());

    let rc_tuple = errors_tuple.clone();

    interface.tree_box.set_visible(false);

    let apply = interface.apply_tree.clone();

    interface.apply_tree.set_sensitive(false);

    interface.tree_branch_entry.connect_changed({  
        move |e| {
        apply.set_sensitive(!e.text().is_empty());
    }});

    interface.ls_tree.connect_clicked({
        let rc_box = rc_box.clone();
        let rc_entry = rc_entry.clone();
        move |_| {
            rc_box.foreach(|child| {
                rc_box.remove(child);
            });
            tree_dialog.run();
            tree_dialog.hide();
            rc_entry.set_text("");
        }
    });

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

    interface.close_tree.connect_clicked({
        let dialog_2 = interface.ls_tree_dialog.clone();
        move |_| {
            dialog_2.hide();
    }});

    interface.error_close.connect_clicked({
        let err_dialog_2 = errors_tuple.0.clone();
        move |_| {
            err_dialog_2.hide();
        }
    });

}

pub fn handle_check_ignore(interface: &RustInterface) {
    let dialog = interface.ignore_dialog.clone();
    let ig_entry = interface.check_ignore_entry.clone();
    let ch_button = interface.check_button.clone();
    let ch_box = interface.check_ignore_box.clone();

    interface.check_button.set_sensitive(false);

    interface.check_ignore_entry.connect_changed({
       move |e| {
            ch_button.set_sensitive(!e.text().is_empty());
       } 
    });

    interface.check_ignore.connect_clicked({
        move |_| {
            dialog.run();
            dialog.hide();
        }
    });

    interface.check_button.connect_clicked({
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

    interface.close_ignore.connect_clicked({
        let dialog2 = interface.ignore_dialog.clone();
        move |_| {
            dialog2.hide();
        } 
    });
}




/* 
pub fn handle_clone(interface: &RustInterface) {
    
    let c_entry = interface.clone_entry.clone();
    let clone_button = interface.clone.clone();
    let info = interface.info_clone.clone();
    let fix_clone = interface.fix.clone();
    
    interface.clone.set_sensitive(false);
    interface.info_clone.set_visible(false);
    
    
    interface.clone_entry.connect_changed({
        move |e| {
            clone_button.set_sensitive(!e.text().is_empty());
        }
    });
    
    interface.clone.connect_clicked({
        let info = info.clone();
        let c_entry = c_entry.clone();
        let fix_clone = fix_clone.clone();
        move |button| {
            
            if let Ok(_) = VersionControlSystem::git_clone(format!("git clone {}",(&c_entry.text()).to_string())) {
                let label = gtk::Label::new(Some(&c_entry.text()));
                label.set_visible(true);
                label.set_xalign(0.5);
                label.set_yalign(0.5);
                let close = Button::builder()
                .label("close")
                .build();
                close.set_visible(true);
                fix_clone.add(&label);
                info.add(&close);
                info.add(&fix_clone);
                info.set_visible(true);
                close.connect_clicked({
                    let info = info.clone();
                    move |_| {
                        info.foreach({|child|{
                            info.remove(child);
                        }});
                    }
                });
            }
            c_entry.set_text("");
            button.set_sensitive(false);
        }
    });
}
*/

/* 
pub fn handle_fetch(interface: &RustInterface) {
    interface.fetch.connect_clicked({
        move |_| {
            if let Ok(current) = VersionControlSystem::read_current_repository() {
                let _ = VersionControlSystem::fetch(format!("git fetch {}",current.display().to_string()));
            }
        }
    });
}

pub fn handle_push(interface: &RustInterface) {
    interface.push.connect_clicked({
        move |_| {
            if let Ok(current) = VersionControlSystem::read_current_repository() {
                let _ = VersionControlSystem::push();
            }
        } 
    });
}

pub fn handle_pull(interface: &RustInterface) {
    
    interface.push.connect_clicked({
        move |_| {
            let _ = VersionControlSystem::pull();
        } 
    });
}
*/