use tauri::api::dialog::ask;
use tauri::{CustomMenuItem, Menu, Submenu, WindowMenuEvent};

pub fn new() -> Submenu {
    Submenu::new(
        "Test",
        Menu::with_items([CustomMenuItem::new("test_foo", "Foo").into()]),
    )
}

pub fn event_handler(event: WindowMenuEvent) {
    match event.menu_item_id().strip_prefix("test_").unwrap() {
        "foo" => {}
        _ => {}
    }
}
