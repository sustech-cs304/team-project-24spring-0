use ropey::Rope;

use std::rc::Rc;

use super::BasicOp;

pub trait Parser<CODE, OUT, STA, ERR>: BasicOp<STA, ERR> {
    fn parse(&mut self, code: Rc<CODE>) -> Box<OUT>;
    fn line_to_addr(&self, line: u64) -> u64;
}
