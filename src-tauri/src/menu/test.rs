use tauri::api::dialog::ask;
use tauri::{CustomMenuItem, Manager, Menu, Submenu, WindowMenuEvent};

use crate::modules::riscv::middleware::frontend_api::{start_rpc_server, stop_rpc_server};
use crate::types::middleware_types::{CurTabName, TabMap};

pub fn new() -> Submenu {
    Submenu::new(
        "Test",
        Menu::with_items([
            CustomMenuItem::new("test_foo", "Start").into(),
            CustomMenuItem::new("test_bar", "Stop").into(),
        ]),
    )
}

pub fn event_handler(event: WindowMenuEvent) {
    match event.menu_item_id().strip_prefix("test_").unwrap() {
        "foo" => {
            let window = event.window();
            start_rpc_server(
                window.state::<CurTabName>(),
                window.state::<TabMap>(),
                11451,
                "foo",
            );
        }
        "bar" => {
            let window = event.window();
            stop_rpc_server(window.state::<TabMap>());
        }
        _ => {}
    }
}
