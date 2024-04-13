use ropey::Rope;

use crate::interface::storage::MFile;
use crate::io::file_io;
use std::path::PathBuf;

pub struct Text {
    data: Box<Rope>,
    path: std::path::PathBuf,
    dirty: bool,
    last_modified: std::time::SystemTime,
}

impl MFile<String> for Text {
    fn from_path(&mut self, filepath: &str) -> Result<bool, String> {
        match file_io::read_file_str(filepath) {
            Ok(content) => {
                self.data = Box::new(Rope::from_str(&content));
                self.path = PathBuf::from(filepath);
                self.dirty = false;
                Ok(true)
            }
            Err(e) => Err(e),
        }
    }

    fn from_str(&mut self, text: &str) -> Result<bool, String> {
        self.data = Box::new(Rope::from_str(text));
        self.dirty = false;
        Ok(true)
    }

    fn save_file(&mut self) -> Result<bool, String> {
        match file_io::write_file(self.path.as_path(), &self.data.as_ref().to_string()) {
            Ok(_) => {
                self.dirty = false;
                Ok(true)
            }
            Err(e) => Err(e),
        }
    }
    //https://docs.rs/ropey/latest/ropey/index.html
}
