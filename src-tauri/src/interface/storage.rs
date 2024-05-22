use std::{error::Error, path::PathBuf};

use crate::types::middleware_types::FileOperation;

#[derive(Default)]
pub enum FileShareStatus {
    #[default]
    Private,
    Host,
    Client,
}
pub trait MFile<D, H>: BasicFile<D> + MeragableFile<D, H> {}

pub trait BasicFile<D>: Send + Sync {
    fn get_path(&self) -> &PathBuf;

    fn get_path_str(&self) -> String;

    fn is_dirty(&self) -> bool;

    fn set_dirty(&mut self, dirty: bool);

    fn to_string(&self) -> String;

    fn save(&mut self) -> Option<Box<dyn Error>>;

    fn update_content(&mut self, content: &str);

    fn get_raw(&mut self) -> &mut D;

    fn handle_modify(&mut self, op: FileOperation) -> Result<(), Box<dyn Error>>;

    fn switch_share_status(&mut self, status: FileShareStatus);
}

pub trait MeragableFile<D, H>: Send + Sync {
    fn merge_history(&mut self, histories: &Vec<H>) -> Result<(), Box<dyn Error>>;
}
