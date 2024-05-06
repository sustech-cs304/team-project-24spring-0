/// Module providing API functions for the frontend of a Tauri application.
/// Could be used by `invoke`
pub mod frontend_api {
    use crate::io::file_io;
    use crate::modules::riscv::basic::interface::parser::{RISCVExtension, RISCVParser};
    use crate::storage::rope_store;
    use crate::types::middleware_types::{
        AssembleResult, AssemblerConfig, CurTabName, Optional, SyscallDataType, Tab, TabMap,
    };
    use tauri::State;

    /// Creates a new tab with content loaded from a specified file path.
    /// - `tab_map`: Current state of all open tabs.
    /// - `filepath`: Path to the file from which content will be loaded.
    ///
    /// Returns `Optional` indicating the success or failure of tab creation.
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

    /// Closes the tab associated with the given file path.
    /// - `cur_name`: Current name of the tab in focus.
    /// - `tab_map`: Current state of all open tabs.
    /// - `filepath`: Path to the file associated with the tab to close.
    ///
    /// Returns `Optional` indicating the success or failure of tab close if
    /// success, return the new tab to focus on, else return error message.
    #[tauri::command]
    pub fn close_tab(
        cur_tab_name: State<CurTabName>,
        tab_map: State<TabMap>,
        filepath: &str,
    ) -> Optional {
        if *cur_tab_name.name.lock().unwrap() == filepath {
            let lock = tab_map.tabs.lock().unwrap();
            if lock.len() == 1 {
                return Optional {
                    success: true,
                    message: "".to_string(),
                };
            } else {
                let mut iter = lock.iter();
                loop {
                    let (new_name, _) = iter.next().unwrap();
                    if new_name != filepath {
                        *cur_tab_name.name.lock().unwrap() = new_name.clone();
                        break;
                    }
                }
            }
        }
        match tab_map.tabs.lock().unwrap().remove(filepath) {
            Some(_) => Optional {
                success: true,
                message: cur_tab_name.name.lock().unwrap().clone(),
            },
            None => Optional {
                success: false,
                message: "Tab not found".to_string(),
            },
        }
    }

    /// Changes the current tab to the one specified by the new path.
    /// - `cur_name`: Current name of the tab in focus.
    /// - `newpath`: Path to the file associated with the new tab to focus.
    /// - `tab_map`: Current state of all open tabs.
    ///
    /// Returns `bool` indicating whether the operation was successful.
    /// The only case where it would fail is if the tab with the specified
    /// path does not exist in opened tabs.
    #[tauri::command]
    pub fn change_current_tab(
        cur_tab_name: State<CurTabName>,
        tab_map: State<TabMap>,
        newpath: &str,
    ) -> bool {
        let lock = tab_map.tabs.lock().unwrap();
        if lock.contains_key(newpath) {
            *cur_tab_name.name.lock().unwrap() = newpath.to_string();
            true
        } else {
            false
        }
    }

