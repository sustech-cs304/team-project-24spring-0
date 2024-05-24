/// This module provides API functions for the frontend. Could be used by
/// `invoke` in EMCAScript
pub mod frontend_api {
    use tauri::State;

    use crate::{
        interface::parser::Parser,
        io::file_io,
        modules::riscv::basic::interface::{
            assembler::RiscVAssembler,
            parser::{RISCVExtension, RISCVParser, RISCV},
        },
        storage::rope_store,
        types::middleware_types::*,
        utility::{ptr::Ptr, state_helper::state},
    };

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
                    assembler: Box::new(RiscVAssembler::new()),
                    data_return_range: Default::default(),
                    assembly_cache: Default::default(),
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

    /// Sets the cursor position in the tab associated with the given file path.
    /// This could be useful for live sharing of code.
    /// - `tab_map`: Current state of all open tabs.
    /// - `filepath`: Path to the file associated with the tab to update.
    /// - `row`: Row number of the cursor.
    /// - `col`: Column number of the cursor.
    ///
    /// Returns `bool` indicating whether the cursor was successfully set.
    #[tauri::command]
    pub fn set_cursor(tab_map: State<TabMap>, filepath: &str, row: u64, col: u64) -> bool {
        let mut server = tab_map.rpc_server.lock().unwrap();
        if server.is_running() {
            server.set_host_cursor(row, col);
            true
        } else {
            false
        }
    }

