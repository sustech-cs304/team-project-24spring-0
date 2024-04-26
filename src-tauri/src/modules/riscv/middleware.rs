pub mod tab_management {
    use crate::{
        io::file_io,
        modules::riscv::basic::interface::parser::{RISCVExtension, RISCVParser},
        storage::rope_store,
        types::middleware_types::{CurTabName, Optional, Tab, TabMap},
    };
    use tauri::State;

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
    use crate::types::middleware_types::{
        AssembleResult, AssemblerConfig, CurTabName, SyscallDataType, TabMap,
    };
    use std::any::Any;
    use tauri::State;

    #[tauri::command]
    pub fn assemble(cur_tab_name: State<CurTabName>, tab_map: State<TabMap>) -> AssembleResult {
        let name = cur_tab_name.name.lock().unwrap().clone();
        let mut lock = tab_map.tabs.lock().unwrap();
        let tab = lock.get_mut(&name).unwrap();
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

    #[tauri::command]
    pub fn dump(tab_map: State<TabMap>) -> bool {
        todo!("Implement dump")
    }

    #[tauri::command]
    pub fn debug(tab_map: State<TabMap>) -> bool {
        todo!("Implement debug")
    }

    #[tauri::command]
    #[allow(non_snake_case)]
    pub fn setBreakPoint(tab_map: State<TabMap>) -> bool {
        todo!("Implement setBreakPoint")
    }

    #[tauri::command]
    #[allow(non_snake_case)]
    pub fn removeBreakPoint(tab_map: State<TabMap>) {
        todo!("Implement removeBreakPoint")
    }

    #[tauri::command]
    #[allow(non_snake_case)]
    pub fn syscallInput(
        cur_name: State<CurTabName>,
        tab_map: State<TabMap>,
        inType: &str,
        val: &dyn Any,
    ) -> bool {
        let name = cur_name.name.lock().unwrap().clone();
        let mut lock = tab_map.tabs.lock().unwrap();
        let tab = lock.get_mut(&name).unwrap();
        let val = match inType {
            "Int" => val.downcast_ref::<i32>().map(|&v| SyscallDataType::Int(v)),
            "Float" => val
                .downcast_ref::<f32>()
                .map(|&v| SyscallDataType::Float(v)),
            "Double" => val
                .downcast_ref::<f64>()
                .map(|&v| SyscallDataType::Double(v)),
            "String" => val
                .downcast_ref::<String>()
                .map(|v| SyscallDataType::String(v.as_bytes().to_vec())),
            "Char" => val.downcast_ref::<u8>().map(|&v| SyscallDataType::Char(v)),
            "Long" => val.downcast_ref::<i64>().map(|&v| SyscallDataType::Long(v)),
            _ => None,
        };
        match val {
            Some(v) => {
                //TODO
                //tab.parser.syscall_input_request(v);
                true
            }
            None => false,
        }
    }

    #[tauri::command]
    #[allow(non_snake_case)]
    pub fn updateAssemblerSettings(
        cur_tab_name: State<CurTabName>,
        tab_map: State<TabMap>,
        settings: &AssemblerConfig,
    ) -> bool {
        todo!("foo");
    }
}

pub mod backend_api {
    use tauri::Manager;

    use crate::{
        types::middleware_types::{SyscallDataType, SyscallRequest},
        APP_HANDLE,
    };

    pub fn syscall_input_request(pathname: &str, acquire_type: SyscallDataType) {
        if let Some(app_handle) = APP_HANDLE.lock().unwrap().as_ref() {
            loop {
                match app_handle.emit_all(
                    "syscall_request",
                    SyscallRequest {
                        path: pathname.to_string(),
                        syscall: acquire_type.to_string(),
                    },
                ) {
                    Ok(_) => break,
                    Err(_) => continue,
                }
            }
        } else {
            eprintln!("AppHandle is not initialized!");
        }
    }
}
