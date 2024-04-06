// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod assembler;
mod interface;
mod middleware;
mod modules;
mod simulator;
mod storage;

use std::collections::HashMap;
use std::sync::Mutex;

use middleware::implementation::Tab;

type Storage = Mutex<HashMap<String, Box<Tab>>>;

#[tauri::command]
fn great(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    tauri::Builder::default()
        //.manage(Storage {})
        .invoke_handler(tauri::generate_handler![great])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
