use std::io;

use gtk::{prelude::*, Button};

//use crate::vcs::version_control_system::VersionControlSystem;

pub struct RustInterface;

impl RustInterface {

    pub fn impl_interface() -> Result<(),std::io::Error>{
        if gtk::init().is_err() {
            //println!("Failed to initialize GTK.");
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Failed to initialize GTK."));
        }
        
        //let vcs = VersionControlSystem::init(Path::new("test_folder"), Vec::new());

        let glade_src = include_str!("interface.glade");
        let builder = gtk::Builder::from_string(glade_src);
    
        let window: gtk::Window = builder.object("window").unwrap();
        let commit_button: gtk::Button = builder.object("commit").unwrap();
        let grid: gtk::Grid = builder.object("grid").unwrap();
        let select_repository: gtk::ComboBoxText = builder.object("select-repository").unwrap();
        let select_branch: gtk::ComboBoxText = builder.object("select-branch").unwrap();
        let box_window: gtk::Box = builder.object("box-add").unwrap();

        // let new_branch: gtk::Button = builder.object("new-branch").unwrap();
        // let new_branch_window: gtk::Window = builder.object("new-branch-window").unwrap();
        // let new_branch_entry: gtk::Entry = builder.object("new-branch-entry").unwrap();
        // let enter_button: gtk::Button = builder.object("enter").unwrap();

        // Lista de rutas
        let routes: Vec<&str> = vec!["/ruta/ruta1", "/ruta/ruta2"];
        let routes2: Vec<&str> = vec!["/ruta/ruta6", "/ruta/ruta7"];
    
        // Lista de repos creadas
        let repositories: Vec<&str> = vec!["repo_1","repo_2","repo_3"];
    
        // Lista de branches creadas
        let branches: Vec<&str> = vec!["master","branch_1","branch_2","branch_3"];

        //let branches = vcs.get_branch()?;
    
        for (i, route) in routes.iter().enumerate() {
            let label = gtk::Label::new(Some(route));
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
    
            grid.attach(&label, 0, i as i32, 1, 1);
            grid.attach(&add_button, 1, i as i32, 1, 1);
            grid.attach(&remove_button, 2, i as i32, 1, 1);
    
            add_button.connect_clicked(move |_| { //tiene que hacer algo
                println!("algo");
            });  
         }
    
         for route in routes2 {
            let label = gtk::Label::new(Some(route));
            label.set_visible(true);
            label.set_xalign(2.0); // Alinea el texto a la izquierda
            label.set_yalign(0.5);
            box_window.add(&label);
         }
    
        commit_button.connect_clicked(move |_| {
            grid.foreach(|child|{
                grid.remove(child);
            });
    
            box_window.foreach(|child|{
                box_window.remove(child);
            });
        });
        
        for repo in repositories {
            let label = gtk::Label::new(Some(repo));
            label.set_visible(true);
            select_repository.append_text(&label.text().to_string());
        }

        // new_branch.connect_clicked(move |_| {
        //     let new_branch_text = new_branch_entry.text();
        //     enter_button.connect_clicked(move |_| {
        //         vcs.branch(BranchOptions::NewBranch(&new_branch_text));
        //         new_branch_window.hide()
        //     });

        //     new_branch_window.show_all();
        // });
        // let create_branch = vcs.branch(BranchOptions::NewBranch("new_branch"))?;
        for branch in branches {
            let label = gtk::Label::new(Some(&branch));
            label.set_visible(true);
    
            select_branch.append_text(&label.text().to_string());
        }
        window.show_all();
    
        gtk::main();
        Ok(())
    }
}
