pub trait Assembler<IN, OUT, SET, ERR>: Send + Sync {
    fn assemble(&mut self, ast: &IN) -> Result<OUT, ERR>;
    fn dump(&self, ast: &IN) -> Result<String, ERR>;
    fn update_setting(&mut self, settings: &SET) -> Result<bool, String>;
}
