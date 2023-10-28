use core::borrow;
use std::{cell::RefCell, rc::Rc, path::Path};

use gtk::{prelude::*, Button, ComboBoxText};
use crate::{vcs::version_control_system::VersionControlSystem, handlers::add};

use super::interface;

pub fn branches(vcs: &VersionControlSystem, combo_box: &ComboBoxText) -> Result<(), std::io::Error>{
    let branches = vcs.get_branches()?;
    draw_branches(&branches, combo_box);
    Ok(())
}

pub fn repositories(_vcs: &VersionControlSystem, combo_box: &ComboBoxText) -> Result<(), std::io::Error>{
    let repositories: Vec<String> = vec!["repo_1".to_string(),"repo_2".to_string(),"repo_3".to_string()];
    draw_repositories(&repositories, combo_box);
    Ok(())
}

pub fn changes_and_staging_area(vcs: &VersionControlSystem, grid: &gtk::Grid, box_window: &gtk::Box) -> Result<(), std::io::Error>{
    let (untracked_files, changes_not_be_commited, changes_to_be_commited) = vcs.status()?;
    let untracked_files_paths: Vec<String> = untracked_files.keys().cloned().collect();
    let changes_not_be_commited_paths: Vec<String> = changes_not_be_commited.keys().cloned().collect();
    
    let changes: Vec<String> = untracked_files_paths.iter().chain(changes_not_be_commited_paths.iter()).cloned().collect();
    let staging_area: Vec<String> = changes_to_be_commited.keys().cloned().collect();

    draw_changes(&changes, grid, vcs, box_window);
    draw_staging_area(&staging_area, box_window);
    Ok(())
}

pub fn draw_changes(changes: &Vec<String>, grid: &gtk::Grid, vcs: &VersionControlSystem, box_window: &gtk::Box){
    let version: Rc<RefCell<VersionControlSystem>> = Rc::new(RefCell::new(vcs.clone()));

    for (index, path) in changes.iter().enumerate() {
        let label = gtk::Label::new(Some(path));
        label.set_visible(true);
        label.set_xalign(2.0); // Alinea el texto a la izquierda
        label.set_yalign(0.5); // Alinea el texto arriba

        let add_button = Button::builder()
        .margin_start(10)
        .label("+")
        .build();
        add_button.set_visible(true);

        let remove_button = Button::builder()
        .margin_start(10)
        .label("-")
        .build();
        add_button.set_visible(true);

        grid.attach(&label, 0, index as i32, 1, 1);
        grid.attach(&add_button, 1, index as i32, 1, 1);
        grid.attach(&remove_button, 2, index as i32, 1, 1);

        let path_clone = path.clone(); // Clona el path
        let box_window = box_window.clone();
        add_button.connect_clicked({
            let version = version.clone();
            move |_|{
                let mut version = version.borrow_mut();
                let _ = version.add(Path::new(&path_clone)); // Usa la copia clonada
                //box_window.add(&label);
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