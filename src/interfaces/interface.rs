use gtk::prelude::*;

use super::css::{init_css, set_styles_css_in_interface};
use super::handler::{
    handle_branch, handle_clone, handle_command, handle_commit, handle_fetch, handle_log,
    handle_logs_errors, handle_merge, handle_other_commands, handle_pull, handle_push,
    handle_repository, handle_rm, handle_status,
};
use crate::interfaces::draw::{branches, changes_and_staging_area, repositories};

#[derive(Debug, Default)]
pub struct RustInterface {
    pub window: gtk::Window,
    pub title: gtk::Label,
    pub _box: gtk::Box,
    pub commit_button: gtk::Button,
    pub grid: gtk::Grid,
    pub select_repository: gtk::ComboBoxText,
    pub repository_button: gtk::Button,
    pub repository_dialog: gtk::Dialog,
    pub repository_entry: gtk::Entry,
    pub repository_box: gtk::Box,
    pub delete_repository: gtk::Button,
    pub create_repository: gtk::Button,
    pub select_branch: gtk::ComboBoxText,
    pub grid_staging: gtk::Grid,
    pub branch_button: gtk::Button,
    pub branch_dialog: gtk::Dialog,
    pub branch_box: gtk::Box,
    pub dialog_entry: gtk::Entry,
    pub create_branch: gtk::Button,
    pub delete_branch: gtk::Button,
    pub status: gtk::Button,
    pub commit_dialog: gtk::Dialog,
    pub commit_box: gtk::Box,
    pub message: gtk::Entry,
    pub message_ok: gtk::Button,
    pub log: gtk::Button,
    pub log_dialog: gtk::Dialog,
    pub log_box: gtk::Box,
    pub close_log: gtk::Button,
    pub title_changes: gtk::Label,
    pub title_sa: gtk::Label,
    pub terminal_dialog: gtk::Dialog,
    pub terminal: gtk::Button,
    pub command_entry: gtk::Entry,
    pub command_box: gtk::Box,
    pub enter: gtk::Button,
    pub rm: gtk::Button,
    pub rm_dialog: gtk::Dialog,
    pub rm_entry: gtk::Entry,
    pub rm_enter: gtk::Button,
    pub rm_box: gtk::Box,
    pub merge: gtk::Button,
    pub merge_entry: gtk::Entry,
    pub merge_dialog: gtk::Dialog,
    pub merge_changes: gtk::Box,
    pub resolve: gtk::Button,
    pub merge_grid: gtk::Grid,
    pub apply_merge: gtk::Button,
    pub clone: gtk::Button,
    pub clone_entry: gtk::Entry,
    pub info_clone: gtk::Box,
    pub both_dialog: gtk::Dialog,
    pub both_ok: gtk::Button,
    pub both_box: gtk::Box,
    pub both_text: gtk::TextView,
    pub fetch: gtk::Button,
    pub pull_push_fetch_dialog: gtk::Dialog,
    pub pull_push_fetch_box: gtk::Box,
    pub pull_push_fetch_close: gtk::Button,
    pub push: gtk::Button,
    pub pull: gtk::Button,
    pub files: gtk::Button,
    pub ls_files_dialog: gtk::Dialog,
    pub all: gtk::Button,
    pub o: gtk::Button,
    pub m: gtk::Button,
    pub c: gtk::Button,
    pub d: gtk::Button,
    pub selection_box: gtk::Box,
    pub close_files: gtk::Button,
    pub ls_tree: gtk::Button,
    pub tree_branch_entry: gtk::Entry,
    pub ls_tree_dialog: gtk::Dialog,
    pub tree_box: gtk::Box,
    pub close_tree: gtk::Button,
    pub apply_tree: gtk::Button,
    pub other_commands: gtk::Button,
    pub others_dialog: gtk::Dialog,
    pub others_close: gtk::Button,
    pub tag: gtk::Button,
    pub create_tag: gtk::Button,
    pub create_light_tag: gtk::Button,
    pub delete_tag: gtk::Button,
    pub show_tags: gtk::Button,
    pub show_tags_box: gtk::Box,
    pub tag_dialog: gtk::Dialog,
    pub create_delete_light_dialog: gtk::Dialog,
    pub create_dialog: gtk::Dialog,
    pub tag_light_entry: gtk::Entry,
    pub tag_entry: gtk::Entry,
    pub tag_message_entry: gtk::Entry,
    pub tag_box: gtk::Box,
    pub tag_light_box: gtk::Box,
    pub create_tag_button: gtk::Button,
    pub create_light_button: gtk::Button,
    pub delete_tag_button: gtk::Button,
    pub tag_close: gtk::Button,
    pub error_dialog: gtk::MessageDialog,
    pub error_box: gtk::Box,
    pub error_close: gtk::Button,
    pub check_ignore: gtk::Button,
    pub check_ignore_entry: gtk::Entry,
    pub ignore_dialog: gtk::Dialog,
    pub check_ignore_box: gtk::Box,
    pub close_ignore: gtk::Button,
    pub check_button: gtk::Button,
    pub show_ref: gtk::Button,
    pub show_ref_dialog: gtk::Dialog,
    pub get_all_refs: gtk::Button,
    pub get_refs_heads: gtk::Button,
    pub get_refs_tags: gtk::Button,
    pub show_ref_close: gtk::Button,
    pub show_ref_box: gtk::Box,
    pub remote: gtk::Button,
    pub remote_options_dialog: gtk::Dialog,
    pub remote_add: gtk::Button,
    pub remote_remove: gtk::Button,
    pub remote_get: gtk::Button,
    pub remote_close: gtk::Button,
    pub remote_add_dialog: gtk::Dialog,
    pub repo_name_add_remote: gtk::Entry,
    pub path_remote: gtk::Entry,
    pub enter_add_remote: gtk::Button,
    pub box_add_remote: gtk::Box,
    pub remote_remove_dialog: gtk::Dialog,
    pub delete_repo_remote: gtk::Button,
    pub box_remove_remote: gtk::Box,
    pub repo_name_remove_remote: gtk::Entry,
    pub remote_get_dialog: gtk::Dialog,
    pub get_repo_remote: gtk::Button,
    pub box_get_remote: gtk::Box,
    pub repo_name_get_remote: gtk::Entry,
    pub push_dialog: gtk::Dialog,
    pub push_entry: gtk::Entry,
    pub push_enter: gtk::Button,
    pub pull_dialog: gtk::Dialog,
    pub pull_entry: gtk::Entry,
    pub pull_enter: gtk::Button,
    pub fetch_dialog: gtk::Dialog,
    pub fetch_entry: gtk::Entry,
    pub fetch_enter: gtk::Button,
    pub rebase: gtk::Button,
    pub rebase_dialog: gtk::Dialog,
    pub rebase_entry: gtk::Entry,
    pub rebase_box: gtk::Box,
    pub rebase_enter: gtk::Button,
    pub rebase_cancel: gtk::Button,
    pub logs_errors: gtk::Button,
    pub logs_errors_dialog: gtk::Dialog,
    pub logs_errors_box: gtk::Box,
    pub logs_errors_close: gtk::Button,
}

