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
mod test;
mod types;
mod utility;

use middleware::implementation::{frontend_api, tab_mamagement};
use tauri::{Manager, State};
use types::middleware_types;

fn main() {
    tauri::Builder::default()
        .menu(menu::init_menu())
        .on_menu_event(menu::event_handler)
        .manage(middleware_types::TabMap {
            tabs: Default::default(),
        })
        .manage(middleware_types::CurTabName {
            name: Default::default(),
        })
        .setup(|app| {
            //let tab_map = app.state::<middleware_types::TabMap>();
            //tab_map
            //.tabs
            //.lock()
            //.unwrap()
            //.insert("foo", middleware_types::Tab::new("foo"));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            frontend_api::read_file,
            frontend_api::write_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
