use crate::types::middleware_types::TabMap;
use crate::utility::state_helper::get_current_tab_name;
use crate::{io::file_io, types::menu_types};

use tauri::api::dialog::MessageDialogKind;
use tauri::{CustomMenuItem, Manager, Menu, Submenu, WindowMenuEvent};

use super::display_alert_dialog;

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

pub fn event_handler(event: WindowMenuEvent) {
    match event.menu_item_id().strip_prefix("file_").unwrap() {
        "open" => {
            open_handler(event);
        }
        "save" => {
            save_handler(event);
        }
        "save_as" => {
            save_as_handler(event);
        }
        "share" => {
            share_handler(event);
        }
        "close" => {
            close_handler(event);
        }
        "exit" => {
            exit_handler(event);
        }
        _ => {
            println!("Unknown file menu item {}", event.menu_item_id());
        }
    }
}

fn open_handler(event: WindowMenuEvent) {
    let picker = tauri::api::dialog::FileDialogBuilder::new();
    picker.pick_file(move |file_path| match file_path {
        Some(file_path) => match file_io::read_file_str(file_path.to_str().unwrap()) {
            Ok(content) => {
                event
                    .window()
                    .emit(
                        "front_file_open",
                        menu_types::OpenFile {
                            file_path: file_path.to_str().unwrap().to_owned(),
                            content,
                        },
                    )
                    .unwrap();
            }
            Err(err) => display_alert_dialog(
                MessageDialogKind::Info,
                format!("Failed to open {:?}", file_path.file_name().unwrap()).as_str(),
                err.as_str(),
                |_| {},
            ),
        },
        _ => {}
    });
}

fn save_handler<'a>(event: WindowMenuEvent) {
    let name = get_current_tab_name(&event);
    let tm = event.window().state::<TabMap>();
    let mut inner_map = tm.tabs.lock().unwrap();
    match inner_map.get_mut(&name) {
        Some(tab) => match tab.text.save_file() {
            Ok(_) => {}
            Err(err) => {
                display_alert_dialog(
                    MessageDialogKind::Info,
                    "Failed to save file",
                    err.as_str(),
                    |_| {},
                );
            }
        },
        None => {}
    }
}

fn save_as_handler(event: WindowMenuEvent) {
    let name = get_current_tab_name(&event);
    let picker = tauri::api::dialog::FileDialogBuilder::new();
    let mut save_success = false;
    picker.save_file(move |file_path| match file_path {
        Some(file_path) => {
            //match file_io::write_file_str(file_path, data)
            //save_success = true;
            event
                .window()
                .emit("front_file_save_as", save_success)
                .unwrap();
        }
        _ => {}
    });
}

fn share_handler(event: WindowMenuEvent) {
    todo!("Share file with socket");
}

fn close_handler(event: WindowMenuEvent) {
    //TODO
}

fn exit_handler(event: WindowMenuEvent) {
    event.window().close().unwrap();
    todo!("check all dirty file before exit");
}
