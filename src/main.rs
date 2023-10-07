use rust_git::{repository::{self, Repository}, commands::init::Init};

fn main() {
    println!("Hello, world!");

    let repository = Init::git_init("repository_name");
}
