pub mod state {
    use super::super::ptr::Ptr;
    use crate::types::middleware_types::{CurTabName, Tab, TabMap};
    use tauri::State;

    pub fn get_current_tab_name(cur_tab_name: State<CurTabName>) -> String {
        cur_tab_name.name.lock().unwrap().clone()
    }

    pub fn set_current_tab_name(cur_tab_name: State<CurTabName>, new_name: &str) {
        let mut name = cur_tab_name.name.lock().unwrap();
        *name = new_name.to_string();
    }
}

pub mod event {
    use crate::types::middleware_types::{CurTabName, Tab, TabMap};
    use std::{collections::HashMap, sync::MutexGuard};
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

    pub fn set_current_tab_name(event: &WindowMenuEvent, new_name: &str) {
        let tn = event.window().state::<CurTabName>();
        let mut name = tn.name.lock().unwrap();
        *name = new_name.to_string();
    }
}
