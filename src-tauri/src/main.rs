// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri_plugin_updater::UpdaterExt;
use serde_json;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn get_OS() -> String {
    let os = std::env::consts::OS;
    let username = std::env::var("USERNAME").unwrap();
    let appname = "Club-Logger".to_string();
    match os {
        "windows" => "C:\\Users\\${username}\\AppData\\Roaming\\${appname}\\config.json".to_string(),
        "macos" => "/Library/Applications/Club-Logger/config.json".to_string(),
        "linux" => "/etc/Club-Logger/config.json".to_string(),
        _ => "/etc/Club-Logger/config.json".to_string(),
    }
}

fn read_config_file() -> String {
    let path = get_OS();
    let file = std::fs::read_to_string(path).expect("Unable to read file");
    let config = serde_json::from_str(&file);
    match config {
        Ok(config) => config,
        Err(e) => {
            println!("Error: {}", e);
            e.to_string()
        }
    }
}

#[tauri::command]
fn create_config_file() -> String {
    let os_path = get_OS();
    let json_content = std::fs::read_to_string(os_path).expect("Unable to read file");
    let config = serde_json::from_str(&json_content);
    match config {
        Ok(config) => config,
        Err(e) => {
            println!("Error: {}", e);
            e.to_string()
        }
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let response = handle.updater().expect("REASON").check().await; // If .await?; works in the setup hook you can remove the if let Ok line - can't try that myself right now.
                if let Ok(response) = response {
                    if let Some(response) = response {
                        // The first || {} ignores the download progress
                        // The second || {} ignores the download finished event
                        // If you wanna handle them you can write actual functions instead
                        let _ = response.download_and_install(|_, _| {}, || {}).await; // this returns a result you may wanna handle
                        println!("Update downloaded and installed");
                        println!("{:?}", response);
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
