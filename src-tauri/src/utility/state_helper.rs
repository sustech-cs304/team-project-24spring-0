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
