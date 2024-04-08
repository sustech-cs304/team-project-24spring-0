// Tab Manage
pub mod tab_mamagement {
    use crate::io::file_io;
    use crate::types::middleware_types::*;
    use tauri::State;

    #[tauri::command]
    pub fn create_tab(tab_map: State<TabMap>, filepath: &str) -> (bool, String) {
        match file_io::read_file(filepath) {
            Ok(data) => {
                todo!("Create Tab !!!");
                let mut tab;
                //let mut tab = Tab {
                //text: Box::new(Rope::from_str("")),
                //parser: Box::new(Parser::new()),
                //assembler: Box::new(Assembler::new()),
                //simulator: Box::new(Simulator::new()),
                //};
                tab_map
                    .tabs
                    .lock()
                    .unwrap()
                    .insert(filepath.to_string(), tab);
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
        match file_io::read_file(filepath) {
            Ok(data) => (true, data),
            Err(e) => (false, e),
        }
    }

    #[tauri::command]
    pub fn write_file(filepath: &str, data: &str) -> (bool, String) {
        match file_io::write_file(filepath, data) {
            Ok(_) => (true, "File saved".to_string()),
            Err(e) => (false, e),
        }
    }
}
