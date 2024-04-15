pub mod tab_mamagement {
    use tauri::{Manager, State, WindowMenuEvent};

    use crate::{
        modules::riscv::basic::interface::parser::RISCVParser,
        storage::rope_store,
        types::middleware_types::{Tab, TabMap},
        utility::state_helper::set_current_tab_name,
    };

    use std::path::Path;

    pub fn create_tab(event: &WindowMenuEvent, file_path: &Path) -> Option<String> {
        match rope_store::Text::from_path(file_path) {
            Ok(content) => {
                let tab_map = event.window().state::<TabMap>();
                let tab = Tab {
                    text: Box::new(content),
                    parser: Box::new(RISCVParser::new()),
                    //assembler: Box::new(Default::default()),
                    //simulator: Box::new(Default::default()),
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
