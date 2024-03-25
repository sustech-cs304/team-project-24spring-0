pub trait MFile<D, STA, ERR> {
    fn open_directory(&self, path: &str) -> Result<Vec<String>, ERR>;

    fn create_directory(&self, path: &str) -> Result<bool, ERR>;

    fn open_file(&mut self, path: &str) -> Result<bool, ERR>;

    //fn new_storage(path: &str) -> Option<Rc<D>>;

    fn from_str(&mut self, text: &str);

    fn save_file(&self) -> Result<bool, ERR>;

    fn save_as(&self, path: String) -> Result<bool, ERR>;
}
