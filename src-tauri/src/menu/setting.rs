use tauri::{CustomMenuItem, Menu, Submenu, WindowMenuEvent};

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
    todo!("update assembler memory setting");
    // you could reference other files in menu folder about how to use the event
    // don't forage to write the comment to generate doc for frontend
}
