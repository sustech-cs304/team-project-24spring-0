use std::path::{Path, PathBuf};

use ropey::Rope;

use crate::interface::storage::MFile;
use crate::io::file_io;

pub struct Text {
    data: Box<Rope>,
    path: std::path::PathBuf,
    dirty: bool,
    last_modified: std::time::SystemTime,
}

impl MFile<String> for Text {
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

    fn save(&mut self) -> Option<String> {
        match file_io::write_file(self.path.as_path(), &self.data.as_ref().to_string()) {
            Some(e) => Some(e),
            None => {
                self.dirty = false;
                None
            }
        }
    }

    fn get_path(&self) -> &std::path::PathBuf {
        &self.path
    }

    //https://docs.rs/ropey/latest/ropey/index.html
}

impl Text {
    pub fn from_path(file_path: &Path) -> Result<Self, String> {
        match file_io::read_file(file_path) {
            Ok(content) => match file_io::get_last_modified(file_path) {
                Ok(last_modified) => Ok(Text {
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

    pub fn from_path_str(file_path: &str) -> Result<Self, String> {
        Text::from_path(Path::new(file_path))
    }

    pub fn from_str(file_path: &Path, text: &str) -> Result<Self, String> {
        Ok(Text {
            data: Box::new(Rope::from_str(text)),
            path: file_path.to_path_buf(),
            dirty: false,
            last_modified: std::time::SystemTime::now(),
        })
    }
}
