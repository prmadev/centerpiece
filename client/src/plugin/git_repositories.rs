use crate::plugin::utils::Plugin;
use anyhow::Context;

pub struct GitRepositoriesPlugin {
    entries: Vec<crate::model::Entry>,
}

impl Plugin for GitRepositoriesPlugin {
    fn id() -> &'static str {
        return "git-repositories";
    }

    fn priority() -> u32 {
        return 28;
    }

    fn title() -> &'static str {
        return "󰘬 Git Repositories";
    }

    fn entries(&self) -> Vec<crate::model::Entry> {
        return self.entries.clone();
    }

    fn new() -> Self {
        let git_repository_paths: Vec<String> =
            crate::plugin::utils::read_index_file("git-repositories-index.json");

        let home = std::env::var("HOME").unwrap_or(String::from(""));

        let entries = git_repository_paths
            .into_iter()
            .filter_map(|git_repository_path| {
                let git_repository_display_name = git_repository_path.replacen(&home, "~", 1);

                return Some(crate::model::Entry {
                    id: git_repository_path,
                    title: git_repository_display_name,
                    action: String::from("focus"),
                    meta: String::from("windows"),
                });
            })
            .collect();

        return Self { entries };
    }

    fn activate(
        &mut self,
        entry_id: String,
        plugin_channel_out: &mut iced::futures::channel::mpsc::Sender<crate::Message>,
    ) -> anyhow::Result<()> {
        std::process::Command::new("alacritty")
            .arg("--working-directory")
            .arg(&entry_id)
            .spawn()
            .context(format!(
                "Failed to launch terminal while activating entry with id '{}'.",
                entry_id
            ))?;

        std::process::Command::new("sublime_text")
            .arg("--new-window")
            .arg(&entry_id)
            .spawn()
            .context(format!(
                "Failed to launch editor while activating entry with id '{}'.",
                entry_id
            ))?;

        std::process::Command::new("sublime_merge")
            .arg("--new-window")
            .arg(&entry_id)
            .spawn()
            .context(format!(
                "Failed to launch git ui while activating entry with id '{}'.",
                entry_id
            ))?;

        plugin_channel_out
            .try_send(crate::Message::Exit)
            .context(format!(
                "Failed to send message to exit application while activating entry with id '{}'.",
                entry_id
            ))?;

        return Ok(());
    }
}
