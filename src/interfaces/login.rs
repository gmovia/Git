use crate::vcs::files::config::Config;

use super::interface::RustInterface;

use gtk::prelude::*;


#[derive(Debug, Clone, Default)]
pub struct DrawLogin {
    pub login_window: gtk::Window,
    pub login_email: gtk::Entry,
    pub login_name: gtk::Entry,
    pub login_enter: gtk::Button,
    pub login_cancel: gtk::Button,
}

impl DrawLogin {


    pub fn new() -> DrawLogin {
        if gtk::init().is_err() {
            println!("Failed to initialize GTK.");
        }

        let glade_src = include_str!("draw_login.glade");
        let builder = gtk::Builder::from_string(glade_src);

        DrawLogin { 
            login_window: builder.object("login-window").unwrap(),
            login_email: builder.object("login-email").unwrap(),
            login_name: builder.object("login-name").unwrap(),
            login_enter: builder.object("login-enter").unwrap(),
            login_cancel: builder.object("login-cancel").unwrap(),
        }
    
    }

    pub fn impl_login(&self) {

        let interface = RustInterface::new();
        let l_email = self.login_email.clone();
        let l_name = self.login_name.clone();
        self.login_enter.connect_clicked({
            move |_| {
                let config = (l_name.text().to_string(), l_email.text().to_string());
                let _ = Config::write_config(config);
                let _ = interface.impl_interface();
            
            }
        });

        self.login_cancel.connect_clicked({
            let log_window = self.login_window.clone();
            move |_|{
                log_window.close();
            }
        });
        
        self.login_window.show_all();
        gtk::main();
        
    }
}