    /// Updates the content of the tab associated with the given file path.
    /// - `cur_tab_name`: State containing the current tab name.
    /// - `tab_map`: Current state of all open tabs.
    /// - `op`: File operation to be performed.
    /// - `start`: Starting position of the content to be updated.
    /// - `end`: Ending position of the content to be updated.
    /// - `content`: New content to be inserted.
    ///
    /// Returns `Optional` indicating the success or failure of the update.
    #[tauri::command]
    pub fn modify_current_tab(
        cur_tab_name: State<CurTabName>,
        tab_map: State<TabMap>,
        op: FileOperation,
        start: CursorPosition,
        end: CursorPosition,
        content: &str,
    ) -> Optional {
        let filepath = cur_tab_name.name.lock().unwrap().clone();
        todo!("implement insert and live shared check");
        match tab_map.tabs.lock().unwrap().get_mut(&filepath) {
            Some(tab) => {
                tab.text.update_content(content);
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

    /// Sets the range [start, end] of data to be returned by the assembly and
    /// simulator for the current tab. Default is [0, 0].
    /// - `cur_tab_name`: State containing the current tab name.
    /// - `tab_map`: State containing the map of all tabs.
    /// - `start`: Start of the range (included).
    /// - `end`: End of the range (included).
    ///
    /// Returns `Vec<Data>` containing the data in the specified range.
    #[tauri::command]
    pub fn set_return_data_range(
        cur_tab_name: State<CurTabName>,
        tab_map: State<TabMap>,
        start: u64,
        end: u64,
    ) -> Vec<Data> {
        let name = cur_tab_name.name.lock().unwrap().clone();
        let mut lock = tab_map.tabs.lock().unwrap();
        let tab = lock.get_mut(&name).unwrap();
        tab.data_return_range = (start, end);
        {
            return vec![0; (end - start + 1) as usize];
        }
        todo!("call simulator to get data");
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
        let code = tab.text.to_string();
        let cache = &mut tab.assembly_cache;
        if cache.code != code {
            cache.parser_result = Default::default();
            cache.assembler_result = Default::default();
        }
        cache.code = code;
        if !parse(cache, &mut tab.parser) {
            AssembleResult::Error(cache.parser_result.clone().unwrap())
        } else if cache.assembler_result.is_some() {
            cache.assembler_result.clone().unwrap()
        } else {
            match tab.assembler.assemble(cache.parser_cache.clone().unwrap()) {
                Ok(res) => {
                    // todo!("load result to simulator");
                    cache.assembler_result = Some(AssembleResult::Success(AssembleSuccess {
                        data: res.data
                            [tab.data_return_range.0 as usize..=tab.data_return_range.1 as usize]
                            .to_vec(),
                        text: res
                            .instruction
                            .iter()
                            .map(|inst| Text {
                                line: inst.line_number + 1,
                                address: inst.address,
                                code: inst.code,
                                basic: Default::default(),
                            })
                            .collect(),
                    }));
                }
                Err(mut e) => {
                    cache.assembler_result = Some(AssembleResult::Error(
                        e.iter_mut()
                            .map(|err| AssembleError {
                                line: err.line as u64,
                                column: 0,
                                msg: std::mem::take(&mut err.msg),
                            })
                            .collect(),
                    ));
                }
            }
            cache.assembler_result.clone().unwrap()
        }
    }

    /// Placeholder for a function to dump data from all tabs.
    /// - `cur_tab_name`: State containing the current tab name.
    /// - `tab_map`: State containing the map of all tabs.
    ///
    /// Returns `DumpResult` indicating whether the dump was successful.
    #[tauri::command]
    pub fn dump(cur_tab_name: State<CurTabName>, tab_map: State<TabMap>) -> DumpResult {
        let name = cur_tab_name.name.lock().unwrap().clone();
        let mut lock = tab_map.tabs.lock().unwrap();
        let tab = lock.get_mut(&name).unwrap();
        let code = tab.text.to_string();
        let cache = &mut tab.assembly_cache;
        if cache.code != code {
            cache.parser_result = Default::default();
            cache.assembler_result = Default::default();
        }
        cache.code = code;
        if !parse(cache, &mut tab.parser) {
            return DumpResult::Error(cache.parser_result.clone().unwrap());
        }
        match tab.assembler.dump(cache.parser_cache.clone().unwrap()) {
            Ok(mem) => {
                for (ext, data) in [("text", &mem.text), ("data", &mem.data)] {
                    if let Some(e) =
                        file_io::write_file(tab.text.get_path().with_extension(ext).as_path(), data)
                    {
                        return DumpResult::Error(vec![AssembleError {
                            line: 0,
                            column: 0,
                            msg: e,
                        }]);
                    }
                }
                DumpResult::Success(())
            }
            Err(mut e) => DumpResult::Error(
                e.iter_mut()
                    .map(|err| AssembleError {
                        line: err.line as u64,
                        column: 0,
                        msg: std::mem::take(&mut err.msg),
                    })
                    .collect(),
            ),
        }
    }

    /// Run the code in the currently active tab in normal mode(won't stop at
    /// exist break point).
    /// - `cur_tab_name`: State containing the current tab name.
    /// - `tab_map`: State containing the map of all tabs.
    ///
    /// Returns `SimulatorResult` indicating whether the debug session was
    /// successfully started.
    #[tauri::command]
    pub fn run(cur_tab_name: State<CurTabName>, tab_map: State<TabMap>) -> SimulatorResult {
        {
            let name = cur_tab_name.name.lock().unwrap().clone();
            let mut lock = tab_map.tabs.lock().unwrap();
            let tab = lock.get_mut(&name).unwrap();
            return fack_simulator_result(&cur_tab_name, &tab_map);
        }
        todo!("Implement debug")
    }

    /// Run the code in the currently active tab in debug mode.
    /// - `cur_tab_name`: State containing the current tab name.
    /// - `tab_map`: State containing the map of all tabs.
    ///
    /// Returns `SimulatorResult` indicating whether the debug session was
    /// successfully started.
    #[tauri::command]
    pub fn debug(cur_tab_name: State<CurTabName>, tab_map: State<TabMap>) -> SimulatorResult {
        {
            let name = cur_tab_name.name.lock().unwrap().clone();
            let mut lock = tab_map.tabs.lock().unwrap();
            let tab = lock.get_mut(&name).unwrap();
            return fack_simulator_result(&cur_tab_name, &tab_map);
        }
        todo!("Implement debug")
    }

    /// Steps through the code in the currently active tab.
    /// - `cur_tab_name`: State containing the current tab name.
    /// - `tab_map`: State containing the map of all tabs.
    ///
    /// Returns `SimulatorResult` indicating whether the step was successful.
    #[tauri::command]
    pub fn step(cur_tab_name: State<CurTabName>, tab_map: State<TabMap>) -> SimulatorResult {
        {
            let name = cur_tab_name.name.lock().unwrap().clone();
            let mut lock = tab_map.tabs.lock().unwrap();
            let tab = lock.get_mut(&name).unwrap();
            return fack_simulator_result(&cur_tab_name, &tab_map);
        }
        todo!("Implement step")
    }

    /// Resets the state of the currently active tab's simulator.
    /// - `cur_tab_name`: State containing the current tab name.
    /// - `tab_map`: State containing the map of all tabs.
    ///
    /// Returns `SimulatorResult` indicating whether the reset was successful.
    #[tauri::command]
    pub fn reset(cur_tab_name: State<CurTabName>, tab_map: State<TabMap>) -> SimulatorResult {
        {
            let name = cur_tab_name.name.lock().unwrap().clone();
            let mut lock = tab_map.tabs.lock().unwrap();
            let tab = lock.get_mut(&name).unwrap();
            return fack_simulator_result(&cur_tab_name, &tab_map);
        }
        todo!("Implement reset")
    }

    /// Undoes the last instruction for current activate tab's simulator.
    /// - `cur_tab_name`: State containing the current tab name.
    /// - `tab_map`: State containing the map of all tabs.
    ///
    /// Returns `SimulatorResult` indicating whether the undo was successful.
    #[tauri::command]
    pub fn undo(cur_tab_name: State<CurTabName>, tab_map: State<TabMap>) -> SimulatorResult {
        {
            let name = cur_tab_name.name.lock().unwrap().clone();
            let mut lock = tab_map.tabs.lock().unwrap();
            let tab = lock.get_mut(&name).unwrap();
            return fack_simulator_result(&cur_tab_name, &tab_map);
        }
        todo!("Implement undo")
    }

    /// Sets a breakpoint at a specified line in the code of the current tab.
    /// - `tab_map`: State containing the map of all tabs.
    /// - `line`: Line number at which to set the breakpoint.
    ///
    /// Returns `bool` indicating whether the breakpoint was successfully set.
    #[tauri::command]
    pub fn set_breakpoint(tab_map: State<TabMap>, line: usize) -> bool {
        {
            return true;
        }
        todo!("Implement setBreakPoint")
    }

    /// Removes a breakpoint at a specified line in the code of the current tab.
    /// - `tab_map`: State containing the map of all tabs.
    /// - `line`: Line number at which to remove the breakpoint.
    ///
    /// Returns `bool` indicating whether the breakpoint was successfully
    /// removed.
    #[tauri::command]
    pub fn remove_breakpoint(tab_map: State<TabMap>, line: u64) -> bool {
        {
            return true;
        }
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
        {
            return true;
        }
        todo!("call simulator syscall_input with val");
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
        {
            return true;
        }
        todo!("Implement updateAssemblerSettings");
    }

    /// Starts the RPC server for the current tab.
    /// - `cur_tab_name`: State containing the current tab name.
    /// - `tab_map`: State containing the map of all tabs.
    /// - `password`: Password to be used for the RPC server.
    ///
    /// Returns `Optional` indicating the success or failure of the RPC server
    #[tauri::command]
    pub fn start_share_server(
        cur_tab_name: State<CurTabName>,
        tab_map: State<TabMap>,
        port: u16,
        password: &str,
    ) -> Optional {
        if tab_map.tabs.lock().unwrap().len() == 0 {
            return Optional {
                success: false,
                message: "No tab had been opened".to_string(),
            };
        }
        let mut rpc_lock = tab_map.rpc_server.lock().unwrap();
        rpc_lock.stop_service();
        rpc_lock.change_password(password);
        if let Err(e) = rpc_lock.set_port(port) {
            return Optional {
                success: false,
                message: e.to_string(),
            };
        } else if let Err(e) = rpc_lock.start_service(
            state::get_current_tab_name(&cur_tab_name),
            Ptr::new(&tab_map),
        ) {
            return Optional {
                success: false,
                message: e.to_string(),
            };
        } else {
            return Optional {
                success: true,
                message: String::new(),
            };
        }
    }

    /// Authorize and connect to a remote RPC server as client.
    /// - `cur_tab_name`: State containing the current tab name.
    /// - `tab_map`: State containing the map of all tabs.
    /// - `ip`: IPV4 address of the remote server.
    /// - `port`: Port number of the remote server.
    /// - `password`: Password to be used for the connection.
    ///
    /// Returns `Optional` indicating the success or failure of the connection.
    #[tauri::command]
    pub fn authorize(
        cur_tab_name: State<CurTabName>,
        tab_map: State<TabMap>,
        ip: String,
        port: u16,
        password: String,
    ) -> Optional {
        {
            return Optional {
                success: true,
                message: "".to_string(),
            };
        }
        todo!("Implement authorize as client");
    }

    /// Stop the share server for the current tab.
    /// - `tab_map`: State containing the map of all tabs.
    ///
    /// Returns `bool` indicating the success or failure, failure means the
    /// server is not running.
    #[tauri::command]
    pub fn stop_share_server(tab_map: State<TabMap>) -> bool {
        let mut server = tab_map.rpc_server.lock().unwrap();
        if !server.is_running() {
            false
        } else {
            server.stop_service();
            true
        }
    }

    /// helper function
    fn parse(cache: &mut AssembleCache, parser: &mut Box<dyn Parser<RISCV>>) -> bool {
        if cache.parser_cache.is_some() {
            true
        } else if cache.parser_result.is_some() {
            false
        } else {
            match parser.parse(&cache.code) {
                Ok(res) => {
                    cache.parser_cache = Some(res);
                    cache.parser_result = None;
                    true
                }
                Err(mut e) => {
                    cache.parser_cache = None;
                    cache.parser_result = Some(
                        e.iter_mut()
                            .map(|err| AssembleError {
                                line: err.pos.0 as u64,
                                column: err.pos.1 as u64,
                                msg: std::mem::take(&mut err.msg),
                            })
                            .collect(),
                    );
                    false
                }
            }
        }
    }

    fn fack_simulator_result(
        cur_tab_name: &State<CurTabName>,
        tab_map: &State<TabMap>,
    ) -> SimulatorResult {
        let name = cur_tab_name.name.lock().unwrap().clone();
        let mut lock = tab_map.tabs.lock().unwrap();
        let tab = lock.get_mut(&name).unwrap();
        return SimulatorResult {
            success: true,
            has_current_text: false,
            current_text: 0,
            registers: vec![
                Register {
                    name: "x0".to_string(),
                    number: "0".to_string(),
                    value: 0,
                },
                Register {
                    name: "pc".to_string(),
                    number: "".to_string(),
                    value: 0,
                },
            ],
            data: vec![0; (tab.data_return_range.1 - tab.data_return_range.0 + 1) as usize],
            has_message: true,
            message: "test".to_string(),
        };
    }
}

/// This module provides API functions for the backend of a Tauri application
/// to emit event to the frontend, and the frontend needs to handle the event by
/// `listen`.
pub mod backend_api {
    use tauri::Manager;

    use crate::{
        types::middleware_types::{SyscallDataType, SyscallOutput, SyscallRequest},
        APP_HANDLE,
    };

    /// Emits a print syscall output event to the frontend.
    /// - `pathname`: Identifier for the tab to which the output should be sent.
    /// - `output`: Output to be printed.
    ///
    /// Returns `Option` containing an error if the event could not be emitted.
    ///
    /// This function will emit a `front_syscall_print` event to the frontend,
    /// and the payload is a `SyscallOutput` containing the filepath and output
    /// to be printed.
    ///
    /// SyscallOutput:
    /// - `filepath`: string
    /// - `data`: string
    pub fn syscall_output_print(
        pathname: &str,
        output: &str,
    ) -> Option<Box<dyn std::error::Error>> {
        if let Some(app_handle) = APP_HANDLE.lock().unwrap().as_ref() {
            if let Ok(_) = app_handle.emit_all(
                "front_syscall_print",
                SyscallOutput {
                    filepath: pathname.to_string(),
                    data: output.to_string(),
                },
            ) {
                None
            } else {
                Some(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to emit syscall output print event!",
                )))
            }
        } else {
            Some(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "AppHandle is not initialized!",
            )))
        }
    }

    /// Emits a syscall input request event to the frontend.
    /// - `pathname`: Identifier for the tab to which the request should be
    ///   sent.
    /// - `acquire_type`: Type of the input to be acquired.
    ///
    /// Returns `Option` containing an error if the event could not be emitted.
    ///
    /// This function will emit a `front_syscall_request` event to the frontend,
    /// and the payload is one of the following string:
    /// - "Int"
    /// - "Float"
    /// - "Double"
    /// - "String"
    /// - "Char"
    /// - "Long"
    pub fn syscall_input_request(
        pathname: &str,
        acquire_type: SyscallDataType,
    ) -> Option<Box<dyn std::error::Error>> {
        if let Some(app_handle) = APP_HANDLE.lock().unwrap().as_ref() {
            if let Ok(_) = app_handle.emit_all(
                "front_syscall_request",
                SyscallRequest {
                    path: pathname.to_string(),
                    syscall: acquire_type.to_string(),
                },
            ) {
                None
            } else {
                Some(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to emit syscall input request event!",
                )))
            }
        } else {
            Some(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "AppHandle is not initialized!",
            )))
        }
    }
}
