use crate::APP_HANDLE;
use tauri::{CustomMenuItem, Manager, Menu, Submenu, WindowMenuEvent};

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
            "manual", /* the unique window label */
            tauri::WindowUrl::External(
                "http://localhost:3000/riscv-spec-v2.2.pdf"
                    .parse()
                    .unwrap(),
            ),
        )
        .title("Manual")
        .menu(Menu::new())
        .build()
        .unwrap();
    }
}

// fn ai_handler(_event: &WindowMenuEvent) {
//     if let Some(app_handle) = APP_HANDLE.lock().unwrap().as_ref() {
//         let _window = tauri::WindowBuilder::new(
//             app_handle,
//             "ai", /* the unique window label */
//             tauri::WindowUrl::External("http://localhost:3000/ai".parse().unwrap()),
//         )
//         .title("AI Chat")
//         .menu(Menu::new())
//         .build()
//         .unwrap();
//     }
// }
