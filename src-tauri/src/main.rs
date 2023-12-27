// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::utils::config::Size;
use tauri_plugin_updater::UpdaterExt;
use serde_json;
use serde::{Serialize, Deserialize};
use std::{error::Error, iter::TrustedRandomAccessNoCoerce};

#[derive(Serialize, Deserialize)]
struct Member {
    member_id: u16,
    member_name: String,
    member_callsign: String,
    member_qso: u32,
}

#[derive(Serialize, Deserialize)]
struct Club {
    club_id: u16,
    club_name: String,
    club_callsign: String,
    club_locator: String,
    club_qso: u32,
    club_members: Vec<Member>
}

#[derive(Serialize, Deserialize)]
struct Config {
    callsign: String,
    name: String,
    locator: String,
    version: String,
    total_qso: u32,
    clubs: Vec<Club>,
}

fn config_file_exist() -> bool {
    let path = get_OS();
    std::path::Path::new(&path).exists()
}

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
        "windows" => format!("C:\\Users\\{}\\Document\\{}\\config.json", username, appname).to_string(),
        "macos" => "/Library/Applications/Club-Logger/config.json".to_string(),
        "linux" => "/etc/Club-Logger/config.json".to_string(),
        _ => "/etc/Club-Logger/config.json".to_string(),
    }
}

fn read_config_file() -> String {
    let path = get_OS();
    let config_file_exist = std::path::Path::new(&path).exists();
    match config_file_exist {
        true => read_config_file(),
        false => create_config_file(),
    }
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

async fn create_config_file() -> Option<()> {
    let path = get_OS();
    let config = Config {
        callsign: "N0CALL".to_string(),
        name: "John Doe".to_string(),
        locator: "AA00aa".to_string(),
        version: "0.0.0".to_string(),
        total_qso: 0,
        clubs: Vec::new(),
    };
    let config = serde_json::to_string(&config);
    match config {
        Ok(config) => (),
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

async fn read_configuration() -> String {
    let path = get_OS();
    println!("Path: {}", path);
    let file: String = std::fs::read_to_string(path).expect("Unable to read file");
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
fn configuration() -> String {
    let config = read_configuration();
    //wait for the async function to finish
    let config = tauri::async_runtime::block_on(config);
    config
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
        // check to see if this is the first time the app has been run, if so, create a new window and load /welcome.html
        //.on_page_load(|window, _| {
        //    if config_file_exist() {
        //        window.builder()
        //            .title("Club Logger")
        //            .size(Size::new(800.0, 600.0))
        //            .resizable(true)
        //            .build(
        //    } else {
        //        window.set_title("Club Logger");
        //        window.load_file("welcome.html");
        //    }
        //})
        .invoke_handler(tauri::generate_handler![greet, configuration])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
