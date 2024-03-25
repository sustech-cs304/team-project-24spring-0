use ropey::Rope;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::rc::Rc;

use crate::interface::storage::MFile;

struct Text<D> {
    data: Rc<D>,
    path: String,
    last_modified: std::time::SystemTime,
}

// impl MFile<Rope, String, String> for Text<Rope> {
//     //TODO
//     fn open_directory(&self, path: &str) -> Result<Vec<String>,String> {
//         //std::fs::read_dir(path)?;
//         true
//     }
//
//     fn create_directory(&self, path: &str) -> bool {
//         //std::fs::create_dir_all(path)?;
//         true
//     }
//
//     fn open_file(&mut self, path: &str) -> bool {
//         //self.data = Rope::from_reader(BufReader::new(File::open(path)?))?;
//         self.path = path.to_string();
//         true
//     }
//
//     fn from_str(&mut self, text: &str) {
//         //self.data = Rope::from_str(text);
//     }
//
//     fn save_file(&self) -> bool {
//         //let file = File::create(&self.path)?;
//         //let mut writer = BufWriter::new(file);
//         //self.data.write_to(&mut writer)?;
//         true
//     }
//
//     fn set_path(&mut self, path: &str) {
//         self.path = path.to_string();
//     }
//
//     fn save_as(&self, path: &str) -> bool {
//         //let file = File::create(path)?;
//         //let mut writer = BufWriter::new(file);
//         //self.data.write_to(&mut writer)?;
//         true
//     }
//
//     fn get_storage(&mut self) -> Option<Rc<Rope>> {
//         Some(Rc::clone(&self.data))
// }

//https://docs.rs/ropey/latest/ropey/index.html
// }
