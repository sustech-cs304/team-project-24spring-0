use std::{error::Error, path::PathBuf};

#[derive(Default)]
pub enum FileShareStatus {
    #[default]
    Private,
    Host,
    Client,
}
pub trait MFile<D, H, C>: BasicFile<D, H> + MeragableFile<D, H, C> {}

pub trait BasicFile<D, H>: Send + Sync {
    fn get_path(&self) -> &PathBuf;

    fn get_path_str(&self) -> String;

    fn is_dirty(&self) -> bool;

    fn set_dirty(&mut self, dirty: bool);

    fn to_string(&self) -> String;

    fn save(&mut self) -> Option<Box<dyn Error + Send + Sync>>;

    fn get_raw(&mut self) -> &mut D;

    fn handle_modify(&mut self, history: &H) -> Result<(), Box<dyn Error + Send + Sync>>;

    fn switch_share_status(&mut self, status: FileShareStatus);
}

pub trait MeragableFile<D, H, C>: Send + Sync {
    fn get_version(&self) -> usize;
    fn merge_history(
        &mut self,
        histories: &[H],
        cursors: &mut C,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
}
