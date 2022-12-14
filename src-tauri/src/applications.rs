use lazy_static::lazy_static;
use regex::Regex;
use std::env::var;
use std::fs;
use walkdir::WalkDir;

use crate::types;

lazy_static! {
    static ref XDG_DATA_DIRS: String = var("XDG_DATA_DIRS").unwrap();
}

fn get_desktop_file_paths() -> Vec<String> {
    let mut desktop_file_paths = Vec::new();
    let xdg_data_dirs = XDG_DATA_DIRS.split(":");

    for xdg_data_dir in xdg_data_dirs {
        for dir_entry_promise in WalkDir::new(xdg_data_dir).follow_links(true) {
            // ignore files that return not found while accessing them
            if let Err(_) = dir_entry_promise {
                continue;
            };
            let dir_entry = dir_entry_promise.unwrap();

            // ignore files that return not found while accessing them
            let dir_entry_path_promise = dir_entry.path().to_str();
            if dir_entry_path_promise.is_none() {
                continue;
            }
            let dir_entry_path = dir_entry_path_promise.unwrap();

            if dir_entry_path.ends_with(".desktop") {
                desktop_file_paths.push(dir_entry_path.to_string())
            };
        }
    }

    return desktop_file_paths;
}

fn to_list_item(desktop_file_path: String) -> Option<types::ListItem> {
    let desktop_file_contents = fs::read_to_string(&desktop_file_path).unwrap();

    lazy_static! {
        static ref NAME_REGEX: Regex = Regex::new(r"\nName=(.*)").unwrap();
        static ref EXEC_REGEX: Regex = Regex::new(r"\nExec=(.*)").unwrap();
        static ref NO_DISPLAY_REGEX: Regex = Regex::new(r"\n(NoDisplay=true|Hidden=true)").unwrap();
        static ref TYPE_APPLICATION_REGEX: Regex = Regex::new(r"\n(Type=Application)").unwrap();
        static ref FILE_NAME_REGEX: Regex = Regex::new(r"/([^/]+).desktop").unwrap();
    }

    let desktop_entry_name_promise = NAME_REGEX.captures(&desktop_file_contents).and_then(|cap| {
        return cap.get(1).map(|name| name.as_str().to_string());
    });
    if desktop_entry_name_promise.is_none() {
        return None;
    }
    let desktop_entry_name = desktop_entry_name_promise.unwrap();

    let desktop_entry_exec_promise = EXEC_REGEX.captures(&desktop_file_contents).and_then(|cap| {
        return cap.get(1).map(|exec| exec.as_str().to_string());
    });
    if desktop_entry_exec_promise.is_none() {
        return None;
    }

    let desktop_entry_no_display_promise = NO_DISPLAY_REGEX
        .captures(&desktop_file_contents)
        .and_then(|captures| {
            return captures.get(1).map(|capture| capture.as_str().to_string());
        });
    if desktop_entry_no_display_promise.is_some() {
        return None;
    }

    let desktop_entry_type_application_promise = TYPE_APPLICATION_REGEX
        .captures(&desktop_file_contents)
        .and_then(|captures| {
            return captures.get(1).map(|capture| capture.as_str().to_string());
        });
    if desktop_entry_type_application_promise.is_none() {
        return None;
    }

    let desktop_file_name_promise = FILE_NAME_REGEX
        .captures(&desktop_file_path)
        .and_then(|cap| {
            return cap.get(1).map(|name| name.as_str().to_string());
        });

    if desktop_file_name_promise.is_none() {
        return None;
    }

    let desktop_file_name = desktop_file_name_promise.unwrap();

    return Some(types::ListItem {
        title: desktop_entry_name,
        actions: vec![types::ListItemAction {
            keys: vec!["↵".into()],
            text: "open".into(),
            command: types::ListItemActionCommand {
                program: String::from("sh"),
                args: vec![
                    String::from("-c"),
                    format!("gtk-launch {}", desktop_file_name).into(),
                ],
            },
        }],
    });
}

pub(crate) fn get_applications_group() -> types::ItemGroup {
    let desktop_file_paths = get_desktop_file_paths();
    let mut list_items: Vec<types::ListItem> = desktop_file_paths
        .into_iter()
        .map(to_list_item)
        .filter(|list_item_option| list_item_option.is_some())
        .map(|list_item_option| list_item_option.unwrap())
        .rev()
        .collect();

    list_items.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
    list_items.dedup_by(|a, b| a.title.to_lowercase().eq(&b.title.to_lowercase()));

    return types::ItemGroup {
        name: "Apps".into(),
        icon: "Rocket".into(),
        items: list_items,
    };
}