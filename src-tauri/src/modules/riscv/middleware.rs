/// This module provides API functions for the frontend. Could be used by
/// `invoke` in EMCAScript
pub mod frontend_api {
    use std::{net::SocketAddr, path::Path};

    use tauri::{async_runtime::block_on, State, Window};

    use crate::{
        interface::parser::Parser,
        io::file_io,
        modules::riscv::basic::interface::{
            assembler::RiscVAssembler,
            parser::{RISCVExtension, RISCVParser, RISCV},
        },
        remote::{Modification, OpRange},
        simulator::simulator::RISCVSimulator,
        storage::rope_store,
        types::middleware_types::*,
        utility::{
            ptr::Ptr,
            state_helper::state::{self, set_current_tab_name},
        },
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
                    simulator: Box::new(RISCVSimulator::new(filepath)),
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
                message: e.to_string(),
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
        match tab_map.tabs.lock().unwrap().get_mut(&filepath) {
            Some(tab) => {
                match tab.text.handle_modify(&Modification {
                    op: op.into(),
                    op_range: OpRange { start, end },
                    version: tab.text.get_version() as u64,
                    modified_content: content.to_owned(),
                }) {
                    Ok(_) => Optional {
                        success: true,
                        message: String::new(),
                    },
                    Err(e) => Optional {
                        success: false,
                        message: e.to_string(),
                    },
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
                message: e.to_string(),
            },
        }
    }

    /// Writes data in specific tab to the file path.
    /// - `filepath`: Path to the file where data should be written.
    /// - `data`: Content to write to the file.
    ///
    /// Returns `Optional` indicating the success or failure of the write
    /// operation.
    #[tauri::command]
    pub fn write_tab(filepath: &str, data: &str) -> Optional {
        match file_io::write_file_str(filepath, data) {
            Ok(_) => Optional {
                success: true,
                message: String::new(),
            },
            Err(e) => Optional {
                success: false,
                message: e.to_string(),
            },
        }
    }

    /// Sets the range [start, end] of data to be returned by the assembly and
    /// simulator for the current tab. Default is [0, 0].
    /// - `cur_tab_name`: State containing the current tab name.
    /// - `tab_map`: State containing the map of all tabs.
    /// - `start`: Start of the range (aligned to 4 bytes)
    /// - `len`: Length of the range (aligned to 4 bytes)
    ///
    /// Returns `Optional` indicating the success or failure of the operation.
    #[tauri::command]
    pub fn set_return_data_range(
        cur_tab_name: State<CurTabName>,
        tab_map: State<TabMap>,
        range: MemoryReturnRange,
    ) -> Optional {
        let name = cur_tab_name.name.lock().unwrap().clone();
        let mut lock = tab_map.tabs.lock().unwrap();
        let tab = lock.get_mut(&name).unwrap();
        match tab.simulator.set_memory_return_range(range) {
            Ok(_) => Optional {
                success: true,
                message: String::new(),
            },
            Err(e) => Optional {
                success: false,
                message: e.to_string(),
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
                    if let Err(e) = tab.simulator.load_inst(res) {
                        return AssembleResult::Error(vec![AssembleError {
                            line: 0,
                            column: 0,
                            msg: e.to_string(),
                        }]);
                    }
                    cache.assembler_result = Some(AssembleResult::Success(AssembleSuccess {
                        text: tab
                            .simulator
                            .get_raw_inst()
                            .as_ref()
                            .unwrap()
                            .instruction
                            .iter()
                            .map(|inst| AssembleText {
                                line: inst.line_number,
                                address: inst.address,
                                code: inst.code,
                                basic: inst.basic.to_string(),
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
                    if let Err(e) =
                        file_io::write_file(tab.text.get_path().with_extension(ext).as_path(), data)
                    {
                        return DumpResult::Error(vec![AssembleError {
                            line: 0,
                            column: 0,
                            msg: e.to_string(),
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
    /// Returns `Optional` indicating whether the run session was successfully
    /// started.
    #[tauri::command]
    pub fn run(cur_tab_name: State<CurTabName>, tab_map: State<TabMap>) -> Optional {
        let name = cur_tab_name.name.lock().unwrap().clone();
        let mut lock = tab_map.tabs.lock().unwrap();
        let tab = lock.get_mut(&name).unwrap();
        match tab.simulator.run() {
            Ok(_) => Optional {
                success: true,
                message: String::new(),
            },
            Err(e) => Optional {
                success: false,
                message: e.to_string(),
            },
        }
    }

    /// Run the code in the currently active tab in debug mode.
    /// - `cur_tab_name`: State containing the current tab name.
    /// - `tab_map`: State containing the map of all tabs.
    ///
    /// Returns `Optional` indicating whether the debug session was successfully
    /// started.
    #[tauri::command]
    pub fn debug(cur_tab_name: State<CurTabName>, tab_map: State<TabMap>) -> Optional {
        let name = cur_tab_name.name.lock().unwrap().clone();
        let mut lock = tab_map.tabs.lock().unwrap();
        let tab = lock.get_mut(&name).unwrap();
        match tab.simulator.debug() {
            Ok(_) => Optional {
                success: true,
                message: String::new(),
            },
            Err(e) => Optional {
                success: false,
                message: e.to_string(),
            },
        }
    }

    /// Stops the currently active tab's simulator.
    /// - `cur_tab_name`: State containing the current tab name.
    /// - `tab_map`: State containing the map of all tabs.
    ///
    /// Returns `Optional` indicating whether the stop was successful.
    #[tauri::command]
    pub fn stop(cur_tab_name: State<CurTabName>, tab_map: State<TabMap>) -> Optional {
        let name = cur_tab_name.name.lock().unwrap().clone();
        let mut lock = tab_map.tabs.lock().unwrap();
        let tab = lock.get_mut(&name).unwrap();
        match tab.simulator.stop() {
            Ok(_) => Optional {
                success: true,
                message: String::new(),
            },
            Err(e) => Optional {
                success: false,
                message: e.to_string(),
            },
        }
    }

    /// Resumes the currently active tab's simulator.
    /// - `cur_tab_name`: State containing the current tab name.
    /// - `tab_map`: State containing the map of all tabs.
    ///
    /// Returns `Optional` indicating whether the resume was successful.
    #[tauri::command]
    pub fn resume(cur_tab_name: State<CurTabName>, tab_map: State<TabMap>) -> Optional {
        let name = cur_tab_name.name.lock().unwrap().clone();
        let mut lock = tab_map.tabs.lock().unwrap();
        let tab = lock.get_mut(&name).unwrap();
        match tab.simulator.resume() {
            Ok(_) => Optional {
                success: true,
                message: String::new(),
            },
            Err(e) => Optional {
                success: false,
                message: e.to_string(),
            },
        }
    }

    /// Steps through the code in the currently active tab.
    /// - `cur_tab_name`: State containing the current tab name.
    /// - `tab_map`: State containing the map of all tabs.
    ///
    /// Returns `Optional` indicating whether the step was successful.
    #[tauri::command]
    pub fn step(cur_tab_name: State<CurTabName>, tab_map: State<TabMap>) -> Optional {
        let name = cur_tab_name.name.lock().unwrap().clone();
        let mut lock = tab_map.tabs.lock().unwrap();
        let tab = lock.get_mut(&name).unwrap();
        match tab.simulator.step() {
            Ok(_) => Optional {
                success: true,
                message: String::new(),
            },
            Err(e) => Optional {
                success: false,
                message: e.to_string(),
            },
        }
    }

    /// Resets the state of the currently active tab's simulator.
    /// - `cur_tab_name`: State containing the current tab name.
    /// - `tab_map`: State containing the map of all tabs.
    ///
    /// Returns `Optional` indicating whether the reset was successful.
    #[tauri::command]
    pub fn reset(cur_tab_name: State<CurTabName>, tab_map: State<TabMap>) -> Optional {
        let name = cur_tab_name.name.lock().unwrap().clone();
        let mut lock = tab_map.tabs.lock().unwrap();
        let tab = lock.get_mut(&name).unwrap();
        match tab.simulator.reset() {
            Ok(_) => Optional {
                success: true,
                message: String::new(),
            },
            Err(e) => Optional {
                success: false,
                message: e.to_string(),
            },
        }
    }

    /// Undoes the last instruction for current activate tab's simulator.
    /// - `cur_tab_name`: State containing the current tab name.
    /// - `tab_map`: State containing the map of all tabs.
    ///
    /// Returns `Optional` indicating whether the undo was successful.
    #[tauri::command]
    pub fn undo(cur_tab_name: State<CurTabName>, tab_map: State<TabMap>) -> Optional {
        let name = cur_tab_name.name.lock().unwrap().clone();
        let mut lock = tab_map.tabs.lock().unwrap();
        let tab = lock.get_mut(&name).unwrap();
        match tab.simulator.undo() {
            Ok(_) => Optional {
                success: true,
                message: String::new(),
            },
            Err(e) => Optional {
                success: false,
                message: e.to_string(),
            },
        }
    }

    /// Sets a breakpoint at a specified line in the code of the current tab.
    /// - `cur_tab_name`: State containing the current tab name.
    /// - `tab_map`: State containing the map of all tabs.
    /// - `line`: Line number at which to set the breakpoint.
    ///
    /// Returns `Optional` indicating whether the breakpoint was successfully
    /// set.
    #[tauri::command]
    pub fn set_breakpoint(
        cur_tab_name: State<CurTabName>,
        tab_map: State<TabMap>,
        line: u64,
    ) -> Optional {
        let name = cur_tab_name.name.lock().unwrap().clone();
        let mut lock = tab_map.tabs.lock().unwrap();
        let tab = lock.get_mut(&name).unwrap();
        match tab.simulator.set_breakpoint(line as usize) {
            Ok(_) => Optional {
                success: true,
                message: String::new(),
            },
            Err(e) => Optional {
                success: false,
                message: e.to_string(),
            },
        }
    }

    /// Removes a breakpoint at a specified line in the code of the current tab.
    /// - `cur_tab_name`: State containing the current tab name.
    /// - `tab_map`: State containing the map of all tabs.
    /// - `line`: Line number at which to remove the breakpoint.
    ///
    /// Returns `Optional` indicating whether the breakpoint was successfully
    /// removed.
    #[tauri::command]
    pub fn remove_breakpoint(
        cur_tab_name: State<CurTabName>,
        tab_map: State<TabMap>,
        line: u64,
    ) -> Optional {
        let name = cur_tab_name.name.lock().unwrap().clone();
        let mut lock = tab_map.tabs.lock().unwrap();
        let tab = lock.get_mut(&name).unwrap();
        match tab.simulator.remove_breakpoint(line as usize) {
            Ok(_) => Optional {
                success: true,
                message: String::new(),
            },
            Err(e) => Optional {
                success: false,
                message: e.to_string(),
            },
        }
    }

    /// Send a syscall input to current tab's simulator.
    /// - `cur_tab_name`: State containing the current tab name.
    /// - `tab_map`: State containing the map of all tabs.
    /// - `val`: Value of the input as a string.
    ///
    /// Returns `Optional` indicating whether the syscall input was successfully
    #[tauri::command]
    pub fn syscall_input(
        cur_tab_name: State<CurTabName>,
        tab_map: State<TabMap>,
        val: String,
    ) -> Optional {
        let name = cur_tab_name.name.lock().unwrap().clone();
        let mut lock = tab_map.tabs.lock().unwrap();
        let tab = lock.get_mut(&name).unwrap();
        match tab.simulator.syscall_input(&val) {
            Ok(_) => Optional {
                success: true,
                message: String::new(),
            },
            Err(e) => Optional {
                success: false,
                message: e.to_string(),
            },
        }
    }

    /// Updates the assembler settings for the current tab.
    /// - `cur_tab_name`: State containing the current tab name.
    /// - `tab_map`: State containing the map of all tabs.
    /// - `settings`: New assembler settings to be applied.
    ///
    /// Returns `Optional` indicating whether the settings were successfully
    /// updated.
    #[tauri::command]
    pub fn update_assembler_settings(
        cur_tab_name: State<CurTabName>,
        tab_map: State<TabMap>,
        settings: AssemblerConfig,
    ) -> Optional {
        let name = cur_tab_name.name.lock().unwrap().clone();
        let mut lock = tab_map.tabs.lock().unwrap();
        let tab = lock.get_mut(&name).unwrap();
        if let Err(e) = tab.simulator.update_config(&settings) {
            return Optional {
                success: false,
                message: e.to_string(),
            };
        }
        tab.assembler.update_config(&settings);
        Optional {
            success: true,
            message: String::new(),
        }
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
        let mut server_lock = tab_map.rpc_server.lock().unwrap();
        if server_lock.is_running() {
            return Optional {
                success: false,
                message: "Server already running".to_string(),
            };
        } else if let Err(e) = server_lock.set_port(port) {
            return Optional {
                success: false,
                message: e.to_string(),
            };
        }

        server_lock.change_password(password);
        if let Err(e) = server_lock.start_server(
            state::get_current_tab_name(&cur_tab_name),
            Ptr::new(&tab_map),
        ) {
            Optional {
                success: false,
                message: e.to_string(),
            }
        } else {
            Optional {
                success: true,
                message: String::new(),
            }
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
    pub fn authorize_share_client(
        window: Window,
        cur_tab_name: State<CurTabName>,
        tab_map: State<TabMap>,
        ip: String,
        port: u16,
        password: String,
    ) -> Optional {
        let mut client = tab_map.rpc_client.lock().unwrap();
        let addr: SocketAddr = match format!("{}:{}", ip, port).parse() {
            Ok(val) => val,
            Err(_) => {
                return Optional {
                    success: false,
                    message: "Invalid IP or port".to_string(),
                };
            }
        };

        if let Err(e) = client.set_server_addr(addr) {
            return Optional {
                success: false,
                message: e.to_string(),
            };
        } else if let Err(e) = client.start() {
            return Optional {
                success: false,
                message: e.to_string(),
            };
        }

        match block_on(client.send_authorize(&password)) {
            Ok(val) => {
                let client_text = rope_store::Text::from_str(Path::new(&val.0), &val.2);
                let client_tab = Tab {
                    text: Box::new(client_text),
                    parser: Box::new(RISCVParser::new(&vec![RISCVExtension::RV32I])),
                    assembler: Box::new(RiscVAssembler::new()),
                    simulator: Box::new(RISCVSimulator::new(&val.0)),
                    assembly_cache: Default::default(),
                };
                tab_map
                    .tabs
                    .lock()
                    .unwrap()
                    .insert(val.0.clone(), client_tab);
                set_current_tab_name(&cur_tab_name, &val.0);
                if let Err(e) = window.emit("front_share_client", {}) {
                    return Optional {
                        success: false,
                        message: e.to_string(),
                    };
                } else {
                    Optional {
                        success: true,
                        message: String::new(),
                    }
                }
            }
            Err(e) => Optional {
                success: false,
                message: e.to_string(),
            },
        }
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
            server.stop_server();
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
}

/// This module provides API functions for the backend of a Tauri application
/// to emit event to the frontend, and the frontend needs to handle the event by
/// `listen`.
pub mod backend_api {
    use strum::VariantArray;
    use tauri::Manager;

    use crate::{
        interface::simulator::Simulator,
        modules::riscv::basic::interface::parser::RV32IRegister,
        types::middleware_types::{
            Optional,
            Register,
            SimulatorData,
            SyscallOutput,
            SyscallRequest,
        },
        APP_HANDLE,
    };

    /// Emits a simulator update event to the frontend.
    /// - `simulator`: Simulator instance to update its state.
    /// - `simulator_res`: Result of the simulator operation.
    ///
    /// Returns `Result` indicating the success or failure of the event
    /// emission.
    ///
    /// This function will emit a `front_simulator_update` event to the
    /// frontend, and the payload is a `SimulatorData` containing the
    /// current pc index, register and memory values.
    ///
    /// [SimulatorData](crate::types::middleware_types::SimulatorData):
    /// - `filepath`: string
    /// - `success`: bool
    /// - `has_current_text`: bool
    /// - `current_text`: u64
    /// - `registers`: Vec<[Register](crate::types::middleware_types::Register)>
    /// - `data`: Vec<u32>
    /// - `message`: string
    pub fn simulator_update(
        simulator: &mut dyn Simulator,
        simulator_res: Optional,
    ) -> Result<(), String> {
        if let Some(app_handle) = APP_HANDLE.lock().unwrap().as_ref() {
            if let Ok(_) = app_handle.emit_all(
                "front_simulator_update",
                SimulatorData {
                    filepath: simulator.get_filepath().to_string(),
                    success: simulator_res.success,
                    has_current_text: simulator.get_pc_idx().is_some(),
                    current_text: simulator.get_pc_idx().unwrap_or(0) as u64,
                    registers: simulator
                        .get_register()
                        .iter()
                        .enumerate()
                        .map(|(i, &val)| Register {
                            name: RV32IRegister::VARIANTS[i].to_string(),
                            number: i.to_string(),
                            value: val as u64,
                        })
                        .collect(),
                    data: simulator.get_memory(),
                    message: simulator_res.message,
                },
            ) {
                Ok(())
            } else {
                Err("Failed to emit simulator update event!".to_string())
            }
        } else {
            Err("AppHandle is not initialized!".to_string())
        }
    }

    /// Emits a print syscall output event to the frontend.
    /// - `pathname`: Identifier for the tab to which the output should be sent.
    /// - `output`: Output to be printed.
    ///
    /// Returns `Result` indicating the success or failure of the event
    /// emission.
    ///
    /// This function will emit a `front_syscall_print` event to the frontend,
    /// and the payload is a `SyscallOutput` containing the filepath and output
    /// to be printed.
    ///
    /// [SyscallOutput](crate::types::middleware_types::SyscallOutput):
    /// - `filepath`: string
    /// - `data`: string
    pub fn syscall_output_print(pathname: &str, output: &str) -> Result<(), String> {
        if let Some(app_handle) = APP_HANDLE.lock().unwrap().as_ref() {
            if let Ok(_) = app_handle.emit_all(
                "front_syscall_print",
                SyscallOutput {
                    filepath: pathname.to_string(),
                    data: output.to_string(),
                },
            ) {
                Ok(())
            } else {
                Err("Failed to emit syscall output print event!".to_string())
            }
        } else {
            Err("AppHandle is not initialized!".to_string())
        }
    }

    /// Emits a print syscall output event to the frontend.
    /// - `pathname`: Identifier for the tab to which the output should be sent.
    ///
    /// Returns `Result` indicating the success or failure of the event
    /// emission.
    ///
    /// This function will emit a `front_syscall_request` event to the frontend,
    /// and the payload is a `SyscallRequest` containing the filepath.
    ///
    /// [SyscallRequest](crate::types::middleware_types::SyscallRequest):
    /// - `filepath`: string
    pub fn syscall_input_request(pathname: &str) -> Result<(), String> {
        if let Some(app_handle) = APP_HANDLE.lock().unwrap().as_ref() {
            if let Ok(_) = app_handle.emit_all(
                "front_syscall_request",
                SyscallRequest {
                    filepath: pathname.to_string(),
                },
            ) {
                Ok(())
            } else {
                Err("Failed to emit syscall input request event!".to_string())
            }
        } else {
            Err("AppHandle is not initialized!".to_string())
        }
    }
}
