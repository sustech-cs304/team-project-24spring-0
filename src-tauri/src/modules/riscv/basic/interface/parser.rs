use crate::interface::parser::{ParserInst, ParserInstSet};
use crate::modules::riscv::rv32i::constants::{RISCVImmediate, RISCVInstruction, RISCVRegister};
use crate::utility::ptr::Ptr;

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
