use super::super::super::basic::interface::parser::{
    ParserRISCVCsr, ParserRISCVInstOp, ParserRISCVRegister,
};
use super::super::super::basic::parser::lexer::Symbol;
use super::super::super::basic::parser::parser::RISCVSymbolList;
use super::super::super::rv32f::constants::{
    RV32FCsr, RV32FInstruction, RV32FRegister, RV32F_REGISTER_VALID_NAME,
};
use super::lexer::RV32FOpToken;
use lazy_static::lazy_static;
use std::str::FromStr;
use strum::IntoEnumIterator;

lazy_static! {
    pub static ref RV32F_SYMBOL_LIST: RISCVSymbolList = vec![&OP_TOKEN, &REG_TOKEN];
}

lazy_static! {
    pub static ref OP_TOKEN_STASH: Vec<(String, Symbol<'static>)> = {
        RV32FOpToken::iter()
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
        RV32F_REGISTER_VALID_NAME
            .iter()
            .map(|reg| {
                (
                    *reg,
                    Symbol::Reg(RV32FRegister::from_str(reg).unwrap().into()),
                )
            })
            .collect()
    };
}

impl From<RV32FRegister> for ParserRISCVRegister {
    fn from(reg: RV32FRegister) -> Self {
        ParserRISCVRegister::RV32F(reg)
    }
}

impl From<RV32FInstruction> for ParserRISCVInstOp {
    fn from(inst: RV32FInstruction) -> Self {
        ParserRISCVInstOp::RV32F(inst)
    }
}

impl From<RV32FCsr> for ParserRISCVCsr {
    fn from(csr: RV32FCsr) -> Self {
        ParserRISCVCsr::RV32F(csr)
    }
}
