use std::{
    error::Error,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::{Path, PathBuf},
    time::SystemTime,
};

use ropey::Rope;

use crate::{
    interface::storage::{
        BasicFile,
        FileShareStatus,
        FileShareStatus::{Client, Host, Private},
        MFile,
        MeragableFile,
    },
    io::file_io,
    middleware_types::Cursor,
    remote::{
        server::editor_rpc::OperationType,
        utils::priority_lsit::get_cursor,
        ClientCursor,
        CursorRowEq,
        Modification,
    },
    types::ResultVoid,
    utility::text_helper::lines_count,
};

pub struct Text {
    share_status: FileShareStatus,
    data: Box<Rope>,
    path: PathBuf,
    version: usize,
    dirty: bool,
    last_modified: SystemTime,
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
                        raw_rope.insert(start_idx, &modify.modified_content);
                    }
                    OperationType::Delete => {
                        raw_rope.remove(start_idx..end_idx);
                    }
                    OperationType::Replace => {
                        raw_rope.remove(start_idx..end_idx);
                        raw_rope.insert(start_idx, &modify.modified_content);
                    }
                }
                self.dirty = true;
                Ok(())
            }
            Host => {
                todo!("perform function change");
            }
            Client => {
                todo!("perform function change");
            }
        }
    }
}

impl Text {
    pub fn from_path(file_path: &Path) -> Result<Self, Box<dyn Error>> {
        match file_io::read_file(file_path) {
            Ok(content) => match file_io::get_last_modified(file_path) {
                Ok(last_modified) => Ok(Text {
                    share_status: Default::default(),
                    data: Box::new(Rope::from_str(&content)),
                    path: PathBuf::from(file_path),
                    version: 0,
                    dirty: false,
                    last_modified,
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
            share_status: Default::default(),
            data: Box::new(Rope::from_str(text)),
            path: file_path.to_path_buf(),
            version: 0,
            dirty: false,
            last_modified: SystemTime::now(),
        }
    }
}

impl MeragableFile<Rope, Modification, Cursor> for Text {
    fn get_version(&self) -> usize {
        self.version
    }

    fn get_share_status(&self) -> FileShareStatus {
        self.share_status.clone()
    }

    fn merge_history(&mut self, modifies: &[Modification], cursors: &mut Cursor) -> ResultVoid {
        for modify in modifies {
            let increase_lines = lines_count(&modify.modified_content);
            let raw_rope = self.data.as_mut();
            let range = &modify.op_range;
            let start_idx =
                raw_rope.line_to_char(range.start.row as usize) + range.start.col as usize;
            let end_idx = raw_rope.line_to_char(range.end.row as usize) + range.end.col as usize;
            let mut changed_lines;
            match modify.op {
                OperationType::Insert => {
                    raw_rope.insert(start_idx, &modify.modified_content);
                    changed_lines = increase_lines;
                }
                OperationType::Delete => {
                    raw_rope.remove(start_idx..end_idx);
                    changed_lines = (range.end.row - range.start.row) as usize;
                }
                OperationType::Replace => {
                    raw_rope.remove(start_idx..end_idx);
                    raw_rope.insert(start_idx, &modify.modified_content);
                    changed_lines = increase_lines - (range.end.row - range.start.row) as usize;
                }
            }
            let mut cursors_to_update = get_cursor::<CursorRowEq>(
                cursors,
                &ClientCursor {
                    addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0),
                    row: range.start.row,
                    col: range.start.col,
                },
            )
            .unwrap();
            if changed_lines == 0 {
                cursors_to_update.current().unwrap().col +=
                    modify.op_range.end.col - modify.op_range.start.col;
            } else {
                cursors_to_update.current().unwrap().col += {
                    let idx = modify.modified_content.rfind("\n").unwrap();
                    (modify.modified_content.len() - idx - 1) as u64
                };
                loop {
                    cursors_to_update.move_next();
                    match cursors_to_update.current() {
                        Some(cursor) => {
                            cursor.row += changed_lines as u64;
                        }
                        None => break,
                    }
                }
            }
        }
        self.dirty = true;
        Ok(())
    }

    fn change_share_status(&mut self, status: FileShareStatus) -> bool {
        if self.share_status == Host && status == Private
            || self.share_status == Private && status == Host
        {
            self.share_status = status;
            true
        } else {
            false
        }
    }
}

impl MFile<Rope, Modification, Cursor> for Text {}
