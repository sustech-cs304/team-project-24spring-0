use crate::types::middleware_types::CurTabName;
use tauri::{Manager, WindowMenuEvent};

pub fn get_current_tab_name(event: &WindowMenuEvent) -> String {
    event
        .window()
        .state::<CurTabName>()
        .name
        .lock()
        .unwrap()
        .clone()
}
