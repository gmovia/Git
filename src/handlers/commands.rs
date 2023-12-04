use super::clone::handler_clone;
use super::fetch::handler_fetch;
use super::ls_files::handler_ls_files;
use super::ls_tree::handler_ls_tree;
use super::merge::handler_merge;
use super::pull::handler_pull;
use super::rebase::handler_rebase;
use super::remote::handler_remote;
use super::show_ref::handler_show_ref;
use super::tag::handler_tag;
use crate::handlers::add::handler_add;
use crate::handlers::branch::handler_branch;
use crate::handlers::cat_file::handler_cat_file;
use crate::handlers::checkout::handler_checkout;
use crate::handlers::commit::handler_commit;
use crate::handlers::hash_object::handler_hash_object;
use crate::handlers::log::handler_log;
use crate::handlers::push::handler_push;
use crate::handlers::rm::handler_rm;
use crate::handlers::status::handler_status;

pub fn handler_command(input: &str) -> String {
    let input = input.trim();
    let _: Vec<String> = input
        .to_string()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    match input {
        "git status" => handler_status(),
        x if x.contains("git merge") => handler_merge(x.to_string()),
        x if x.contains("git hash-object") => handler_hash_object(x.to_string()),
        x if x.contains("git add") => handler_add(x.to_string()),
        x if x.contains("git cat-file") => handler_cat_file(x.to_string()),
        x if x.contains("git rm") => handler_rm(x.to_string()),
        x if x.contains("git log") => handler_log(),
        x if x.contains("git commit") => handler_commit(x.to_string()),
        x if x.contains("git branch") => handler_branch(x.to_string()),
        x if x.contains("git checkout") => handler_checkout(x.to_string()),
        x if x.contains("git clone") => handler_clone(x.to_string()),
        x if x.contains("git ls-files") => handler_ls_files(x.to_string()),
        x if x.contains("git push") => handler_push(x.to_string()),
        x if x.contains("git fetch") => handler_fetch(x.to_string()),
        x if x.contains("git ls-tree") => handler_ls_tree(x.to_string()),
        x if x.contains("git pull") => handler_pull(x.to_string()),
        x if x.contains("git tag") => handler_tag(x.to_string()),
        x if x.contains("git remote") => handler_remote(x.to_string()),
        x if x.contains("git show-ref") => handler_show_ref(x.to_string()),
        x if x.contains("git rebase") => handler_rebase(x.to_string()),
        _ => "Failed or Panicked.".to_string(),
    }
}
