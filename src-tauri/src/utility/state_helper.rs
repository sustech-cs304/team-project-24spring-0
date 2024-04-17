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

    pub fn get_current_tab(cur_tab_name: State<CurTabName>, tab_map: State<TabMap>) -> Ptr<Tab> {
        let name = get_current_tab_name(cur_tab_name);
        Ptr::new(tab_map.tabs.lock().unwrap().get(&name).unwrap())
    }
}

pub mod event {
    use super::super::ptr::Ptr;
    use crate::types::middleware_types::{CurTabName, Tab, TabMap};
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
}
