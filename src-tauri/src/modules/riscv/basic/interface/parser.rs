use crate::utility::any::AnyU8;

pub use super::super::parser::parser::RISCVParser;
pub use crate::interface::parser::*;

pub const MAX_DATA_SIZE: usize = 0xf_ffff;
pub const DATA_CHUNK_RECOMMEND_SIZE: usize = 0x7ff;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RISCV;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ParserRISCVInstOp(AnyU8, &'static str);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ParserRISCVRegister(AnyU8, &'static str);

pub type ParserRISCVImmediate = super::super::super::rv32i::constants::RISCVImmediate;

pub type ParserRISCVCsr = super::super::super::rv32i::constants::RISCVCsr;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ParserRISCVInstOpd {
    Reg(ParserRISCVRegister),
    Imm(ParserRISCVImmediate),
    Lbl(ParserRISCVLabel),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParserRISCVLabel {
    Text(usize),          // ParserResult<RISCV>::text[usize]
    Data((usize, usize)), // ParserResult<RISCV>::data[usize][usize]
    Unknown(Pos),         // the label position in the code (mustn't exist in the output)
}

pub trait ParserRISCVInstOpTrait: Clone + Copy + std::fmt::Debug + PartialEq + Eq {
    fn get_name(&self) -> &'static str;
}
pub trait ParserRISCVRegisterTrait: Clone + Copy + std::fmt::Debug + PartialEq + Eq {
    fn get_name(&self) -> &'static str;
}

impl ParserInstSet for RISCV {
    type Operator = ParserRISCVInstOp;
    type Operand = ParserRISCVInstOpd;
}

// ------------------------- Implementations -------------------------

impl<T: ParserRISCVInstOpTrait + std::fmt::Debug + 'static> From<T> for ParserRISCVInstOp {
    fn from(op: T) -> Self {
        ParserRISCVInstOp(AnyU8::from(op), op.get_name())
    }
}

impl ParserRISCVInstOp {
    pub fn is<T: ParserRISCVInstOpTrait>(&self) -> bool
    where
        T: 'static,
    {
        self.0.is::<T>()
    }

    pub fn to<T: ParserRISCVInstOpTrait>(&self) -> Option<T>
    where
        T: 'static,
    {
        self.0.to::<T>()
    }
}

impl<T: ParserRISCVRegisterTrait + 'static> From<T> for ParserRISCVRegister {
    fn from(reg: T) -> Self {
        ParserRISCVRegister(AnyU8::from(reg), reg.get_name())
    }
}

impl ParserRISCVRegister {
    pub fn is<T: ParserRISCVRegisterTrait>(&self) -> bool
    where
        T: 'static,
    {
        self.0.is::<T>()
    }

    pub fn to<T: ParserRISCVRegisterTrait>(&self) -> Option<T>
    where
        T: 'static,
    {
        self.0.to::<T>()
    }
}

impl std::fmt::Debug for ParserRISCVInstOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.1)
    }
}

impl std::fmt::Debug for ParserRISCVRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.1)
    }
}
