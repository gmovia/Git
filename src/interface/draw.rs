use gtk::{prelude::*, Button, ComboBoxText};

/*
pub fn draw(vcs: &VersionControlSystem, grid: &gtk::Grid, _box: &gtk::Box, combo_box: &gtk::ComboBoxText) -> Result<(), std::io::Error>{
    let repositories = vec!["repo1".to_string(), "repo2".to_string()];
    let branches = vcs.get_branches()?;
    
    let (untracked_files, changes_not_be_commited, changes_to_be_commited) = vcs.status()?;
    let changes_a: Vec<String> = untracked_files.keys().cloned().collect();
    let changes_b: Vec<String> = changes_not_be_commited.keys().cloned().collect();
    
    let changes: Vec<String> = changes_a.iter().chain(changes_b.iter()).cloned().collect();
    let staging_area: Vec<String> = changes_to_be_commited.keys().cloned().collect();

    draw_changes(&changes, grid);
    draw_staging_area(&staging_area, _box);
    draw_repositories(&repositories, combo_box);
    draw_branches(&branches, combo_box);
    Ok(())
}
*/

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