    /// Updates the content of the tab associated with the given file path.
    /// - `tab_map`: Current state of all open tabs.
    /// - `filepath`: Path to the file associated with the tab to update.
    /// - `data`: New content to replace the existing content of the tab.
    /// Returns `Optional` indicating the success or failure of the update.
    #[tauri::command]
    pub fn update_tab(tab_map: State<TabMap>, filepath: &str, data: &str) -> Optional {
        match tab_map.tabs.lock().unwrap().get_mut(filepath) {
            Some(tab) => {
                tab.text.update_content(data);
                tab.text.set_dirty(true);
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

    /// Reads the content of a tab from the file at the specified path.
    /// - `filepath`: Path to the file to read.
    ///
    /// Returns `Optional` containing the file content if successful, or an
    /// error message if not.
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

    /// Writes data in specific tab to the file file path.
    /// - `filepath`: Path to the file where data should be written.
    /// - `data`: Content to write to the file.
    ///
    /// Returns `Optional` indicating the success or failure of the write
    /// operation.
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

    /// Assembles the code in the currently active tab.
    /// - `cur_tab_name`: State containing the current tab name.
    /// - `tab_map`: State containing the map of all tabs.
    ///
    /// Returns `AssembleResult` indicating the outcome of the assembly process.
    #[tauri::command]
    pub fn assembly(cur_tab_name: State<CurTabName>, tab_map: State<TabMap>) -> AssembleResult {
        let name = cur_tab_name.name.lock().unwrap().clone();
        let mut lock = tab_map.tabs.lock().unwrap();
        let tab = lock.get_mut(&name).unwrap();
        match tab.parser.parse(tab.text.to_string()) {
            Ok(ir) => {
                //TODO
                AssembleResult {
                    success: true,
                    error: Default::default(),
                }
            }
            Err(e) => AssembleResult {
                success: false,
                error: e,
            },
        }
    }

    /// Placeholder for a function to dump data from all tabs.
    /// - `cur_tab_name`: State containing the current tab name.
    /// - `tab_map`: State containing the map of all tabs.
    ///
    /// Returns `bool` indicating whether the dump was successful.
    #[tauri::command]
    pub fn dump(cur_tab_name: State<CurTabName>, tab_map: State<TabMap>) -> bool {
        todo!("Implement dump")
    }

    /// Run the code in the currently active tab in debug mode.
    /// - `cur_tab_name`: State containing the current tab name.
    /// - `tab_map`: State containing the map of all tabs.
    ///
    /// Returns `bool` indicating whether the debug session was successfully
    /// started.
    #[tauri::command]
    pub fn debug(cur_tab_name: State<CurTabName>, tab_map: State<TabMap>) -> bool {
        todo!("Implement debug")
    }

    /// Steps through the code in the currently active tab.
    /// - `cur_tab_name`: State containing the current tab name.
    /// - `tab_map`: State containing the map of all tabs.
    ///
    /// Returns `bool` indicating whether the step was successful.
    #[tauri::command]
    pub fn step(cur_tab_name: State<CurTabName>, tab_map: State<TabMap>) -> bool {
        todo!("Implement step")
    }

    /// Resets the state of the currently active tab's simulator.
    /// - `cur_tab_name`: State containing the current tab name.
    /// - `tab_map`: State containing the map of all tabs.
    ///
    /// Returns `bool` indicating whether the reset was successful.
    #[tauri::command]
    pub fn reset(cur_tab_name: State<CurTabName>, tab_map: State<TabMap>) -> bool {
        todo!("Implement reset")
    }

    /// Undoes the last instruction for current activate tab's simulator.
    /// - `cur_tab_name`: State containing the current tab name.
    /// - `tab_map`: State containing the map of all tabs.
    ///
    /// Returns `bool` indicating whether the undo was successful.
    #[tauri::command]
    pub fn undo(cur_tab_name: State<CurTabName>, tab_map: State<TabMap>) -> bool {
        todo!("Implement undo")
    }

    /// Sets a breakpoint at a specified line in the code of the current tab.
    /// - `tab_map`: State containing the map of all tabs.
    /// - `line`: Line number at which to set the breakpoint.
    ///
    /// Returns `bool` indicating whether the breakpoint was successfully set.
    #[tauri::command]
    pub fn set_breakpoint(tab_map: State<TabMap>, line: usize) -> bool {
        //todo!("Implement setBreakPoint")
        true
    }

    /// Removes a breakpoint at a specified line in the code of the current tab.
    /// - `tab_map`: State containing the map of all tabs.
    /// - `line`: Line number at which to remove the breakpoint.
    ///
    /// Returns `bool` indicating whether the breakpoint was successfully
    /// removed.
    #[tauri::command]
    pub fn remove_breakpoint(tab_map: State<TabMap>, line: u64) -> bool {
        todo!("Implement removeBreakPoint")
    }

    /// Send a syscall input to current tab's simulator.
    /// - `cur_tab_name`: State containing the current tab name.
    /// - `tab_map`: State containing the map of all tabs.
    /// - `inputType`: Type of the input, should be one of the following:
    ///    - "Int"
    ///    - "Float"
    ///    - "Double"
    ///    - "String"
    ///    - "Char"
    ///    - "Long"
    /// - `val`: Value of the input as a string.
    ///
    /// Returns `bool` indicating whether the syscall input was successfully
    #[tauri::command]
    #[allow(non_snake_case)]
    pub fn syscall_input(
        cur_tab_name: State<CurTabName>,
        tab_map: State<TabMap>,
        inputType: &str,
        val: String,
    ) -> bool {
        let name = cur_tab_name.name.lock().unwrap().clone();
        let mut lock = tab_map.tabs.lock().unwrap();
        let tab = lock.get_mut(&name).unwrap();
        let val = match inputType {
            "Int" => SyscallDataType::Int(val.parse::<i32>().unwrap()),
            "Float" => SyscallDataType::Float(val.parse::<f32>().unwrap()),
            "Double" => SyscallDataType::Double(val.parse::<f64>().unwrap()),
            "String" => SyscallDataType::String(val.bytes().collect()),
            "Char" => SyscallDataType::Char(val.bytes().next().unwrap()),
            "Long" => SyscallDataType::Long(val.parse::<i64>().unwrap()),
            _ => return false,
        };
        //TODO
        //tab.parser.syscall_input_request(v);
        true
    }

    /// Updates the assembler settings for the current tab.
    /// - `cur_tab_name`: State containing the current tab name.
    /// - `tab_map`: State containing the map of all tabs.
    /// - `settings`: New assembler settings to be applied.
    ///
    /// Returns `bool` indicating whether the settings were successfully
    /// updated.
    #[tauri::command]
    pub fn update_assembler_settings(
        cur_tab_name: State<CurTabName>,
        tab_map: State<TabMap>,
        settings: AssemblerConfig,
    ) -> bool {
        todo!("foo");
    }
}

pub mod backend_api {
    use crate::types::middleware_types::{SyscallDataType, SyscallRequest};
    use crate::APP_HANDLE;
    use tauri::Manager;

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
