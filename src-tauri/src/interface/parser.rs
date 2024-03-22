use ropey::Rope;

use super::BasicOp;

pub trait Parser<Output>: BasicOp<Output, String> {
    fn parse(&self, code: &Rope) -> Box<Output>;
}
