use crate::types::middleware_types::{CurTabName, Tab, TabMap};
use tauri::{Manager, WindowMenuEvent};

use super::ptr::Ptr;

pub fn get_current_tab_name(event: &WindowMenuEvent) -> String {
    event
        .window()
        .state::<CurTabName>()
        .name
        .lock()
        .unwrap()
        .clone()
}

pub fn set_current_tab_name(event: &WindowMenuEvent, new_name: &str) {
    let tn = event.window().state::<CurTabName>();
    let mut name = tn.name.lock().unwrap();
    *name = new_name.to_string();
}

pub fn get_current_tab(event: &WindowMenuEvent) -> Ptr<Tab> {
    let name = get_current_tab_name(&event);
    Ptr::new(
        event
            .window()
            .state::<TabMap>()
            .tabs
            .lock()
            .unwrap()
            .get(&name)
            .unwrap(),
    )
}
