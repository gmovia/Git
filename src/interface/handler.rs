
use std::collections::HashMap;
use crate::{vcs::{version_control_system::VersionControlSystem, entities::{conflict::Conflict, change::{write_changes, read_changes}}}, constants::constants::{CURRENT, INCOMING, BOTH}};

use super::{interface::RustInterface, handler_button::{handle_buttons_branch, handle_button_select_branch, handle_commit_button,  handle_buttons_repository, handle_select_repository, handle_rm_button, handle_terminal}, draw::changes_and_staging_area};
use gtk::{prelude::*, Button, Scrollbar, Adjustment};


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
    let version = vcs.clone();
    let rc_grid = interface.grid.clone();
    let rc_add = interface.box_window.clone();
    interface.status.connect_clicked({
        let version = version.clone();
        let rc_grid = rc_grid.clone();
        let rc_add = rc_add.clone();
        move |_| {
            let _ = changes_and_staging_area(&version, &rc_grid, &rc_add);
        }
    });
}

pub fn handle_log(interface: &RustInterface, vcs: &VersionControlSystem) {

    let version = vcs.clone();
    interface.log_dialog.set_title("Log information");
    let dialog = interface.log_dialog.clone();
    let log_box = interface.log_box.clone();

    interface.log.connect_clicked({
        let version = version.clone();
        move |_| {

            log_box.foreach(|child| {
                log_box.remove(child);
            });
            if let Ok(log) = version.log() {
                let label = gtk::Label::new(Some(&log));
                label.set_visible(true);
                label.set_xalign(2.5);
                label.set_yalign(2.5);
                log_box.add(&label);
                let scroll = Scrollbar::new(gtk::Orientation::Vertical, None::<&Adjustment>);
                scroll.set_visible(true);
                //scroll.policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
                log_box.add(&scroll);
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

    handle_terminal(interface, vcs);
}

pub fn handle_rm(interface: &RustInterface, vcs: &VersionControlSystem) {
    let rm_dialog = interface.rm_dialog.clone();

    let rm_enter = interface.rm_enter.clone();

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

    handle_rm_button(interface, vcs);

}

pub fn handle_merge(interface: &RustInterface, vcs: &VersionControlSystem) { //FALTA VER SI EL FOR ANDA PORQUE TODAVIA NO ESTA IMPLEMENTADO LO DE CONFLICTOS

    let merge_dialog = interface.merge_dialog.clone();
    let button_merge = interface.merge.clone();
    let version = vcs.clone();
    let version1 = vcs.clone();
    let m_entry = interface.merge_entry.clone();
    let m_changes = interface.merge_changes.clone();
    let button_resolve = interface.resolve.clone();
    let merge_grid_clone = interface.merge_grid.clone();
    let apply_clone = interface.apply_merge.clone();
    let b_dialog = interface.both_dialog.clone();
    let ok = interface.both_ok.clone();
    let b_box = interface.both_box.clone();
    let b_text = interface.both_text.clone();

    interface.merge.set_sensitive(false);
    interface.apply_merge.set_visible(false);
    interface.apply_merge.set_sensitive(true);

    interface.merge_entry.connect_changed({
        move |e| {
            button_merge.set_sensitive(!e.text().is_empty());
        }
    });
    
    interface.merge.connect_clicked({  //TRATAR DE DIVIDIRLO EN FUNCIONES
        let version = version.clone();
        let m_changes = m_changes.clone();
        let apply_c = apply_clone.clone();
        let m_entry = m_entry.clone();
        move |button| {
            m_changes.foreach(|child| {
                m_changes.remove(child);
            });
            if let Ok(conflicts) = version.merge(&m_entry.text()){
                if conflicts.len() == 0 {
                    add_message(&m_changes, &"Merged successfully".to_string());
                    button_resolve.set_sensitive(false);
                    //button_resolve.set_visible(false);
                }
                else{
                    add_message(&m_changes, &"Conflicts need to be resolve".to_string());
                    button_resolve.set_sensitive(true);
                    button_resolve.connect_clicked({
                        let version1 = version1.clone();
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
                                if let (Ok(cat_current), Ok(cat_incoming)) = (version1.cat_file(&value.change_current.hash),version1.cat_file(&value.change_branch.hash)) {
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
                                    add_current(&version1,&current, &value);
                                    add_incoming(&version1,&incoming, &value);
                                    add_both(&version1, &both, &value,&labels,&b_dialog,&b_box,&b_text,&ok);
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
                        let version2 = version1.clone();
                        let m_entry1 = m_entry.clone();
                        let m_box = m_changes.clone();
                        move |button|{
                            if let Ok(list_conflicts) = read_changes(&version2){
                                let _ = version2.resolve_conflicts(&m_entry1.text(), list_conflicts);
                            }
                            m_box.foreach(|child| {
                                m_box.remove(child);
                            });
                            add_message(&m_box, &"Merged successfully".to_string());
                            button.set_sensitive(false);
                            //button.set_visible(false);
                        }
                    });   
                }
            }
            merge_dialog.run();
            merge_dialog.hide();

            m_entry.set_text("");
            button.set_sensitive(false);
        }
    });

}

fn add_message(m_changes: &gtk::Box, message: &String) {
    let label = gtk::Label::new(Some(message));
    label.set_visible(true);
    label.set_xalign(0.5);
    label.set_yalign(0.5);
    m_changes.add(&label);
}

pub fn add_current(vcs: &VersionControlSystem, button: &gtk::Button, conflict: &Conflict) {
    let conflict_c = conflict.clone();
    let version = vcs.clone();
    button.connect_clicked({
        let conflict = conflict_c.clone();
        let version = version.clone();
        move |_| {
            let conflict = Conflict { file: conflict.file.clone(), change_current: conflict.change_current.clone(), change_branch: conflict.change_branch.clone(), resolved: CURRENT.to_string() };
            let _ = write_changes(&version, &conflict);

    }});
    
}

pub fn add_incoming(vcs: &VersionControlSystem, button: &gtk::Button, conflict: &Conflict) {
    let conflict_c = conflict.clone();
    let version = vcs.clone();
    button.connect_clicked({
        let conflict = conflict_c.clone();
        let version = version.clone();
        move |_| {
            let conflict = Conflict { file: conflict.file.clone(), change_current: conflict.change_current.clone(), change_branch: conflict.change_branch.clone(), resolved: INCOMING.to_string() };
            let _ = write_changes(&version,&conflict);

    }});
    
}

pub fn add_both(vcs: &VersionControlSystem, button: &gtk::Button, conflict: &Conflict,labels: &gtk::Label, dialog: &gtk::Dialog, both_box: &gtk::Box, both_text: &gtk::TextView, both_ok: &gtk::Button) {
    let conflict_c = conflict.clone();
    let version = vcs.clone();
    let dialog = dialog.clone();
    let labels = labels.clone();
    let both_box = both_box.clone();
    let both_text = both_text.clone();
    let both_ok = both_ok.clone();
    button.connect_clicked({
        let conflict = conflict_c.clone();
        let version = version.clone();
        let dialog = dialog.clone();
        let labels = labels.clone();
        let both_box = both_box.clone();
        let both_ok = both_ok.clone();
        move |_| {
            //let conflict = Conflict { file: conflict.file.clone(), change_current: conflict.change_current.clone(), change_branch: conflict.change_branch.clone(), resolved: BOTH.to_string() };
            //let _ = write_changes(&version,&conflict);
            both_box.add(&labels);
            both_ok.connect_clicked({
                let both_text = both_text.clone();
                move |_| {
                    println!("{:?}",both_text);
                }
            });
            dialog.run();
            dialog.hide();
    }});
    
}


pub fn handle_clone(interface: &RustInterface, vcs: &VersionControlSystem) {
    let version = vcs.clone();
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
        let _version = version.clone();
        let info = info.clone();
        let c_entry = c_entry.clone();
        let fix_clone = fix_clone.clone();
        move |button| {
            
            //if let Ok(content) = version.clone(&c_entry.text()) {
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
            //}
            c_entry.set_text("");
            button.set_sensitive(false);
        }

    });
}



