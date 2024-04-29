// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    debug_assertions,
    allow(unused_variables, unused_macros, unused_imports, dead_code)
)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod interface;
mod io;
mod menu;
mod modules;
mod remote;
mod storage;
mod tests;
mod types;
mod utility;

use std::sync::{Arc, Mutex};

use modules::riscv::middleware::*;
use once_cell::sync::Lazy;
use tauri::{AppHandle, Manager};
use types::middleware_types;

static APP_HANDLE: Lazy<Arc<Mutex<Option<AppHandle>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));
mod assembler;

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
            let app_handle = app.app_handle();
            let mut global_handle = APP_HANDLE.lock().unwrap();
            *global_handle = Some(app_handle);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            frontend_api::create_tab,
            frontend_api::close_tab,
            frontend_api::change_current_tab,
            frontend_api::update_tab,
            frontend_api::read_tab,
            frontend_api::write_tab,
            frontend_api::assembly,
            frontend_api::debug,
            frontend_api::set_breakpoint,
            frontend_api::remove_breakpoint,
            //frontend_api::syscall_input,
            frontend_api::update_assembler_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
