use tauri::{CustomMenuItem, Menu, Submenu, WindowMenuEvent};

use crate::APP_HANDLE;

pub fn new() -> Submenu {
    Submenu::new(
        "Setting",
        Menu::with_items([CustomMenuItem::new("setting_assembler_memory", "Set Memory").into()]),
    )
}

pub fn event_handler(event: WindowMenuEvent) {
    match event.menu_item_id().strip_prefix("setting_").unwrap() {
        "assembler_memory" => {
            assembler_memory_handler(event);
        }
        _ => {
            println!("Unknown file menu item {}", event.menu_item_id());
        }
    }
}

fn assembler_memory_handler(event: WindowMenuEvent) {
    let handle_lock = APP_HANDLE.lock().unwrap();
    let app_handle = handle_lock.as_ref().unwrap();
    let _window = tauri::WindowBuilder::new(
        app_handle,
        "assembler_memory",
        tauri::WindowUrl::App("/assembler-settings".parse().unwrap()),
    )
    .title("Set Memory")
    .menu(Menu::new())
    .build()
    .unwrap();
}
