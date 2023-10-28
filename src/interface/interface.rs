
use std::path::Path;

use gtk::prelude::*;

use crate::vcs::version_control_system::VersionControlSystem;
use crate::interface::draw::{repositories, branches, changes_and_staging_area};

use super::handler::{handle_branch, handle_commit, handle_status};

pub struct RustInterface {
    pub window: gtk::Window,
    pub commit_button: gtk::Button,
    pub grid: gtk::Grid,
    pub select_repository: gtk::ComboBoxText,
    pub select_branch: gtk::ComboBoxText,
    pub box_window: gtk::Box,
    pub new_branch_button: gtk::Button,
    pub dialog: gtk::Dialog,
    pub dialog_entry: gtk::Entry,
    pub create_branch: gtk::Button,
    pub status: gtk::Button,
}

impl RustInterface {

    pub fn new() -> RustInterface {
        if gtk::init().is_err() {
            println!("Failed to initialize GTK.");
        }

        let glade_src = include_str!("interface.glade");
        let builder = gtk::Builder::from_string(glade_src);
        
        RustInterface {
            window: builder.object("window").unwrap(),
            commit_button: builder.object("commit").unwrap(),
            grid: builder.object("grid").unwrap(),
            select_repository: builder.object("select-repository").unwrap(),
            select_branch: builder.object("select-branch").unwrap(),
            box_window: builder.object("box-add").unwrap(),
            new_branch_button: builder.object("new-branch").unwrap(),
            dialog: builder.object("dialog").unwrap(),
            dialog_entry: builder.object("dialog-entry").unwrap(),
            create_branch: builder.object("create").unwrap(),
            status: builder.object("status").unwrap(),
        }
    }
    
    pub fn impl_interface(&self) -> Result<(), std::io::Error>{    
        let vcs = VersionControlSystem::init(Path::new("test_folder"), Vec::new());

        let _ = changes_and_staging_area(&vcs, &self.grid, &self.box_window);
        repositories(&vcs, &self.select_repository)?;
        branches(&vcs, &self.select_branch)?;
        handle_branch(&self, &vcs);
        handle_commit(&self);
        handle_status(&self, &vcs);
        
        

        self.window.show_all();   
        gtk::main();  // esto corta el ciclo de ejecucion

        Ok(())
    }
}



        //let has: HashMap<String, String> = HashMap::new();
        //let g = Arc::new(Mutex::new(has));
        //let shared_g = g.clone();

        //let handle2 = thread::spawn(move || {
        //    let mut locked_vector = shared_g.lock().unwrap();
        //});