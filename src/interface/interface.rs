use std::{path::Path, cell::RefCell, rc::Rc};

use gtk::prelude::*;

use crate::vcs::{version_control_system::VersionControlSystem, commands::branch::BranchOptions};
use crate::interface::draw::{draw_repositories, draw_branches, draw_changes, draw_staging_area};

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
   
        draw_changes(&routes, &grid);
        draw_staging_area(&routes2, &box_window);
    
        commit_button.connect_clicked(move |_| {
            grid.foreach(|child|{
                grid.remove(child);
            });
    
            box_window.foreach(|child|{
                box_window.remove(child);
            });
        });
    
        draw_repositories(&repositories, &select_repository);
        draw_branches(&branches, &select_branch);
    
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
}
