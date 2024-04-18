// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod assembler;
mod interface;
mod io;
mod menu;
mod modules;
mod storage;
mod tests;
mod types;
mod utility;

use modules::riscv;
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
            riscv::middleware::tab_management::create_tab,
            riscv::middleware::tab_management::close_tab,
            riscv::middleware::tab_management::change_current_tab,
            riscv::middleware::tab_management::update_tab,
            riscv::middleware::tab_management::read_tab,
            riscv::middleware::tab_management::write_tab,
            riscv::middleware::frontend_api::assemble,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
