use tauri::{CustomMenuItem, Menu, Submenu, WindowMenuEvent};

pub fn new() -> Submenu {
    Submenu::new(
        "Setting",
        Menu::with_items([CustomMenuItem::new("setting_assembler_memory", "Set Memory").into()]),
    )
}

pub fn event_handler(event: WindowMenuEvent) {
    match event.menu_item_id().strip_prefix("setting_").unwrap() {
        _ => {
            println!("Unknown file menu item {}", event.menu_item_id());
        }
    }
}
