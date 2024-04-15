pub mod tab_mamagement {
    use tauri::{Manager, State, WindowMenuEvent};

    use crate::{
        modules::riscv::basic::interface::parser::RISCVParser,
        storage::rope_store,
        types::middleware_types::{CloseTabResponse, CurTabName, Tab, TabMap},
        utility::state_helper::set_current_tab_name,
    };

    use std::path::Path;

    pub fn create_tab(event: &WindowMenuEvent, file_path: &Path) -> Option<String> {
        //TODO: change this to export function
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

    #[tauri::command]
    pub fn close_tab(
        cur_name: State<CurTabName>,
        tab_map: State<TabMap>,
        filepath: &str,
    ) -> CloseTabResponse {
        match tab_map.tabs.lock().unwrap().remove(filepath) {
            Some(_) => CloseTabResponse {
                success: true,
                message: String::new(),
            },
            None => CloseTabResponse {
                success: false,
                message: "Tab not found".to_string(),
            },
        }
    }

    #[tauri::command]
    pub fn change_current_tab(cur_name: State<CurTabName>, newpath: &str) -> bool {
        //set_current_tab_name(&cur_name, newpath)
        todo!("Implement change_current_tab")
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
            Some(e) => (false, e),
            None => (true, "File saved".to_string()),
        }
    }
}
