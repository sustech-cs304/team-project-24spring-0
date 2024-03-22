pub mod assembler;
pub mod file_io;
pub mod frontend;
pub mod parser;
pub mod simulator;

pub trait BasicOp<S, E> {
    fn get_status(&self) -> S;
    fn get_error(&self) -> E;
    fn interrupt(&mut self);
}
