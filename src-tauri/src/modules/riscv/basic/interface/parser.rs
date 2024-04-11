use crate::utility::ptr::Ptr;

pub use super::super::super::rv32i::constants::*;
pub use super::super::parser::parser::RISCVParser;
pub use crate::interface::parser::*;

#[derive(Clone, Copy, Debug)]
pub struct RISCV;

pub type ParserRISCVInstOp = RISCVInstruction;

#[derive(Clone, Copy, Debug)]
pub enum ParserRISCVInstOpd {
    Reg(RISCVRegister),
    Imm(RISCVImmediate),
    Lbl(Ptr<ParserInst<RISCV>>),
}

impl ParserInstSet for RISCV {
    type Operator = ParserRISCVInstOp;
    type Operand = ParserRISCVInstOpd;
}
