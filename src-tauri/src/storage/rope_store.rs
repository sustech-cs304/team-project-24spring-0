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
    fn get_string(&self) -> String {
        self.data.as_ref().to_string()
    }

    fn save(&mut self) -> Result<bool, String> {
        match file_io::write_file(self.path.as_path(), &self.data.as_ref().to_string()) {
            Ok(_) => {
                self.dirty = false;
                Ok(true)
            }
            Err(e) => Err(e),
        }
    }

    fn from_path(filepath: &str) -> Result<Box<Self>, String> {
        match file_io::read_file_str(filepath) {
            Ok(content) =>Ok ( Text {
                data: Box::new(Rope::from_str(&content)),
                path: PathBuf::from(filepath),
                dirty: false,
                last_modified: file_io::get_last_modified(filepath),
            } )
            Err(e) => Err(e),
        }
    }

    fn from_str(text: &str) -> Result<Box<Self>, String> {
        Ok(Text {
            data: Box::new(Rope::from_str(text)),
            path: PathBuf::new(),
            dirty: false,
            last_modified: std::time::SystemTime::now(),
        })
    }

    //https://docs.rs/ropey/latest/ropey/index.html
}
