// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod interface;
mod io;
mod menu;
mod middleware;
mod modules;
mod parser;
mod simulator;
mod storage;
mod types;
mod utility;

use middleware::implementation::{frontend_api, tab_mamagement};
use types::middleware_types;

fn main() {
    tauri::Builder::default()
        .menu(menu::init_menu())
        .on_menu_event(menu::event_handler)
        .setup(|app| {
            //todo!("init function need here");
            Ok(())
        })
        .manage(middleware_types::TabMap {
            tabs: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![
            tab_mamagement::create_tab,
            tab_mamagement::close_tab,
            frontend_api::read_file,
            frontend_api::write_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
