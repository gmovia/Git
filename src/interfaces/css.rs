use super::interface::RustInterface;
use super::login::DrawLogin;
use gtk::prelude::*;
use gtk::{gdk, CssProvider, StyleContext};

pub fn init_css() {
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

pub fn set_styles_css_in_interface(interface: &RustInterface) {
    interface.window.style_context().add_class("custom-window");
    interface
        .commit_button
        .style_context()
        .add_class("custom-commit");
    interface.title.style_context().add_class("custom-title");
    interface
        .title_changes
        .style_context()
        .add_class("custom-title-changes");
    interface
        .title_sa
        .style_context()
        .add_class("custom-title-sa");
    interface
        .select_repository
        .style_context()
        .add_class("custom-select-repository");
    interface
        .commit_button
        .style_context()
        .add_class("button-commit");
    interface.rm.style_context().add_class("button-rm");
    interface.status.style_context().add_class("button-status");
    interface.clone_entry.style_context().add_class("entry");
    interface.merge_entry.style_context().add_class("entry");
}

pub fn set_styles_css_in_login(login: &DrawLogin) {
    login.login_window.style_context().add_class("login-window");
    login.title_login.style_context().add_class("login-title");
    login
        .login_enter
        .style_context()
        .add_class("login-ok-button");
    login
        .login_close
        .style_context()
        .add_class("login-nok-button");
    login.login_email.style_context().add_class("login-border");
    login.login_name.style_context().add_class("login-border");
}
