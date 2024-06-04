#![cfg_attr(
    debug_assertions,
    allow(dead_code),
    allow(unused_variables),
    allow(unused_mut),
    allow(unused_assignments),
    allow(unreachable_code),
    allow(unused_macros)
)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(linked_list_cursors)]

use std::sync::{Arc, Mutex};

use modules::riscv::middleware::frontend_api;
use once_cell::sync::Lazy;
use tauri::{AppHandle, Manager};
use types::{middleware_types, rpc_types};

use crate::remote::Modification;

/// front_end api is under [`middleware.rs`]
///
/// [`middleware.rs`]: crate::modules::riscv::middleware
mod interface;

mod io;
mod menu;
mod modules;
mod remote;
mod simulator;
mod storage;
mod types;
mod utility;

#[cfg(test)]
mod tests;

static APP_HANDLE: Lazy<Arc<Mutex<Option<AppHandle>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));
static CURSOR_LIST: Lazy<Arc<Mutex<rpc_types::CursorList>>> =
    Lazy::new(|| Arc::new(Mutex::new(Default::default())));

static HISTORY: Lazy<Arc<Mutex<Vec<Modification>>>> =
    Lazy::new(|| Arc::new(Mutex::new(Vec::new())));
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
        .manage(rpc_types::RpcState {
            rpc_server: Default::default(),
            rpc_client: Default::default(),
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
            frontend_api::modify_current_tab,
            frontend_api::read_tab,
            frontend_api::write_tab,
            frontend_api::set_return_data_range,
            frontend_api::assembly,
            frontend_api::dump,
            frontend_api::run,
            frontend_api::debug,
            frontend_api::stop,
            frontend_api::resume,
            frontend_api::step,
            frontend_api::undo,
            frontend_api::reset,
            frontend_api::set_breakpoint,
            frontend_api::remove_breakpoint,
            frontend_api::syscall_input,
            frontend_api::update_assembler_settings,
            frontend_api::start_share_server,
            frontend_api::stop_share_server,
            frontend_api::authorize_share_client
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
