pub mod tab_management {
    use tauri::State;

    use crate::{
        io::file_io,
        modules::riscv::basic::interface::parser::{RISCVExtension, RISCVParser},
        storage::rope_store,
        types::middleware_types::{CurTabName, Optional, Tab, TabMap},
    };

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
                    parser: Box::new(RISCVParser::new(&vec![RISCVExtension::RV32I])),
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

pub mod frontend_api {
    use crate::{
        types::middleware_types::{AssembleResult, CurTabName, TabMap},
        utility::state_helper::state::get_current_tab,
    };
    use tauri::State;

    #[tauri::command]
    pub fn assemble(cur_tab_name: State<CurTabName>, tab_map: State<TabMap>) -> AssembleResult {
        let tab_ptr = get_current_tab(cur_tab_name, tab_map);
        let tab = tab_ptr.as_mut();
        match tab.parser.parse(tab.text.to_string()) {
            Ok(ir) => AssembleResult {
                success: true,
                error: Default::default(),
            },
            Err(e) => AssembleResult {
                success: false,
                error: e,
            },
        }
    }
}
