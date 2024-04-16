pub use super::super::super::rv32i::constants::*;
pub use super::super::parser::parser::RISCVParser;
pub use crate::interface::parser::*;

pub const MAX_DATA_SIZE: usize = 0xf_ffff;
pub const DATA_CHUNK_RECOMMEND_SIZE: usize = 0x7ff;

#[derive(Clone, Copy, Debug)]
pub struct RISCV;

pub type ParserRISCVInstOp = RISCVInstruction;

#[derive(Clone, Copy, Debug)]
pub enum ParserRISCVInstOpd {
    Reg(RISCVRegister),
    Imm(RISCVImmediate),
    Lbl(ParserRISCVLabel),
}

#[derive(Clone, Copy, Debug)]
pub enum ParserRISCVLabel {
    Text(usize),          // ParserResult<RISCV>::text[usize]
    Data((usize, usize)), // ParserResult<RISCV>::data[usize][usize]
    Unknown(Pos),         // the label position in the code (mustn't exist in the output)
}

impl ParserInstSet for RISCV {
    type Operator = ParserRISCVInstOp;
    type Operand = ParserRISCVInstOpd;
}
