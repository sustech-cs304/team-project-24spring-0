pub trait MFile<ERR>: Send + Sync {
    fn from_path(&mut self, path: &str) -> Result<bool, ERR>;

    fn from_str(&mut self, text: &str) -> Result<bool, ERR>;

    fn save_file(&mut self) -> Result<bool, ERR>;
}
