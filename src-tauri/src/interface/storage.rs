pub trait MFile<ERR>: Send + Sync {
    fn get_string(&self) -> String;

    fn save(&mut self) -> Option<ERR>;
}
