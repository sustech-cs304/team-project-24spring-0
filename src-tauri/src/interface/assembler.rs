use super::BasicOp;
use std::rc::Rc;

pub trait Assembler<In, Out, Set>: BasicOp<String, String> {
    fn assemble(&mut self, ast: &In) -> Option<Rc<Out>>;
    fn dump(&self, ast: &In) -> Option<String>;
    fn update_setting(&mut self, settings: &Set) -> bool;
}
