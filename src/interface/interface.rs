use std::{path::Path, cell::RefCell, rc::Rc};

use gtk::{prelude::*, Button, ComboBoxText};

use crate::vcs::{version_control_system::VersionControlSystem, commands::branch::BranchOptions};


pub struct RustInterface;

impl RustInterface {
    pub fn impl_interface() -> Result<(), std::io::Error>{
        if gtk::init().is_err() {
            println!("Failed to initialize GTK.");
            return Ok(());
        }
    
        let glade_src = include_str!("interface.glade");
        let builder = gtk::Builder::from_string(glade_src);
    
        let window: gtk::Window = builder.object("window").unwrap();
        let commit_button: gtk::Button = builder.object("commit").unwrap();
        let grid: gtk::Grid = builder.object("grid").unwrap();
        let select_repository: gtk::ComboBoxText = builder.object("select-repository").unwrap();
        let select_branch: gtk::ComboBoxText = builder.object("select-branch").unwrap();
        let box_window: gtk::Box = builder.object("box-add").unwrap();
        let new_branch_button: gtk::Button = builder.object("new-branch").unwrap();
        let dialog: gtk::Dialog = builder.object("dialog").unwrap();
        let dialog_entry: gtk::Entry = builder.object("dialog-entry").unwrap();

        // Lista de rutas
        let routes: Vec<String> = vec!["/ruta/ruta1".to_string(), "/ruta/ruta2".to_string()];
        let routes2: Vec<String> = vec!["/ruta/ruta6".to_string(), "/ruta/ruta7".to_string()];
    
        // Lista de repos creadas
        let repositories: Vec<String> = vec!["repo_1".to_string(),"repo_2".to_string(),"repo_3".to_string()];
    
        // Lista de branches creadas
        let vcs = VersionControlSystem::init(Path::new("test_folder"), Vec::new());
        let branches = vcs.get_branches()?;
   
        Self::draw_changes(&routes, &grid);
        Self::draw_staging_area(&routes2, &box_window);
    
        commit_button.connect_clicked(move |_| {
            grid.foreach(|child|{
                grid.remove(child);
            });
    
            box_window.foreach(|child|{
                box_window.remove(child);
            });
        });
    
        Self::draw_repositories(&repositories, &select_repository);
        Self::draw_branches(&branches, &select_branch);
    
        let version = Rc::new(RefCell::new(vcs));
        let create_branch: gtk::Button = builder.object("create").unwrap();

        // Conectar el botón a la función de manejo de eventos
        new_branch_button.connect_clicked({
            move |_| {
                dialog.run();
                dialog.hide();
            }
        });

        let rc_entry = Rc::new(RefCell::new(dialog_entry.clone()));
        let rc_branch = Rc::new(RefCell::new(select_branch.clone()));

        create_branch.connect_clicked({
            let version = version.clone();
            let rc_entry = rc_entry.clone();
            let rc_branch = rc_branch.clone();
            move |_|{
                let version = version.borrow_mut();
                let rc_entry = rc_entry.borrow_mut();
                let rc_branch = rc_branch.borrow_mut();
                let _ = version.branch(BranchOptions::NewBranch(&rc_entry.text()));
                rc_branch.append_text(&rc_entry.text());
                rc_entry.set_text("Create new branch ...");
            }});
    
        window.show_all();
    
        gtk::main();
        Ok(())
    }

    
pub fn draw_changes(changes: &Vec<String>, grid: &gtk::Grid){
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
}
