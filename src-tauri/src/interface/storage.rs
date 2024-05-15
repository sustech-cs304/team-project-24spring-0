use std::path::PathBuf;

pub trait MFile<CON, ERR>: Send + Sync {
    fn get_path(&self) -> PathBuf;

    fn get_path_str(&self) -> String;

    fn is_dirty(&self) -> bool;

    fn set_dirty(&mut self, dirty: bool);

    fn to_string(&self) -> String;

    fn save(&mut self) -> Option<ERR>;

    fn update_content(&mut self, content: &str);

    fn get_raw(&mut self) -> &mut CON;
}
