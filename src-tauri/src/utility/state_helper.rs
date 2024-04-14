use crate::types::middleware_types::{CurTabName, Tab, TabMap};
use tauri::{Manager, State, WindowMenuEvent};

pub fn get_current_tab_name(event: &WindowMenuEvent) -> String {
    event
        .window()
        .state::<CurTabName>()
        .name
        .lock()
        .unwrap()
        .clone()
}

pub fn get_current_tab(event: &WindowMenuEvent) -> State<TabMap> {
    let tab_name = get_current_tab_name(event);
    event.window().state::<TabMap>()
}

pub fn get_current_tab_mut(event: &WindowMenuEvent) -> &mut Tab {
    let tab_name = get_current_tab_name(event);
    let mut tm = event.window().state::<TabMap>();
    tm.tabs.lock().unwrap().get_mut(&tab_name).unwrap()
}
