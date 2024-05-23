use std::{
    error::Error,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::{Path, PathBuf},
    time::SystemTime,
};

use ropey::Rope;

use crate::{
    interface::storage::{BasicFile, FileShareStatus, MFile, MeragableFile},
    io::file_io,
    middleware_types::Cursor,
    remote::{
        server::editor_rpc::OperationType,
        utils::priority_lsit::get_cursor,
        ClientCursor,
        CursorRowEq,
        History,
    },
    utility::text_helper::lines_count,
};

pub struct Text {
    share_status: FileShareStatus,
    data: Box<Rope>,
    path: PathBuf,
    dirty: bool,
    last_modified: SystemTime,
}

impl BasicFile<Rope> for Text {
    fn get_path(&self) -> &PathBuf {
        &self.path
    }
    fn get_path_str(&self) -> String {
        self.path.to_str().unwrap().to_string()
    }

    fn update_content(&mut self, content: &str) {
        *self.data = Rope::from_str(&content);
        self.dirty = true;
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

    fn save(&mut self) -> Option<Box<dyn Error + Send + Sync>> {
        match file_io::write_file(self.path.as_path(), &self.data.as_ref().to_string()) {
            Some(e) => Some(e),
            None => {
                self.dirty = false;
                None
            }
        }
    }

    fn get_raw(&mut self) -> &mut Rope {
        self.data.as_mut()
    }

    fn handle_modify(
        &mut self,
        op: crate::types::middleware_types::FileOperation,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match self.share_status {
            FileShareStatus::Host => {
                todo!("perform function change");
                self.dirty = true;
            }
            FileShareStatus::Client => {
                todo!("perform function change");
            }
            FileShareStatus::Private => {
                todo!("perform function change");
                self.dirty = true;
                Ok(())
            }
        }
    }

    fn switch_share_status(&mut self, status: crate::interface::storage::FileShareStatus) {
        todo!("perform function change");
    }
}

impl Text {
    pub fn from_path(file_path: &Path) -> Result<Self, Box<dyn Error + Send + Sync>> {
        match file_io::read_file(file_path) {
            Ok(content) => match file_io::get_last_modified(file_path) {
                Ok(last_modified) => Ok(Text {
                    share_status: Default::default(),
                    data: Box::new(Rope::from_str(&content)),
                    path: PathBuf::from(file_path),
                    dirty: false,
                    last_modified,
                }),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }

    pub fn from_path_str(file_path: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Text::from_path(Path::new(file_path))
    }

    pub fn from_str(file_path: &Path, text: &str) -> Result<Self, String> {
        Ok(Text {
            share_status: Default::default(),
            data: Box::new(Rope::from_str(text)),
            path: file_path.to_path_buf(),
            dirty: false,
            last_modified: std::time::SystemTime::now(),
        })
    }
}

impl MeragableFile<Rope, History, Cursor> for Text {
    fn merge_history(
        &mut self,
        histories: &[History],
        cursors: &mut Cursor,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        for history in histories {
            let increase_lines = lines_count(&history.modified_content);
            let raw_rope = self.data.as_mut();
            let range = &history.op_range;
            let start_idx =
                raw_rope.line_to_char(range.start.row as usize) + range.start.col as usize;
            let end_idx = raw_rope.line_to_char(range.end.row as usize) + range.end.col as usize;
            let mut changed_lines;
            match history.op {
                OperationType::Insert => {
                    raw_rope.insert(start_idx, &history.modified_content);
                    changed_lines = increase_lines;
                }
                OperationType::Delete => {
                    raw_rope.remove(start_idx..end_idx);
                    changed_lines = (range.end.row - range.start.row) as usize;
                }
                OperationType::Replace => {
                    raw_rope.remove(start_idx..end_idx);
                    raw_rope.insert(start_idx, &history.modified_content);
                    changed_lines = increase_lines - (range.end.row - range.start.row) as usize;
                }
            }
            let mut cusrors_to_update = get_cursor::<CursorRowEq>(
                cursors,
                &ClientCursor {
                    addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0),
                    row: range.start.row,
                    col: range.start.col,
                },
            )
            .unwrap();
            if changed_lines == 0 {
                cusrors_to_update.current().unwrap().col +=
                    history.op_range.end.col - history.op_range.start.col;
            } else {
                cusrors_to_update.current().unwrap().col += {
                    let idx = history.modified_content.rfind("\n").unwrap();
                    (history.modified_content.len() - idx - 1) as u64
                };
                loop {
                    cusrors_to_update.move_next();
                    match cusrors_to_update.current() {
                        Some(cursor) => {
                            cursor.row += changed_lines as u64;
                        }
                        None => break,
                    }
                }
            }
        }
        Ok(())
    }
}

impl MFile<Rope, History, Cursor> for Text {}
