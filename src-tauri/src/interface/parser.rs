use crate::modules::riscv::rv32i::constants::{RISCVImmediate, RISCVInstruction, RISCVRegister};
use crate::utility::ptr::Ptr;

pub trait Parser: Send + Sync {
    fn parse(&mut self, code: &ropey::Rope) -> Result<ParserResult, ParserError>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pos(pub usize, pub usize);

#[derive(Clone, Debug)]
pub struct ParserResult {
    pub data: Vec<ParserResultData>,
    pub text: Vec<ParserResultText>,
}

#[derive(Clone, Debug)]
pub struct ParserError {
    pub pos: Pos,
    pub msg: String,
}

#[derive(Clone, Debug)]
pub enum ParserResultData {
    Data(Vec<u8>),
    Align(u8),
}

#[derive(Clone, Debug)]
pub enum ParserResultText {
    Text(ParserRISCVInst),
    Align(u8),
}

#[derive(Clone, Debug)]
pub struct ParserRISCVInst {
    pub line: usize,
    pub op: ParserRISCVInstOp,
    pub opd: Vec<ParserRISCVInstOpd>,
}

pub type ParserRISCVInstOp = RISCVInstruction;

#[derive(Clone, Copy, Debug)]
pub enum ParserRISCVInstOpd {
    Reg(RISCVRegister),
    Imm(RISCVImmediate),
    Lbl(Ptr<ParserRISCVInst>),
}
