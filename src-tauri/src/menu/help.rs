use tauri::{CustomMenuItem, Menu, Submenu, WindowMenuEvent};

use crate::APP_HANDLE;

pub fn new() -> Submenu {
    Submenu::new(
        "Help",
        Menu::with_items([CustomMenuItem::new("help_manual", "Manual").into()]),
    )
}

pub fn event_handler(event: WindowMenuEvent) {
    match event.menu_item_id().strip_prefix("help_").unwrap() {
        "manual" => manual_handler(&event),
        _ => {}
    }
}

fn manual_handler(_event: &WindowMenuEvent) {
    if let Some(app_handle) = APP_HANDLE.lock().unwrap().as_ref() {
        let _window = tauri::WindowBuilder::new(
            app_handle,
            "manual",
            tauri::WindowUrl::App("/doc/riscv/index.html".parse().unwrap()),
        )
        .title("Manual")
        .menu(Menu::new())
        .build()
        .unwrap();
    }
}
