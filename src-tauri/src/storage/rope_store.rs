use std::{
    error::Error,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::SystemTime,
};

use ropey::Rope;

use crate::{
    dprintln,
    interface::storage::{
        BasicFile,
        FileShareStatus::{self, Client, Private, Server},
        HistorianFile,
        MFile,
    },
    io::file_io,
    remote::{server::editor_rpc::OperationType, Modification},
    types::{rpc_types::CursorList, ResultVoid},
    utility::text_helper::{all_to_lf, lines_count},
    CURSOR_LIST,
    HISTORY,
};

pub struct ConcurrencyShare {
    mutex: Mutex<()>,
    update_thread: Option<std::thread::JoinHandle<()>>,
    cursor_list: Option<Arc<Mutex<CursorList>>>,
}

pub struct Text {
    share_status: FileShareStatus,
    data: Box<Rope>,
    path: PathBuf,
    version: usize,
    dirty: bool,
    last_modified: SystemTime,
    concurrent_share: ConcurrencyShare,
}

impl BasicFile<Rope, Modification> for Text {
    fn get_path(&self) -> &PathBuf {
        &self.path
    }
    fn get_path_str(&self) -> String {
        self.path.to_str().unwrap().to_string()
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    fn to_string(&self) -> String {
        self.data.as_ref().to_string()
    }

    fn save(&mut self) -> ResultVoid {
        file_io::write_file(self.path.as_path(), &self.data.as_ref().to_string())?;
        self.dirty = false;
        Ok(())
    }

    fn get_raw(&mut self) -> &mut Rope {
        self.data.as_mut()
    }

    fn handle_modify(&mut self, modify: &Modification) -> ResultVoid {
        dprintln!("handle_modify, version {}", self.version);
        let modified_content = all_to_lf(&modify.modified_content);
        match &self.share_status {
            Private => {
                let raw_rope = self.data.as_mut();
                let range = &modify.op_range;
                let start_idx =
                    raw_rope.line_to_char(range.start.row as usize) + range.start.col as usize;
                let end_idx =
                    raw_rope.line_to_char(range.end.row as usize) + range.end.col as usize;

                match modify.op {
                    OperationType::Insert => {
                        raw_rope.insert(start_idx, &modified_content);
                    }
                    OperationType::Delete => {
                        raw_rope.remove(start_idx..end_idx);
                    }
                    OperationType::Replace => {
                        raw_rope.remove(start_idx..end_idx);
                        raw_rope.insert(start_idx, &modified_content);
                    }
                }
                self.dirty = true;
                Ok(())
            }
            Server => {
                let cursor_list = self.concurrent_share.cursor_list.as_ref().unwrap().clone();
                let mut histories_mutex = HISTORY.lock().unwrap();
                histories_mutex.push(modify.clone());
                self.merge_history(&vec![modify.clone()], &mut cursor_list.lock().unwrap())?;
                self.dirty = true;
                Ok(())
            }
            Client => {
                let concur_lock = self.concurrent_share.mutex.lock().unwrap();

                let raw_rope = self.data.as_mut();
                let range = &modify.op_range;
                let start_idx =
                    raw_rope.line_to_char(range.start.row as usize) + range.start.col as usize;
                let end_idx =
                    raw_rope.line_to_char(range.end.row as usize) + range.end.col as usize;

                match modify.op {
                    OperationType::Insert => {
                        raw_rope.insert(start_idx, &modified_content);
                    }
                    OperationType::Delete => {
                        raw_rope.remove(start_idx..end_idx);
                    }
                    OperationType::Replace => {
                        raw_rope.remove(start_idx..end_idx);
                        raw_rope.insert(start_idx, &modified_content);
                    }
                }
                self.version += 1;
                self.dirty = true;
                Ok(())
            }
        }
    }
}

impl Text {
    pub fn from_path(file_path: &Path) -> Result<Self, Box<dyn Error>> {
        match file_io::read_file(file_path) {
            Ok(content) => match file_io::get_last_modified(file_path) {
                Ok(last_modified) => Ok(Text {
                    share_status: FileShareStatus::default(),
                    data: Box::new(Rope::from_str(&all_to_lf(&content))),
                    path: PathBuf::from(file_path),
                    version: 0,
                    dirty: false,
                    last_modified,
                    concurrent_share: ConcurrencyShare {
                        mutex: Mutex::new(()),
                        update_thread: None,
                        cursor_list: None,
                    },
                }),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }

    pub fn from_path_str(file_path: &str) -> Result<Self, Box<dyn Error>> {
        Text::from_path(Path::new(file_path))
    }

    pub fn from_str(file_path: &Path, text: &str) -> Self {
        Text {
            share_status: FileShareStatus::default(),
            data: Box::new(Rope::from_str(text)),
            path: file_path.to_path_buf(),
            version: 0,
            dirty: false,
            last_modified: SystemTime::now(),
            concurrent_share: ConcurrencyShare {
                mutex: Mutex::new(()),
                update_thread: None,
                cursor_list: None,
            },
        }
    }
}

impl HistorianFile<Rope, Modification, CursorList> for Text {
    fn get_version(&self) -> usize {
        self.version
    }

    fn get_share_status(&self) -> FileShareStatus {
        self.share_status.clone()
    }

    fn merge_history(&mut self, modifies: &[Modification], cursors: &mut CursorList) -> ResultVoid {
        let concur_lock = self.concurrent_share.mutex.lock().unwrap();

        for modify in modifies {
            let increase_lines = lines_count(&modify.modified_content);
            let raw_rope = self.data.as_mut();
            let range = &modify.op_range;
            let start_idx =
                raw_rope.line_to_char(range.start.row as usize) + range.start.col as usize;
            let end_idx = raw_rope.line_to_char(range.end.row as usize) + range.end.col as usize;
            // let mut changed_lines;

            match modify.op {
                OperationType::Insert => {
                    raw_rope.insert(start_idx, &modify.modified_content);
                    // changed_lines = increase_lines;
                }
                OperationType::Delete => {
                    raw_rope.remove(start_idx..end_idx);
                    // changed_lines = (range.end.row - range.start.row) as
                    // usize;
                }
                OperationType::Replace => {
                    raw_rope.remove(start_idx..end_idx);
                    raw_rope.insert(start_idx, &modify.modified_content);
                    // changed_lines = increase_lines - (range.end.row -
                    // range.start.row) as usize;
                }
            }

            // let mut cursors_to_update = get_cursor::<CursorRowEq>(
            //     cursors,
            //     &ClientCursor {
            //         addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0,
            // 1)), 0),         row: range.start.row,
            //         col: range.start.col,
            //     },
            // )
            // .unwrap();
            //
            // if changed_lines == 0 {
            //     cursors_to_update.current().unwrap().col +=
            //         modify.op_range.end.col - modify.op_range.start.col;
            // } else {
            //     cursors_to_update.current().unwrap().col += {
            //         let idx = modify.modified_content.rfind("\n").unwrap();
            //         (modify.modified_content.len() - idx - 1) as u64
            //     };
            //     loop {
            //         cursors_to_update.move_next();
            //         match cursors_to_update.current() {
            //             Some(cursor) => {
            //                 cursor.row += changed_lines as u64;
            //             }
            //             None => break,
            //         }
            //     }
            // }
        }
        self.dirty = true;
        self.version += modifies.len();
        Ok(())
    }

    fn change_share_status(&mut self, status: FileShareStatus) -> bool {
        if self.share_status == Server && status == Private {
            self.share_status = status;
            true
        } else if self.share_status == Private && status == Server {
            self.concurrent_share = ConcurrencyShare {
                mutex: Mutex::new(()),
                update_thread: None,
                cursor_list: Some(CURSOR_LIST.clone()),
            };
            self.share_status = status;
            true
        } else if self.share_status == Private && status == Client {
            self.concurrent_share = ConcurrencyShare {
                mutex: Mutex::new(()),
                update_thread: None,
                cursor_list: None,
            };
            self.share_status = status;
            true
        } else {
            false
        }
    }
}

impl MFile<Rope, Modification, CursorList> for Text {}

#[cfg(test)]
mod rope_test {
    use super::*;
    use crate::{
        interface::storage::BasicFile,
        remote::{server::editor_rpc::OperationType, Modification, OpRange},
        types::rpc_types::CursorPosition,
    };

    fn get_full_path(name: &str) -> String {
        match std::env::var("TEMP") {
            Ok(val) => format!("{}/{}", val, name),
            Err(_e) => format!("/tmp/{}", name),
        }
    }

    #[test]
    fn test_get_path() {
        let file_name = get_full_path("moras_test.txt");
        std::fs::write(&file_name, "Hello, world!\nThis is a test file.\n").unwrap();
        let file_path = PathBuf::from(&file_name);
        let text = Text::from_path(&file_path).unwrap();

        assert_eq!(text.get_path(), &file_path);
    }

    #[test]
    fn test_get_path_str() {
        let file_name = get_full_path("moras_test.txt");
        std::fs::write(&file_name, "Hello, world!\nThis is a test file.\n").unwrap();
        let file_path = PathBuf::from(&file_name);
        let text = Text::from_path(&file_path).unwrap();

        assert_eq!(text.get_path_str(), file_name);
    }

    #[test]
    fn test_is_dirty() {
        let file_name = get_full_path("moras_test.txt");
        std::fs::write(&file_name, "Hello, world!\nThis is a test file.\n").unwrap();
        let file_path = PathBuf::from(&file_name);
        let mut text = Text::from_path(&file_path).unwrap();

        assert_eq!(text.is_dirty(), false);

        text.set_dirty(true);
        assert_eq!(text.is_dirty(), true);
    }

    #[test]
    fn test_to_string() {
        let file_name = get_full_path("moras_test2.txt");
        std::fs::write(&file_name, "Hello, world!\nThis is a test file.\n").unwrap();
        let file_path = PathBuf::from(&file_name);
        let text = Text::from_path(&file_path).unwrap();

        assert_eq!(text.to_string(), "Hello, world!\nThis is a test file.\n");
    }

    #[test]
    fn test_save() {
        let file_name = get_full_path("moras_test.txt");
        std::fs::write(&file_name, "Hello, world!\nThis is a test file.\n").unwrap();
        let file_path = PathBuf::from(&file_name);
        let mut text = Text::from_path(&file_path).unwrap();

        text.set_dirty(true);
        let _ = text.save();

        assert_eq!(text.is_dirty(), false);
    }

    #[test]
    fn test_get_raw() {
        let file_name = get_full_path("moras_test.txt");
        std::fs::write(&file_name, "Hello, world!\nThis is a test file.\n").unwrap();
        let file_path = PathBuf::from(&file_name);
        let mut text = Text::from_path(&file_path).unwrap();

        let path = text.get_path_str();

        assert_eq!(path.len(), file_name.len());
    }

    #[test]
    fn test_handle_modify() {
        let file_name = get_full_path("moras_test1.txt");
        std::fs::write(&file_name, "Hello, world!\nThis is a test file.\n").unwrap();
        let file_path = PathBuf::from(&file_name);
        let mut text = Text::from_path(&file_path).unwrap();

        let modify = Modification {
            op: OperationType::Insert,
            version: 0,
            op_range: OpRange {
                start: CursorPosition { row: 0, col: 0 },
                end: CursorPosition { row: 0, col: 0 },
            },
            modified_content: "Test".to_string(),
        };

        text.handle_modify(&modify).unwrap();

        assert_eq!(
            text.to_string(),
            "TestHello, world!\nThis is a test file.\n"
        );
    }
}
