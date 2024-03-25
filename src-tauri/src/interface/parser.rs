use ropey::Rope;

use std::rc::Rc;

pub trait Parser<CODE, COMP, OUT, ERR> {
    fn parse(&mut self, code: &CODE) -> Result<OUT, ERR>;
    fn completion(&mut self, code: CODE, pos: (u64, u64)) -> Result<COMP, ERR>;
}
