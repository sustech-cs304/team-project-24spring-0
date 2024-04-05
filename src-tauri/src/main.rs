// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod interface;
mod io;
mod middleware;
mod simulator;
mod storage;
mod types;
mod utility;

use middleware::implementation::*;
use types::*;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            //todo!("init function need here");
            Ok(())
        })
        .manage(TabMap::default())
        .invoke_handler(tauri::generate_handler![
            tab_mamagement::create_tab,
            frontend_api::read_file,
            frontend_api::write_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
