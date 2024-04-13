pub mod tab_mamagement {
    use crate::interface::storage::MFile;
    use crate::io::file_io;
    use crate::storage::rope_store;
    use crate::types::middleware_types::{CurTabName, Tab, TabMap};
    use tauri::State;

    #[tauri::command]
    pub fn create_tab(
        tab_map: State<TabMap>,
        cur_tab_name: State<CurTabName>,
        filepath: &str,
    ) -> (bool, String) {
        match file_io::read_file_str(filepath) {
            Ok(data) => {
                //let mut tab = Tab {
                //text: Box::new(rope_store::Text::from_path(filepath)),
                //parser: Box::new(Default::default()),
                //assembler: Box::new(Default::default()),
                //simulator: Box::new(Default::default()),
                //};
                //tab_map
                //.tabs
                //.lock()
                //.unwrap()
                //.insert(filepath.to_string(), tab);
                ////cur_tab_name.name.lock().unwrap().as_mut() = filepath.to_string();
                (true, data)
            }
            Err(e) => (false, e),
        }
    }

    #[tauri::command]
    pub fn close_tab(tab_map: State<TabMap>, filepath: &str) -> (bool, String) {
        match tab_map.tabs.lock().unwrap().remove(filepath) {
            Some(_) => (true, "Tab closed".to_string()),
            None => (false, "Tab not found".to_string()),
        }
    }
}

pub mod frontend_api {
    use crate::io::file_io;

    #[tauri::command]
    pub fn read_file(filepath: &str) -> (bool, String) {
        match file_io::read_file_str(filepath) {
            Ok(data) => (true, data),
            Err(e) => (false, e),
        }
    }

    #[tauri::command]
    pub fn write_file(filepath: &str, data: &str) -> (bool, String) {
        match file_io::write_file_str(filepath, data) {
            Ok(_) => (true, "File saved".to_string()),
            Err(e) => (false, e),
        }
    }
}
