use std::fmt::format;
use std::path::Path;

use tauri::api::dialog::{FileDialogBuilder, MessageDialogKind};
use tauri::{CustomMenuItem, Manager, Menu, Submenu, WindowMenuEvent};

use super::{display_alert_dialog, display_confirm_dialog};
use crate::io::file_io;
use crate::modules::riscv::basic::interface::parser::{RISCVExtension, RISCVParser};
use crate::storage::rope_store;
use crate::types::menu_types;
use crate::types::middleware_types::{Tab, TabMap};
use crate::utility::state_helper::event::{get_current_tab_name, set_current_tab_name};

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
