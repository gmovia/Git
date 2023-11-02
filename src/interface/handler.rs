use std::{cell::RefCell, rc::Rc, collections::HashMap};

use crate::vcs::version_control_system::VersionControlSystem;

use super::{interface::RustInterface, handler_button::{handle_buttons_branch, handle_button_select_branch, handle_commit_button,  handle_buttons_repository, handle_select_repository, handle_rm_button, handle_terminal}, draw::changes_and_staging_area};
use gtk::{prelude::*, Button};

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
    let version = Rc::new(RefCell::new(vcs.clone()));
    let m_entry = interface.merge_entry.clone();
    let m_changes = interface.merge_changes.clone();
    //let mut conflicts:HashMap<&str, &str> = HashMap::new();
    let button_resolve = interface.resolve.clone();
    // conflicts.insert("file1.txt", "holahola");
    // conflicts.insert("file2.txt", "holis");
    // conflicts.insert("file3.txt", "hello");
    // conflicts.insert("file4.txt", "buenas");
    //let conflicts_clone = Rc::new(RefCell::new(conflicts.clone()));
    
    interface.merge.set_sensitive(false);

    interface.merge_entry.connect_changed({
        move |e| {
            button_merge.set_sensitive(!e.text().is_empty());
        }
    });

    interface.merge.connect_clicked({  //TRATAR DE DIVIDIRLO EN FUNCIONES
        let version = version.clone();
        let m_changes = m_changes.clone();
        move |button| {
            let version = version.borrow_mut();
            m_changes.foreach(|child| {
                m_changes.remove(child);
            });
            if let Ok(conflicts) = version.merge(&m_entry.text()){
                if conflicts.len() == 0 {
                    let label = gtk::Label::new(Some(&"Merge successfully"));
                    label.set_visible(true);
                    label.set_xalign(0.5);
                    label.set_yalign(0.5);
                    m_changes.add(&label);
                    button_resolve.set_sensitive(false);
                }else{
                    let label = gtk::Label::new(Some(&"Conflicts need to be resolve"));
                    label.set_visible(true);
                    label.set_xalign(0.5);
                    label.set_yalign(0.5);
                    m_changes.add(&label);}
                    button_resolve.connect_clicked({
                        let conflicts_clone = conflicts.clone();
                       move |_| {
                        for (key, value) in &conflicts_clone{
                            let set_label_current = format!("{}\n             {:?}",key,value);
                            let label_current = gtk::Label::new(Some(&set_label_current));
                            label_current.set_visible(true);
                            label_current.set_xalign(0.5);
                            label_current.set_yalign(0.5);
                            let set_label_incoming = format!("{}\n            {:?}",key,value);
                            let label_incoming = gtk::Label::new(Some(&set_label_incoming));
                            label_incoming.set_visible(true);
                            label_incoming.set_xalign(0.5);
                            label_incoming.set_yalign(0.5);
                            let blank = gtk::Label::new(Some(&""));
                            blank.set_visible(true);
                            blank.set_xalign(0.5);
                            blank.set_yalign(0.5);
                            let conflict_dialog = gtk::Dialog::new();  // Crea una nueva caja de diálogo por conflicto
                            conflict_dialog.set_title("Conflict Details");
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
                            if let (Some(both_label), Some(current_label), Some(incoming_label)) = (&both.label(), &current.label(), &incoming.label()) {
                                conflict_dialog.add_button(both_label.as_str(), gtk::ResponseType::Accept);
                                conflict_dialog.add_button(current_label.as_str(), gtk::ResponseType::Accept);
                                conflict_dialog.add_button(incoming_label.as_str(), gtk::ResponseType::Accept);
                            } 
                            
                            let content_area = conflict_dialog.content_area();
                            content_area.add(&blank);
                            content_area.add(&label_current);
                            content_area.add(&blank);
                            content_area.add(&label_incoming);
                            
                            conflict_dialog.run();
                            conflict_dialog.hide();
                        }
                            // if let (Ok(cat_current), Ok(cat_incoming)) = (version.cat_file(&value.change_current.hash),version.cat_file(&value.change_branch.hash)) {
                            //     let set_label_current = format!("{}\n             {}",key,cat_current);
                            //     let label_current = gtk::Label::new(Some(&set_label_current));
                            //     label_current.set_visible(true);
                            //     label_current.set_xalign(0.5);
                            //     label_current.set_yalign(0.5);
                            //     let set_label_incoming = format!("{}\n            {}",key,cat_incoming);
                            //     let label_incoming = gtk::Label::new(Some(&set_label_incoming));
                            //     label_incoming.set_visible(true);
                            //     label_incoming.set_xalign(0.5);
                            //     label_incoming.set_yalign(0.5);
                            //     let blank = gtk::Label::new(Some(&""));
                            //     blank.set_visible(true);
                            //     blank.set_xalign(0.5);
                            //     blank.set_yalign(0.5);
                            //     // SE ACTUALIZA LA VENTANA MERGE DIALOG POR CADA CONFLICTO
                            //     let conflict_dialog = gtk::Dialog::new();  // Crea una nueva caja de diálogo por conflicto
                            //     conflict_dialog.set_title("Conflict Details");
                            //     conflict_dialog.add_button("Accept both", gtk::ResponseType::Accept);
                            //     conflict_dialog.add_button("Accept current", gtk::ResponseType::Accept);
                            //     conflict_dialog.add_button("Accept incoming", gtk::ResponseType::Accept);
                            //     let content_area = conflict_dialog.content_area();
                            //     content_area.add(&blank);
                            //     content_area.add(&label_current);
                            //     content_area.add(&blank);
                            //     content_area.add(&label_incoming);
                            //     button_both.set_sensitive(true);
                            //     button_current.set_sensitive(true);
                            //     button_incoming.set_sensitive(true);
                            //     conflict_dialog.run();
                            //     conflict_dialog.hide();
                            // }
                            
                        }
                    //}
                       //} 
                    });
                }
                
            //}    
                
            merge_dialog.run();
            merge_dialog.hide();

            m_entry.set_text("");
            button.set_sensitive(false);
        }
    });



    

}
