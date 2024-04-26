use std::str::FromStr;

use lazy_static::lazy_static;
use strum::IntoEnumIterator;

use super::{
    super::super::{
        basic::{
            interface::parser::{ParserRISCVCsr, ParserRISCVInstOp, ParserRISCVRegister},
            parser::{lexer::Symbol, parser::RISCVSymbolList},
        },
        rv32i::constants::{RV32ICsr, RV32IInstruction, RV32IRegister, RV32I_REGISTER_VALID_NAME},
    },
    lexer::RV32IOpToken,
};

lazy_static! {
    pub static ref RV32I_SYMBOL_LIST: RISCVSymbolList = vec![&OP_TOKEN, &REG_TOKEN];
}

lazy_static! {
    pub static ref OP_TOKEN_STASH: Vec<(String, Symbol<'static>)> = {
        RV32IOpToken::iter()
            .map(|op| (op.name(), Symbol::Op((op).into())))
            .collect()
    };
    pub static ref OP_TOKEN: Vec<(&'static str, Symbol<'static>)> = {
        OP_TOKEN_STASH
            .iter()
            .map(|op| (op.0.as_str(), op.1))
            .collect()
    };
    pub static ref REG_TOKEN: Vec<(&'static str, Symbol<'static>)> = {
        RV32I_REGISTER_VALID_NAME
            .iter()
            .map(|reg| {
                (
                    *reg,
                    Symbol::Reg(RV32IRegister::from_str(reg).unwrap().into()),
                )
            })
            .collect()
    };
    pub static ref CSR_TOKEN: Vec<(&'static str, Symbol<'static>)> = {
        RV32ICsr::iter()
            .map(|csr| (csr.into(), Symbol::Csr(csr.into())))
            .collect()
    };
}

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
