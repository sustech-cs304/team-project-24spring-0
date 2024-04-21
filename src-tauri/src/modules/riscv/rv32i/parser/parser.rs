use super::super::super::basic::interface::parser::{
    ParserRISCVInstOpTrait, ParserRISCVRegisterTrait,
};
use super::super::super::basic::parser::lexer::Symbol;
use super::super::super::basic::parser::parser::RISCVSymbolList;
use super::super::super::rv32i::constants::{
    RISCVCsr, RV32IInstruction, RV32IRegister, RV32I_REGISTER_VALID_NAME,
};
use super::lexer::RV32IOpToken;
use lazy_static::lazy_static;
use std::str::FromStr;
use strum::IntoEnumIterator;

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
        RISCVCsr::iter()
            .map(|csr| (csr.into(), Symbol::Csr(csr.into())))
            .collect()
    };
}

impl ParserRISCVRegisterTrait for RV32IRegister {}

impl ParserRISCVInstOpTrait for RV32IInstruction {}
