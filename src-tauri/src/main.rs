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

use middleware::implementation::tab_management;
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
            tab_management::create_tab,
            tab_management::close_tab,
            tab_management::change_current_tab,
            tab_management::update_tab,
            tab_management::read_tab,
            tab_management::write_tab
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
