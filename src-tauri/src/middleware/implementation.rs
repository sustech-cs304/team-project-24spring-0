pub mod tab_management {
    use tauri::{Manager, State, WindowMenuEvent};

    use crate::{
        io::file_io,
        modules::riscv::basic::interface::parser::RISCVParser,
        storage::rope_store,
        types::middleware_types::{CurTabName, Optional, Tab, TabMap},
        utility::state_helper::set_current_tab_name,
    };

    use std::path::Path;

    pub fn new_tab(event: &WindowMenuEvent, file_path: &Path) -> Option<String> {
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
    pub fn create_tab(tab_map: State<TabMap>, filepath: &str) -> Optional {
        if tab_map.tabs.lock().unwrap().contains_key(filepath) {
            return Optional {
                success: false,
                message: "Tab already exists".to_string(),
            };
        }
        match rope_store::Text::from_path_str(filepath) {
            Ok(content) => {
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
                    .insert(filepath.to_string(), tab);
                Optional {
                    success: true,
                    message: String::new(),
                }
            }
            Err(e) => Optional {
                success: false,
                message: e,
            },
        }
    }

    #[tauri::command]
    pub fn close_tab(
        cur_name: State<CurTabName>,
        tab_map: State<TabMap>,
        filepath: &str,
    ) -> Optional {
        if *cur_name.name.lock().unwrap() == filepath {}

        match tab_map.tabs.lock().unwrap().remove(filepath) {
            Some(_) => Optional {
                success: true,
                message: String::new(),
            },
            None => Optional {
                success: false,
                message: "Tab not found".to_string(),
            },
        }
    }

    #[tauri::command]
    pub fn change_current_tab(cur_name: State<CurTabName>, newpath: &str) -> bool {
        *cur_name.name.lock().unwrap() = newpath.to_string();
        todo!("Implement change_current_tab")
    }

    #[tauri::command]
    pub fn update_tab(tab_map: State<TabMap>, filepath: &str, data: &str) -> Optional {
        match tab_map.tabs.lock().unwrap().get_mut(filepath) {
            Some(tab) => {
                tab.text = Box::new(rope_store::Text::from_str(data).unwrap());
                Optional {
                    success: true,
                    message: String::new(),
                }
            }
            None => Optional {
                success: false,
                message: "Tab not found".to_string(),
            },
        }
    }

    #[tauri::command]
    pub fn read_tab(filepath: &str) -> Optional {
        match file_io::read_file_str(filepath) {
            Ok(data) => Optional {
                success: true,
                message: data,
            },
            Err(e) => Optional {
                success: false,
                message: e,
            },
        }
    }

    #[tauri::command]
    pub fn write_tab(filepath: &str, data: &str) -> Optional {
        match file_io::write_file_str(filepath, data) {
            Some(e) => Optional {
                success: false,
                message: e,
            },
            None => Optional {
                success: true,
                message: String::new(),
            },
        }
    }
}

pub mod frontend_api {}
