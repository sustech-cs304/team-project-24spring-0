pub use super::super::super::rv32f::constants::*;
pub use super::super::super::rv32i::constants::*;
pub use super::super::parser::parser::RISCVParser;
use super::super::parser::parser::RISCVSymbolList;
pub use crate::interface::parser::*;

pub const MAX_DATA_SIZE: usize = 0xf_ffff;
pub const DATA_CHUNK_RECOMMEND_SIZE: usize = 0x7ff;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RISCV;

pub enum RISCVExtension {
    RV32I,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParserRISCVInstOp {
    RV32I(RV32IInstruction),
    RV32F(RV32FInstruction),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParserRISCVRegister {
    RV32I(RV32IRegister),
    RV32F(RV32FRegister),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParserRISCVImmediate {
    Imm(RISCVImmediate),
    Lbl((ParserRISCVLabel, ParserRISCVLabelHandler)),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParserRISCVCsr {
    RV32I(RV32ICsr),
    RV32F(RV32FCsr),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParserRISCVInstOpd {
    Reg(ParserRISCVRegister),
    Imm(ParserRISCVImmediate),
    Lbl(ParserRISCVLabel),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParserRISCVLabel {
    Text(usize),          // ParserResult<RISCV>::text[usize]
    Data((usize, usize)), // ParserResult<RISCV>::data[usize][usize]
    Unknown(Pos),         /* the label position in the code (mustn't exist in
                           * the output) */
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParserRISCVLabelHandler {
    Low,
    High,
    DeltaHigh,
    DeltaMinusOneLow,
}

impl ParserInstSet for RISCV {
    type Operator = ParserRISCVInstOp;
    type Operand = ParserRISCVInstOpd;
}

// ------------------------- Implementations -------------------------

impl RISCVExtension {
    pub fn get_symbol_parser(&self) -> &RISCVSymbolList {
        match self {
            RISCVExtension::RV32I => &super::super::super::rv32i::parser::parser::RV32I_SYMBOL_LIST,
        }
    }
}
