use super::BasicOp;
use std::rc::Rc;

pub trait Assembler<IN, OUT, SET, SAT, ERR>: BasicOp<SAT, ERR> {
    fn assemble(&mut self, ast: &IN) -> Option<Rc<OUT>>;
    fn dump(&self, ast: &IN) -> Option<String>;
    fn update_setting(&mut self, settings: &SET) -> bool;
}
