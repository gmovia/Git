
use std::{cell::RefCell, rc::Rc, path::Path, collections::HashMap, fs::OpenOptions, io::{self, BufRead}};

use gtk::{prelude::*, Button, ComboBoxText};
use crate::vcs::version_control_system::VersionControlSystem;

pub fn branches(combo_box: &ComboBoxText) -> Result<(), std::io::Error>{
    let branches = VersionControlSystem::get_branches()?;
    draw_branches(&branches, combo_box);
    Ok(())
}

pub fn repositories(combo_box: &ComboBoxText) -> Result<(), std::io::Error>{
    
    let repositories = VersionControlSystem::read_bdd_of_repositories()?;
    draw_repositories(&repositories, combo_box);
    Ok(())
}

pub fn changes_and_staging_area(grid: &gtk::Grid, box_window: &gtk::Box) -> Result<(), std::io::Error>{
    grid.foreach(|child|{
        grid.remove(child);
    });
    
    box_window.foreach(|child|{
        box_window.remove(child);
    });
    println!("HOLA");
    let (untracked_files, changes_not_be_commited, changes_to_be_commited) = VersionControlSystem::status()?;
    println!("Hola");
    let staging_area: Vec<String> = changes_to_be_commited.keys().cloned().collect();

    let mut changes = untracked_files.clone();
    changes.extend(changes_not_be_commited);

    draw_staging_area(&staging_area, box_window);
    draw_changes(&changes, grid, box_window);
    Ok(())
}

pub fn draw_changes(changes: &HashMap<String, String>, grid: &gtk::Grid, box_window: &gtk::Box){

    //let version: Rc<RefCell<VersionControlSystem>> = Rc::new(RefCell::new(vcs.clone()));

    let mut index = 0;
    for (path, state) in changes {
        let path_label = gtk::Label::new(Some(path));
        path_label.set_visible(true);
        path_label.set_xalign(2.0); // Alinea el texto a la izquierda
        path_label.set_yalign(0.5); // Alinea el texto arriba

        let state_label = gtk::Label::new(Some(state));
        state_label.set_visible(true);
        state_label.set_xalign(2.0); // Alinea el texto a la izquierda
        state_label.set_yalign(0.5); // Alinea el texto arriba

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

        grid.attach(&path_label, 0, index as i32, 1, 1);
        grid.attach(&state_label, 1, index as i32, 1, 1);
        grid.attach(&add_button, 2, index as i32, 1, 1);
        index += 1;

        let path_clone = path.clone(); // Clona el path
        add_button.connect_clicked({
            //let version = version.clone();
            let rc_grid = grid.clone();
            let rc_add = box_window.clone();
            move |widget|{ 
                //let version = version.borrow_mut();
                let _ = VersionControlSystem::add(Path::new(&path_clone)); // Usa la copia clonada
                rc_grid.remove(widget);
                rc_grid.remove(&path_label);
                rc_grid.remove(&state_label);
                rc_add.add(&path_label);
        }});
    }
}

pub fn draw_staging_area(staging_area: &Vec<String>, _box: &gtk::Box){

    for path in staging_area {
        let label = gtk::Label::new(Some(path));
        label.set_visible(true);
        label.set_xalign(2.0);
        label.set_yalign(0.5);
        _box.add(&label);
     }
}

pub fn draw_repositories(repositories: &Vec<String>, combo_box: &ComboBoxText){
    for repository in repositories {
        let label = gtk::Label::new(Some(repository));
        label.set_visible(true);
        combo_box.append_text(&label.text().to_string());
    }
}

pub fn draw_branches(branches: &Vec<String>, combo_box: &ComboBoxText){
    for branch in branches {
        let label = gtk::Label::new(Some(branch));
        label.set_visible(true);
        combo_box.append_text(&label.text().to_string());
    }
}