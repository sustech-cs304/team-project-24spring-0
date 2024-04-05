// Tab Manage
pub mod tab_mamagement {
    use crate::io::file_io;
    use crate::types::*;
    use tauri::State;

    #[tauri::command]
    pub fn create_tab(tab_map: State<TabMap>, file_path_str: &str) -> (bool, String) {
        match file_io::read_file(file_path_str) {
            Ok(data) => {
                todo!("Create Tab !!!");
                let mut tab;
                //let mut tab = Tab {
                //text: Box::new(Rope::from_str("")),
                //parser: Box::new(Parser::new()),
                //assembler: Box::new(Assembler::new()),
                //simulator: Box::new(Simulator::new()),
                //};
                tab_map.insert(file_path_str.to_string(), tab);
                (true, data)
            }
            Err(e) => (false, e),
        }
    }
}

pub mod frontend_api {
    use crate::io::file_io;

    #[tauri::command]
    pub fn read_file(file_path_str: &str) -> (bool, String) {
        match file_io::read_file(file_path_str) {
            Ok(data) => (true, data),
            Err(e) => (false, e),
        }
    }

    #[tauri::command]
    pub fn write_file(file_path_str: &str, data: &str) -> (bool, String) {
        match file_io::write_file(file_path_str, data) {
            Ok(_) => (true, "File saved".to_string()),
            Err(e) => (false, e),
        }
    }
}
