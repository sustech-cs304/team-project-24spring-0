use super::display_dialog;
use crate::io::file_io;
use crate::modules::riscv::basic::interface::assembler::RiscVAssembler;
use crate::modules::riscv::basic::interface::parser::{RISCVExtension, RISCVParser};
use crate::storage::rope_store;
use crate::types::menu_types;
use crate::types::middleware_types::{Tab, TabMap};
use crate::utility::ptr::Ptr;
use crate::utility::state_helper::event::{get_current_tab_name, set_current_tab_name};
use std::path::Path;
use tauri::api::dialog::{FileDialogBuilder, MessageDialogButtons, MessageDialogKind};
use tauri::{CustomMenuItem, Manager, Menu, Submenu, WindowMenuEvent};

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
            save_handler(&event);
        }
        "save_as" => {
            save_as_handler(event);
        }
        "share" => {
            share_handler(&event);
        }
        "close" => {
            close_handler(&event);
        }
        "exit" => {
            exit_handler(&event);
        }
        _ => {
            println!("Unknown file menu item {}", event.menu_item_id());
        }
    }
}

/// event emit: front_file_open
/// payload: OpenFile { file_path: String, content: String }
fn open_handler(event: WindowMenuEvent) {
    let picker = FileDialogBuilder::new();
    picker.pick_file(move |file_path| match file_path {
        Some(file_path) => match new_tab(&event, file_path.as_path()) {
            Some(err) => display_dialog(
                MessageDialogKind::Info,
                MessageDialogButtons::Ok,
                format!("Failed to open {:?}", file_path.file_name().unwrap()).as_str(),
                err.as_str(),
                |_| {},
            ),
            None => {
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

/// event emit: front_file_save
/// payload: String
fn save_handler(event: &WindowMenuEvent) {
    let name = get_current_tab_name(&event);
    let tab_map = event.window().state::<TabMap>();
    let mut lock = tab_map.tabs.lock().unwrap();
    let tab = lock.get_mut(&name).unwrap();
    match tab.text.save() {
        Some(err) => {
            display_dialog(
                MessageDialogKind::Info,
                MessageDialogButtons::Ok,
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

/// event emit: front_file_save_as
/// payload: String
/// FIXME: the emit event maybe unused?
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
                display_dialog(
                    MessageDialogKind::Info,
                    MessageDialogButtons::Ok,
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

fn share_handler(event: &WindowMenuEvent) {
    let _window = tauri::WindowBuilder::new(
        &event.window().app_handle(),
        "live_share", /* the unique window label */
        tauri::WindowUrl::External("https://tauri.app/".parse().unwrap()),
    )
    .title("Live Share")
    .menu(Menu::new())
    .build()
    .unwrap();
}

fn close_handler(event: &WindowMenuEvent) {
    let window = event.window();
    let tab_map = window.state::<TabMap>();
    let name = get_current_tab_name(event);
    let mut lock = tab_map.tabs.lock().unwrap();
    let mut tab = lock.get_mut(&name).unwrap();
    dirty_close_checker(event, &name, &mut tab);
}

/// Iterate all tabs, check if each tab is dirty, if so, display a dialog to ask
/// user to save the file or not. If user choose to save, save the file and then
/// close the tab. If user choose not to save, close the tab directly.
///
/// This function will emit a `front_close_tab` event to the window, and
/// may emit multiple times if there are multiple dirty tabs need to be closed.
fn exit_handler(event: &WindowMenuEvent) {
    let window = event.window();
    let tab_map = window.state::<TabMap>();
    let mut lock = tab_map.tabs.lock().unwrap();
    for (name, tab) in lock.iter_mut() {
        dirty_close_checker(event, name, tab);
    }
}

/// Create a new tab with the file in provided file path
///
/// Return None if success, Some(err) if failed.
/// If success, will set the current tab name to the opened file path.
fn new_tab(event: &WindowMenuEvent, file_path: &Path) -> Option<String> {
    match rope_store::Text::from_path(file_path) {
        Ok(content) => {
            let tab_map = event.window().state::<TabMap>();
            let tab = Tab {
                text: Box::new(content),
                parser: Box::new(RISCVParser::new(&vec![RISCVExtension::RV32I])),
                assembler: Box::new(RiscVAssembler::new()),
                //simulator: Box::new(Default::default()),
                parser_result: Default::default(),
                assemble_result: Default::default(),
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

/// Check if the file is dirty, if so, display a dialog to ask user to save the
/// file or not. If user choose to save, save the file and then close the tab.
/// If user choose not to save, close the tab directly.
///
/// This function will emit a `front_close_tab` event to the window.
fn dirty_close_checker(event: &WindowMenuEvent, name: &str, tab: &mut Tab) {
    let tab_ptr = Ptr::new(tab);
    let event_ptr = Ptr::new(event);
    if tab.text.is_dirty() {
        display_dialog(
            MessageDialogKind::Warning,
            MessageDialogButtons::YesNo,
            "Warning",
            format!(
                "File {} is modified but not save, are you sure to exit",
                name
            )
            .as_str(),
            move |save| {
                if save {
                    let tab = tab_ptr.as_mut();
                    tab.text.save();
                }
                let event = event_ptr.as_ref();
                let _ = event.window().emit("front_close_tab", true);
            },
        )
    }
}
