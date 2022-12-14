#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod applications;
mod git_projects;
mod types;
mod windows;

#[tauri::command]
fn get_applications_group() -> types::ItemGroup {
    return applications::get_applications_group();
}

#[tauri::command]
fn get_windows_group() -> types::ItemGroup {
    return windows::get_windows_group();
}

#[tauri::command]
fn get_git_projects_group() -> types::ItemGroup {
    return git_projects::get_git_projects_group();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_applications_group,
            get_windows_group,
            get_git_projects_group
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}