impl RustInterface {
    pub fn new() -> RustInterface {
        if gtk::init().is_err() {
            println!("Failed to initialize GTK.");
        }

        let glade_src = include_str!("interface.glade");
        let builder = gtk::Builder::from_string(glade_src);

        init_css();

        RustInterface {
            window: builder.object("window").unwrap(),
            title: builder.object("title").unwrap(),
            _box: builder.object("box").unwrap(),
            commit_button: builder.object("commit").unwrap(),
            grid: builder.object("grid").unwrap(),
            select_repository: builder.object("select-repository").unwrap(),
            repository_button: builder.object("repository").unwrap(),
            repository_dialog: builder.object("repository-dialog").unwrap(),
            repository_entry: builder.object("dialog-entry-repo").unwrap(),
            repository_box: builder.object("repo-box").unwrap(),
            delete_repository: builder.object("delete-repo").unwrap(),
            create_repository: builder.object("create-repo").unwrap(),
            select_branch: builder.object("select-branch").unwrap(),
            grid_staging: builder.object("grid-staging").unwrap(),
            branch_button: builder.object("branch").unwrap(),
            branch_dialog: builder.object("branch-dialog").unwrap(),
            branch_box: builder.object("branch-box").unwrap(),
            dialog_entry: builder.object("dialog-entry").unwrap(),
            create_branch: builder.object("create").unwrap(),
            delete_branch: builder.object("delete").unwrap(),
            status: builder.object("status").unwrap(),
            commit_dialog: builder.object("commit-dialog").unwrap(),
            commit_box: builder.object("commit-box").unwrap(),
            message: builder.object("message-entry").unwrap(),
            message_ok: builder.object("message-ok").unwrap(),
            log: builder.object("log").unwrap(),
            log_dialog: builder.object("log-dialog").unwrap(),
            log_box: builder.object("log-box").unwrap(),
            close_log: builder.object("close-log").unwrap(),
            title_changes: builder.object("title-changes").unwrap(),
            title_sa: builder.object("title-sa").unwrap(),
            terminal_dialog: builder.object("terminal-dialog").unwrap(),
            terminal: builder.object("terminal").unwrap(),
            command_box: builder.object("command-box").unwrap(),
            command_entry: builder.object("command-entry").unwrap(),
            enter: builder.object("enter").unwrap(),
            rm: builder.object("rm").unwrap(),
            rm_dialog: builder.object("rm-dialog").unwrap(),
            rm_entry: builder.object("rm-entry").unwrap(),
            rm_enter: builder.object("rm-enter").unwrap(),
            rm_box: builder.object("rm-box").unwrap(),
            merge: builder.object("merge").unwrap(),
            merge_dialog: builder.object("merge-dialog").unwrap(),
            merge_entry: builder.object("merge-entry").unwrap(),
            merge_changes: builder.object("merge-changes").unwrap(),
            resolve: builder.object("resolve").unwrap(),
            merge_grid: builder.object("merge-grid").unwrap(),
            apply_merge: builder.object("apply-merge").unwrap(),
            clone: builder.object("clone").unwrap(),
            clone_entry: builder.object("clone-entry").unwrap(),
            info_clone: builder.object("info-clone").unwrap(),
            both_dialog: builder.object("both-dialog").unwrap(),
            both_ok: builder.object("both-ok").unwrap(),
            both_box: builder.object("both-box").unwrap(),
            both_text: builder.object("both-text").unwrap(),
            fetch: builder.object("fetch").unwrap(),
            pull_push_fetch_dialog: builder.object("pull-push-fetch-dialog").unwrap(),
            pull_push_fetch_box: builder.object("pull-push-fetch-box").unwrap(),
            pull_push_fetch_close: builder.object("pull-push-fetch-close").unwrap(),
            push: builder.object("push").unwrap(),
            pull: builder.object("pull").unwrap(),
            files: builder.object("ls-files").unwrap(),
            ls_files_dialog: builder.object("ls-files-dialog").unwrap(),
            all: builder.object("all-files").unwrap(),
            o: builder.object("-o").unwrap(),
            m: builder.object("-m").unwrap(),
            c: builder.object("-c").unwrap(),
            d: builder.object("-d").unwrap(),
            selection_box: builder.object("ls-files-box").unwrap(),
            close_files: builder.object("close-files").unwrap(),
            ls_tree: builder.object("ls-tree").unwrap(),
            tree_branch_entry: builder.object("tree-branch-entry").unwrap(),
            ls_tree_dialog: builder.object("tree-dialog").unwrap(),
            tree_box: builder.object("ls-tree-box").unwrap(),
            close_tree: builder.object("close-tree").unwrap(),
            apply_tree: builder.object("apply-tree").unwrap(),
            other_commands: builder.object("other-commands").unwrap(),
            others_close: builder.object("others-close").unwrap(),
            others_dialog: builder.object("others-dialog").unwrap(),
            tag: builder.object("tag").unwrap(),
            create_tag: builder.object("create-tag").unwrap(),
            create_light_tag: builder.object("create-light-tag").unwrap(),
            delete_tag: builder.object("delete-tag").unwrap(),
            show_tags: builder.object("get-tags").unwrap(),
            show_tags_box: builder.object("get-tags-box").unwrap(),
            tag_dialog: builder.object("tag-dialog").unwrap(),
            create_delete_light_dialog: builder.object("delete-create-light-dialog").unwrap(),
            create_dialog: builder.object("create-dialog").unwrap(),
            tag_entry: builder.object("tag-entry").unwrap(),
            tag_light_entry: builder.object("tag-light-entry").unwrap(),
            tag_message_entry: builder.object("tag-message-entry").unwrap(),
            tag_box: builder.object("tag-box").unwrap(),
            tag_light_box: builder.object("tag-light-box").unwrap(),
            create_tag_button: builder.object("create-tag1").unwrap(),
            create_light_button: builder.object("create-tag-button").unwrap(),
            delete_tag_button: builder.object("delete-tag-button").unwrap(),
            tag_close: builder.object("tag-close").unwrap(),
            error_dialog: builder.object("error-dialog").unwrap(),
            error_box: builder.object("error-box").unwrap(),
            error_close: builder.object("close-error-dialog").unwrap(),
            check_ignore: builder.object("check-ignore").unwrap(),
            check_ignore_entry: builder.object("check-ignore-entry").unwrap(),
            ignore_dialog: builder.object("check-ignore-dialog").unwrap(),
            check_ignore_box: builder.object("check-ignore-box").unwrap(),
            close_ignore: builder.object("close-ignore").unwrap(),
            check_button: builder.object("check-ignore-button").unwrap(),
            show_ref: builder.object("show-ref").unwrap(),
            show_ref_dialog: builder.object("show-ref-dialog").unwrap(),
            get_all_refs: builder.object("get-all-refs").unwrap(),
            get_refs_heads: builder.object("get-refs-heads").unwrap(),
            get_refs_tags: builder.object("get-refs-tags").unwrap(),
            show_ref_close: builder.object("show-ref-close").unwrap(),
            show_ref_box: builder.object("show-ref-box").unwrap(),
            remote: builder.object("remote").unwrap(),
            remote_options_dialog: builder.object("remote-options-dialog").unwrap(),
            remote_add: builder.object("remote-add").unwrap(),
            remote_remove: builder.object("remote-remove").unwrap(),
            remote_get: builder.object("remote-get").unwrap(),
            remote_add_dialog: builder.object("remote-add-dialog").unwrap(),
            repo_name_add_remote: builder.object("repo-name-add-remote").unwrap(),
            path_remote: builder.object("path-remote").unwrap(),
            enter_add_remote: builder.object("enter-add-remote").unwrap(),
            box_add_remote: builder.object("box-add-remote").unwrap(),
            remote_remove_dialog: builder.object("remote-remove-dialog").unwrap(),
            delete_repo_remote: builder.object("delete-repo-remote").unwrap(),
            box_remove_remote: builder.object("box-remove-remote").unwrap(),
            repo_name_remove_remote: builder.object("repo-name-remove-remote").unwrap(),
            remote_get_dialog: builder.object("remote-get-dialog").unwrap(),
            get_repo_remote: builder.object("get-repo-remote").unwrap(),
            box_get_remote: builder.object("box-get-remote").unwrap(),
            repo_name_get_remote: builder.object("repo-name-get-remote").unwrap(),
            push_dialog: builder.object("push-dialog").unwrap(),
            push_entry: builder.object("push-entry").unwrap(),
            push_enter: builder.object("push-enter").unwrap(),
            pull_dialog: builder.object("pull-dialog").unwrap(),
            pull_entry: builder.object("pull-entry").unwrap(),
            pull_enter: builder.object("pull-enter").unwrap(),
            fetch_dialog: builder.object("fetch-dialog").unwrap(),
            fetch_entry: builder.object("fetch-entry").unwrap(),
            fetch_enter: builder.object("fetch-enter").unwrap(),
            rebase: builder.object("rebase").unwrap(),
            rebase_dialog: builder.object("rebase-dialog").unwrap(),
            rebase_box: builder.object("rebase-box").unwrap(),
            rebase_entry: builder.object("rebase-entry").unwrap(),
            rebase_enter: builder.object("rebase-enter").unwrap(),
            rebase_cancel: builder.object("rebase-cancel").unwrap(),
            remote_close: builder.object("remote-close").unwrap(),
            logs_errors: builder.object("logs-errors").unwrap(),
            logs_errors_dialog: builder.object("logs-errors-dialog").unwrap(),
            logs_errors_box: builder.object("logs-errors-box").unwrap(),
            logs_errors_close: builder.object("logs-errors-close").unwrap(),
        }
    }

    pub fn impl_interface(&self) -> Result<(), std::io::Error> {
        set_styles_css_in_interface(self);

        let _ = changes_and_staging_area(&self.grid, &self.grid_staging);
        repositories(&self.select_repository)?;
        branches(&self.select_branch)?;
        handle_branch(self);
        handle_commit(self);
        handle_status(self);
        handle_log(self);
        handle_command(self);
        handle_rm(self);
        handle_merge(self);
        handle_repository(self);
        handle_other_commands(self);
        handle_clone(self);
        handle_fetch(self);
        handle_pull(self);
        handle_push(self);
        handle_logs_errors(self);

        self.window.show_all();
        gtk::main();

        Ok(())
    }
}
