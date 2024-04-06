pub trait MFile<D, STA, ERR>: Send + Sync {
    fn from_file(&mut self, path: &str) -> Result<bool, ERR>;

    fn from_str(&mut self, text: &str) -> Result<bool, ERR>;

    fn save_file(&self) -> Result<bool, ERR>;

    fn save_as(&self, path: String) -> Result<bool, ERR>;
}
