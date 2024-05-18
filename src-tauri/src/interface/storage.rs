use std::{default, error::Error, path::PathBuf};

use crate::types::middleware_types::FileOperation;

#[derive(Default)]
pub enum FileShareStatus {
    #[default]
    Private,
    Host,
    Client,
}

pub trait MFile<CON, ERR>: Send + Sync {
    fn get_path(&self) -> PathBuf;

    fn get_path_str(&self) -> String;

    fn is_dirty(&self) -> bool;

    fn set_dirty(&mut self, dirty: bool);

    fn to_string(&self) -> String;

    fn save(&mut self) -> Option<ERR>;

    fn update_content(&mut self, content: &str);

    fn get_raw(&mut self) -> &mut CON;

    fn handle_modify(&mut self, op: FileOperation) -> Result<(), Box<dyn Error>>;

    fn switch_share_status(&mut self, status: FileShareStatus);
}
