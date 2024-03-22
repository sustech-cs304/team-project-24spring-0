use std::rc::Rc;

use super::BasicOp;

pub trait MFile<D, STA, ERR>: BasicOp<STA, ERR> {
    fn open_directory(&self, path: &str) -> bool;

    fn create_directory(&self, path: &str) -> bool;

    fn open_file(&mut self, path: &str) -> bool;

    //fn new_storage(path: &str) -> Option<Rc<D>>;

    fn from_str(&mut self, text: &str);

    fn save_file(&self) -> bool;

    fn set_path(&mut self, path: &str);

    fn save_as(&self, path: &str) -> bool;

    fn get_storage(&mut self) -> Option<Rc<D>>;
}
