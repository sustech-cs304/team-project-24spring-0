// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    debug_assertions,
    allow(dead_code),
    allow(unused_imports),
    allow(unused_variables),
    allow(unused_mut),
    allow(unused_assignments),
    allow(unreachable_code),
    allow(unused_macros)
)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(linked_list_cursors)]
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

use modules::riscv::middleware::frontend_api;
use once_cell::sync::Lazy;
use tauri::{AppHandle, Manager};
use types::middleware_types;

static APP_HANDLE: Lazy<Arc<Mutex<Option<AppHandle>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));

fn main() {
    tauri::Builder::default()
        .menu(menu::init_menu())
        .on_menu_event(menu::event_handler)
        .manage(middleware_types::TabMap {
            tabs: Default::default(),
            rpc_server: Default::default(),
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
            frontend_api::insert_in_current_tab,
            frontend_api::delete_in_current_tab,
            frontend_api::read_tab,
            frontend_api::write_tab,
            frontend_api::set_cursor,
            frontend_api::assembly,
            frontend_api::run,
            frontend_api::debug,
            frontend_api::step,
            frontend_api::dump,
            frontend_api::undo,
            frontend_api::reset,
            frontend_api::set_breakpoint,
            frontend_api::remove_breakpoint,
            frontend_api::syscall_input,
            frontend_api::update_assembler_settings,
            frontend_api::start_rpc_server,
            frontend_api::stop_rpc_server,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
