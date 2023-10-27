use super::interface::RustInterface;
use gtk::{prelude::*};

pub fn handle_commit(interface: &RustInterface){
    let grid = interface.grid.clone();
    let box_window = interface.box_window.clone();

    interface.commit_button.connect_clicked(move |_| {
        grid.foreach(|child|{
            grid.remove(child);
        });
        
        box_window.foreach(|child|{
            box_window.remove(child);
        });
    });
}

pub fn handle_branch(interface: &RustInterface){
    let dialog = interface.dialog.clone();
    interface.new_branch_button.connect_clicked({
        move |_| {
            dialog.run();
            dialog.hide();
        }
    });
}