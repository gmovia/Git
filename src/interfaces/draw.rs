use std::{path::Path, collections::HashMap};
use gtk::{prelude::*, Button, ComboBoxText};
use crate::{vcs::{version_control_system::VersionControlSystem, commands::branch::BranchOptions, files::repositories::Repositories}, handlers::branch};


pub fn branches(combo_box: &ComboBoxText) -> Result<(), std::io::Error>{
    let branches = VersionControlSystem::branch(BranchOptions::GetBranches)?;
    draw_branches(&branches, combo_box);
    Ok(())
}

pub fn repositories(combo_box: &ComboBoxText) -> Result<(), std::io::Error>{
    let repositories = Repositories::read()?;
    draw_repositories(&repositories, combo_box);
    Ok(())
}

pub fn changes_and_staging_area(grid: &gtk::Grid, grid_staging: &gtk::Grid) -> Result<(), std::io::Error>{
    grid.foreach(|child|{
        grid.remove(child);
    });
    
    grid_staging.foreach(|child|{
        grid_staging.remove(child);
    });

    let (untracked_files, changes_not_be_commited, changes_to_be_commited) = VersionControlSystem::status()?;

    let staging_area: Vec<String> = changes_to_be_commited.keys().cloned().collect();

    let mut changes = untracked_files.clone();
    changes.extend(changes_not_be_commited);

    draw_staging_area(&staging_area, grid_staging);
    draw_changes(&changes, grid, grid_staging);
    Ok(())
}

pub fn draw_changes(changes: &HashMap<String, String>, grid: &gtk::Grid, grid_staging: &gtk::Grid){

    //let mut index = 0;
    for (index,(path, state)) in changes.iter().enumerate() {
        let path_label = gtk::Label::new(Some(path));
        path_label.set_visible(true);
        path_label.set_xalign(2.0); 
        path_label.set_yalign(0.5); 

        path_label.style_context().add_class("custom-label-message");

        let state_label = gtk::Label::new(Some(state));
        state_label.set_visible(true);
        state_label.set_xalign(2.0); 
        state_label.set_yalign(0.5); 

        path_label.style_context().add_class("custom-add-label");

        if state == "CREATED"{
            state_label.style_context().add_class("custom-changes-label-created");
        }
        if state == "MODIFIED"{
            state_label.style_context().add_class("custom-changes-label-modified");
        }
        if state == "DELETED"{
            state_label.style_context().add_class("custom-changes-label-deleted");
        }

        let add_button = Button::builder()
        .margin_start(10)
        .label("+")
        .build();
        add_button.set_visible(true);

        
        add_button.style_context().add_class("custom-add-button");
        
        let reset_button = Button::builder()
        .margin_start(10)
        .label("-")
        .build();

        reset_button.set_visible(true);
        reset_button.style_context().add_class("custom-reset-button");

        grid.attach(&path_label, 0, index as i32, 1, 1);
        grid.attach(&state_label, 1, index as i32, 1, 1);
        grid.attach(&add_button, 2, index as i32, 1, 1);
        
        //index += 1;

        let path_clone = path.clone(); 
        let reset_button = reset_button.clone();
        let path_label = path_label.clone();
        add_button.connect_clicked({
            let reset_button = reset_button.clone();
            let rc_grid = grid.clone();
            let rc_add = grid_staging.clone();
            let path_label = path_label.clone();
            move |widget|{ 

                let _ = VersionControlSystem::add(Path::new(&path_clone)); 
                rc_grid.remove(widget);
                rc_grid.remove(&path_label);
                rc_grid.remove(&state_label);
                rc_add.attach(&path_label, 0, index as i32, 1, 1);
                rc_add.attach(&reset_button, 1, index as i32, 1, 1);
        }});

        let reset_button = reset_button.clone();
        let path_clone = path.clone(); 
        let path_label = path_label.clone();
        reset_button.connect_clicked({
            let rc_grid = grid_staging.clone();
            let path_clone = path_clone.clone(); 
            let path_label = path_label.clone();
            move |widget|{ 
                let _ = VersionControlSystem::reset(Path::new(&path_clone));
                rc_grid.remove(widget);
                rc_grid.remove(&path_label);
        }});
        
    }
}

