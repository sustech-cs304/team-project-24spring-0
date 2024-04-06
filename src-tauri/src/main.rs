// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod interface;
mod io;
mod middleware;
mod modules;
mod parser;
mod simulator;
mod storage;
mod types;
mod utility;

use middleware::implementation::*;
use types::middleware_types::*;

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

fn main() {
    let menu = init_menu();
    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| match event.menu_item_id() {
            "quit" => {
                std::process::exit(0);
            }
            "close" => {
                event.window().close().unwrap();
            }
            _ => {}
        })
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
fn init_menu() -> Menu {
    Menu::with_items([
        MenuItem::SelectAll.into(),
        #[cfg(target_os = "macos")]
        MenuItem::Redo.into(),
        Submenu::new(
            "File",
            Menu::with_items([
                CustomMenuItem::new("open_file", "Open").into(),
                CustomMenuItem::new("save_file", "Save").into(),
                //CustomMenuItem::new(, title)
            ]),
        )
        .into(),
    ])
}
