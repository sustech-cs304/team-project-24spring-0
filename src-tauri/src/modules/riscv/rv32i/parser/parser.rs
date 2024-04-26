use std::str::FromStr;

use once_cell::sync::Lazy;
use strum::IntoEnumIterator;

use super::super::super::basic::interface::parser::{
    ParserRISCVCsr, ParserRISCVInstOp, ParserRISCVRegister,
};
use super::super::super::basic::parser::lexer::Symbol;
use super::super::super::basic::parser::parser::RISCVSymbolList;
use super::super::super::rv32i::constants::{
    RV32ICsr, RV32IInstruction, RV32IRegister, RV32I_REGISTER_VALID_NAME,
};
use super::lexer::RV32IOpToken;

pub static RV32I_SYMBOL_LIST: Lazy<RISCVSymbolList> = Lazy::new(|| vec![&OP_TOKEN, &REG_TOKEN]);

pub static OP_TOKEN: Lazy<Vec<(&'static str, Symbol<'static>)>> = Lazy::new(|| {
    OP_TOKEN_STASH
        .iter()
        .map(|op| (op.0.as_str(), op.1))
        .collect()
});

pub static REG_TOKEN: Lazy<Vec<(&'static str, Symbol<'static>)>> = Lazy::new(|| {
    RV32I_REGISTER_VALID_NAME
        .iter()
        .map(|reg| {
            (
                *reg,
                Symbol::Reg(RV32IRegister::from_str(reg).unwrap().into()),
            )
        })
        .collect()
});

pub static CSR_TOKEN: Lazy<Vec<(&'static str, Symbol<'static>)>> = Lazy::new(|| {
    RV32ICsr::iter()
        .map(|csr| (csr.into(), Symbol::Csr(csr.into())))
        .collect()
});

static OP_TOKEN_STASH: Lazy<Vec<(String, Symbol<'static>)>> = Lazy::new(|| {
    RV32IOpToken::iter()
        .map(|op| (op.name(), Symbol::Op((op).into())))
        .collect()
});

impl From<RV32IRegister> for ParserRISCVRegister {
    fn from(reg: RV32IRegister) -> Self {
        ParserRISCVRegister::RV32I(reg)
    }
}

impl From<RV32IInstruction> for ParserRISCVInstOp {
    fn from(inst: RV32IInstruction) -> Self {
        ParserRISCVInstOp::RV32I(inst)
    }
}

impl From<RV32ICsr> for ParserRISCVCsr {
    fn from(csr: RV32ICsr) -> Self {
        ParserRISCVCsr::RV32I(csr)
    }
}