pub fn draw_staging_area(staging_area: &[String], grid: &gtk::Grid){

    //let mut index = 0;
    for (index,path) in staging_area.iter().enumerate() {
        let label = gtk::Label::new(Some(path));
        label.set_visible(true);
        label.set_xalign(2.0);
        label.set_yalign(0.5);

        label.style_context().add_class("custom-label-message");

        let reset_button = Button::builder()
        .margin_start(10)
        .label("-")
        .build();

        reset_button.set_visible(true);
        reset_button.style_context().add_class("custom-reset-button");

        grid.attach(&label, 0, index as i32, 1, 1);
        grid.attach(&reset_button, 1, index as i32, 1, 1);
        
        //index += 1;
        let path_clone = path.clone(); 
        reset_button.connect_clicked({
            let rc_grid = grid.clone();
            move |widget|{ 
                let _ = VersionControlSystem::reset(Path::new(&path_clone));
                rc_grid.remove(widget);
                rc_grid.remove(&label);
        }});
     }
}

pub fn draw_repositories(repositories: &Vec<String>, combo_box: &ComboBoxText){
    for repository in repositories {
        let label = gtk::Label::new(Some(repository));
        label.set_visible(true);
        combo_box.append_text(label.text().as_ref());
    }
}

pub fn draw_branches(branches: &Vec<String>, combo_box: &ComboBoxText){
    for branch in branches {
        let label = gtk::Label::new(Some(branch));
        label.set_visible(true);
        combo_box.append_text(label.text().as_ref());
    }
}

pub fn draw_message(m_changes: &gtk::Box, message: &String, align: f32) {
    let label = gtk::Label::new(Some(message));
    label.set_visible(true);
    label.set_xalign(align);
    label.set_yalign(align);
    label.style_context().add_class("custom-label-message");
    m_changes.add(&label);
}

pub fn draw_error(errors: (gtk::MessageDialog, gtk::Box), message: &String, c_entry: &gtk::Entry) {
    errors.1.foreach(|child| {
        errors.1.remove(child);
    });
    draw_message(&errors.1, message, 2.0);

    errors.0.style_context().add_class("custom-error-dialog");

    errors.0.run();
    errors.0.hide();

    c_entry.set_text("");

}

pub fn draw_push_pull(rc_branch: &gtk::ComboBoxText, input: String, info: &gtk::Box, message: &String) {
    info.foreach({|child|{
        info.remove(child);
    }});
    match message.as_str() {
        "PUSH" => {
            let _ = VersionControlSystem::push(input);
            draw_info_box(info, &message);
        },
        "PULL" => {
            let _ = VersionControlSystem::pull(input);
            rc_branch.remove_all();
            let _ = branches(&rc_branch);            
            draw_info_box(info, &message);
        },
        _ => {},
    }
}

pub fn draw_info_box(info: &gtk::Box, message: &String) {
    let close = Button::builder()
                        .label("close")
                        .build();
    close.set_visible(true);
    draw_message(&info, &format!("    {} SUCCESSFULLY!     ",message).to_string(), 0.5);
    info.add(&close);
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

pub fn draw_fetch(rc_branch: &gtk::ComboBoxText, input: String, box_fetch: &gtk::Box, dialog: &gtk::Dialog) {
    box_fetch.foreach(|child| {
        box_fetch.remove(child);
    });
    let _ = VersionControlSystem::fetch(input);
    rc_branch.remove_all();
    let _ = branches(&rc_branch);
    draw_message(&box_fetch, &"     FETCH SUCCESSFULLY!      ".to_string(), 0.5);
    dialog.run();
    dialog.hide();
}