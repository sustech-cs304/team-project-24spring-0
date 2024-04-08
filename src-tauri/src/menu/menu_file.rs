use tauri::{CustomMenuItem, Menu, Submenu};

pub fn new() -> Submenu {
    Submenu::new(
        "File",
        Menu::with_items([
            CustomMenuItem::new("file_open", "Open").into(),
            CustomMenuItem::new("file_save", "Save").into(),
            CustomMenuItem::new("file_save_as", "Save As...").into(),
            CustomMenuItem::new("file_share", "Share").into(),
            CustomMenuItem::new("file_close", "Close Tab").into(),
            CustomMenuItem::new("file_exit", "Exit").into(),
        ]),
    )
}

pub fn event_handler(event: tauri::WindowMenuEvent) {
    match event.menu_item_id() {
        "file_open" => {
            println!("Open file");
        }
        "file_save" => {
            println!("Save file");
        }
        "file_save_as" => {
            println!("Save As file");
        }
        "file_share" => {
            println!("Share file");
        }
        "file_close" => {
            println!("Close Tab");
        }
        "file_exit" => {
            event.window().close().unwrap();
        }
        _ => {
            println!("Unknown file menu item {}", event.menu_item_id());
        }
    }
}
