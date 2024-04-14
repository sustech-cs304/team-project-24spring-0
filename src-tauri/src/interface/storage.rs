use std::boxed::Box;
pub trait MFile<ERR>: Send + Sync {
    fn get_string(&self) -> String;

    fn save(&mut self) -> Result<bool, ERR>;

    fn from_path(path: &str) -> Result<Box<Self>, ERR>;

    fn from_str(text: &str) -> Result<Box<Self>, ERR>;
}
