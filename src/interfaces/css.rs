use gtk::{CssProvider, gdk, StyleContext};
use gtk::prelude::*;
use super::interface::RustInterface;

pub fn init_css(){
    let provider = CssProvider::new();

    // Load CSS from a string (style.css)
    let _ = provider.load_from_data(include_str!("style.css").as_bytes());
    
    // Add the provider to the default screen
    if let Some(display) = gdk::Display::default() {
        StyleContext::add_provider_for_screen(
            &display.default_screen(),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

pub fn set_styles_css(interface: &RustInterface){
        interface.window.style_context().add_class("custom-window");
        interface._box.style_context().add_class("custom-box");
        interface.commit_button.style_context().add_class("custom-commit");
        interface.title.style_context().add_class("custom-title");
        interface.title_changes.style_context().add_class("custom-title-changes");
        interface.title_sa.style_context().add_class("custom-title-sa");
        interface.select_repository.style_context().add_class("custom-select-repository");
        interface.commit_button.style_context().add_class("button-commit");
        interface.rm.style_context().add_class("button-rm");
        interface.status.style_context().add_class("button-status");
}