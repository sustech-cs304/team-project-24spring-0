use std::path::Path;

use tauri::api::dialog::{FileDialogBuilder, MessageDialogKind};
use tauri::{CustomMenuItem, Manager, Menu, Submenu, WindowMenuEvent};

use super::display_alert_dialog;
use crate::io::file_io;
use crate::modules::riscv::basic::interface::parser::{RISCVExtension, RISCVParser};
use crate::storage::rope_store;
use crate::types::menu_types;
use crate::types::middleware_types::{Tab, TabMap};
use crate::utility::state_helper::event::{get_current_tab_name, set_current_tab_name};

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
    let picker = FileDialogBuilder::new();
    picker.pick_file(move |file_path| match file_path {
        Some(file_path) => match new_tab(&event, file_path.as_path()) {
            Some(err) => display_alert_dialog(
                MessageDialogKind::Info,
                format!("Failed to open {:?}", file_path.file_name().unwrap()).as_str(),
                err.as_str(),
                |_| {},
            ),
            _ => {
                let name = get_current_tab_name(&event);
                let tab_map = event.window().state::<TabMap>();
                let lock = tab_map.tabs.lock().unwrap();
                let tab = lock.get(&name).unwrap();
                let content = tab.text.to_string();
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
        },
        _ => {}
    });
}

fn save_handler<'a>(event: WindowMenuEvent) {
    let name = get_current_tab_name(&event);
    let tab_map = event.window().state::<TabMap>();
    let mut lock = tab_map.tabs.lock().unwrap();
    let tab = lock.get_mut(&name).unwrap();
    match tab.text.save() {
        Some(err) => {
            display_alert_dialog(
                MessageDialogKind::Info,
                "Failed to save file",
                err.as_str(),
                |_| {},
            );
        }
        None => {
            let _ = event
                .window()
                .emit("front_file_save", get_current_tab_name(&event));
        }
    }
}

fn save_as_handler(event: WindowMenuEvent) {
    let content = {
        let name = get_current_tab_name(&event);
        let tab_map = event.window().state::<TabMap>();
        let lock = tab_map.tabs.lock().unwrap();
        let tab = lock.get(&name).unwrap();
        tab.text.to_string()
    };
    let picker = tauri::api::dialog::FileDialogBuilder::new();
    picker.save_file(move |file_path| match file_path {
        Some(file_path) => match file_io::write_file(file_path.as_path(), &content) {
            Some(err) => {
                display_alert_dialog(
                    MessageDialogKind::Info,
                    "Failed to save file",
                    err.as_str(),
                    |_| {},
                );
            }
            None => {
                event.window().emit("front_file_save_as", true).unwrap();
            }
        },
        _ => {}
    });
}

fn share_handler(event: WindowMenuEvent) {
    todo!("Share file with socket");
}

fn close_handler(event: WindowMenuEvent) {
    //TODO: check if the file is dirty
}

fn exit_handler(event: WindowMenuEvent) {
    event.window().close().unwrap();
    todo!("check all dirty file before exit");
}

fn new_tab(event: &WindowMenuEvent, file_path: &Path) -> Option<String> {
    match rope_store::Text::from_path(file_path) {
        Ok(content) => {
            let tab_map = event.window().state::<TabMap>();
            let tab = Tab {
                text: Box::new(content),
                parser: Box::new(RISCVParser::new(&vec![RISCVExtension::RV32I])),
                //assembler: Box::new(Default::default()),
                //simulator: Box::new(Default::default()),
            };
            tab_map
                .tabs
                .lock()
                .unwrap()
                .insert(file_path.to_str().unwrap().to_string(), tab);
            set_current_tab_name(&event, file_path.to_str().unwrap());
            None
        }
        Err(e) => Some(e),
    }
}

fn dirty_close_checker(event: &WindowMenuEvent, tab: &mut Tab) -> bool {
    if tab.text.is_dirty() {}
    true
}
