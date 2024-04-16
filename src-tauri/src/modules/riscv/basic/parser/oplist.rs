use crate::modules::riscv::basic::interface::parser::ParserRISCVRegisterTrait;

use super::super::interface::parser::{
    ParserRISCVImmediate, ParserRISCVInstOp, ParserRISCVInstOpd,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RISCVExpectImm {
    U4,
    U5,
    U12,
    U20,
    I12,
    I32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RISCVExpectToken {
    Comma,
    LParen,
    RParen,
    Reg,
    Csr,
    Imm(RISCVExpectImm),
    Lbl,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RISCVOpdSetAimOpdIdx {
    pub idx: usize,
    pub handler: fn(ParserRISCVInstOpd) -> ParserRISCVInstOpd,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RISCVOpdSetAimOpd {
    Idx(RISCVOpdSetAimOpdIdx),
    Val(ParserRISCVInstOpd),
}

#[derive(Clone, Debug)]
pub struct RISCVOpdSetAim {
    pub op: ParserRISCVInstOp,
    pub opds: Vec<RISCVOpdSetAimOpd>,
}

#[derive(Clone, Debug)]
pub struct RISCVOpdSet {
    pub hint: String,
    pub tokens: Vec<RISCVExpectToken>,
    pub aim_basics: Vec<RISCVOpdSetAim>,
}

pub use RISCVExpectImm::*;
pub use RISCVExpectToken::*;

// --------------------reg-------------------------
pub fn reg<T: ParserRISCVRegisterTrait + 'static>(reg: T) -> RISCVOpdSetAimOpd {
    RISCVOpdSetAimOpd::Val(ParserRISCVInstOpd::Reg(reg.into()))
}
// --------------------imm-------------------------
pub fn imm_i(imm: i128) -> RISCVOpdSetAimOpd {
    RISCVOpdSetAimOpd::Val(ParserRISCVInstOpd::Imm(ParserRISCVImmediate::Int(imm)))
}
// --------------------idx-------------------------
pub fn idx(idx: usize) -> RISCVOpdSetAimOpd {
    idx_handler(idx, |opd| opd)
}
pub fn idx_handler(
    idx: usize,
    handler: fn(ParserRISCVInstOpd) -> ParserRISCVInstOpd,
) -> RISCVOpdSetAimOpd {
    RISCVOpdSetAimOpd::Idx(RISCVOpdSetAimOpdIdx { idx, handler })
}
pub fn idx_handler_low(opd: ParserRISCVInstOpd) -> ParserRISCVInstOpd {
    if let ParserRISCVInstOpd::Imm(ParserRISCVImmediate::Int(i)) = opd {
        if i & 0x800 != 0 {
            ParserRISCVInstOpd::Imm(ParserRISCVImmediate::Int(-(i & 0x7ff)))
        } else {
            ParserRISCVInstOpd::Imm(ParserRISCVImmediate::Int(i & 0x7ff))
        }
    } else {
        opd
    }
}
pub fn idx_handler_high(opd: ParserRISCVInstOpd) -> ParserRISCVInstOpd {
    if let ParserRISCVInstOpd::Imm(ParserRISCVImmediate::Int(i)) = opd {
        if i & 0x800 != 0 {
            ParserRISCVInstOpd::Imm(ParserRISCVImmediate::Int(
                ((i as u32 + 0x0000_1000) >> 12) as i128,
            ))
        } else {
            ParserRISCVInstOpd::Imm(ParserRISCVImmediate::Int(((i as u32) >> 12) as i128))
        }
    } else {
        opd
    }
}
// --------------------expect-------------------------
pub fn expect_opd(opds: Vec<RISCVExpectToken>) -> Vec<RISCVExpectToken> {
    opds
}
pub fn expect_reg_any(any: RISCVExpectToken) -> Vec<RISCVExpectToken> {
    expect_opd(vec![Reg, Comma, any])
}
pub fn expect_reg_reg() -> Vec<RISCVExpectToken> {
    expect_reg_any(Reg)
}
pub fn expect_reg_reg_any(any: RISCVExpectToken) -> Vec<RISCVExpectToken> {
    expect_opd(vec![Reg, Comma, Reg, Comma, any])
}
pub fn expect_reg_reg_reg() -> Vec<RISCVExpectToken> {
    expect_reg_reg_any(Reg)
}
pub fn expect_csr(last_opd: RISCVExpectToken) -> Vec<RISCVExpectToken> {
    expect_opd(vec![Reg, Comma, Csr, Comma, last_opd])
}
// --------------------basic-------------------------
pub fn basic_op(op: ParserRISCVInstOp, opds: Vec<RISCVOpdSetAimOpd>) -> RISCVOpdSetAim {
    RISCVOpdSetAim { op, opds }
}
pub fn basic_op_02(op: ParserRISCVInstOp) -> RISCVOpdSetAim {
    basic_op(op, vec![idx(0), idx(2)])
}
pub fn basic_op_20(op: ParserRISCVInstOp) -> RISCVOpdSetAim {
    basic_op(op, vec![idx(2), idx(0)])
}
pub fn basic_op_024(op: ParserRISCVInstOp) -> RISCVOpdSetAim {
    basic_op(op, vec![idx(0), idx(2), idx(4)])
}
pub fn basic_op_042(op: ParserRISCVInstOp) -> RISCVOpdSetAim {
    basic_op(op, vec![idx(0), idx(4), idx(2)])
}
pub fn basic_op_204(op: ParserRISCVInstOp) -> RISCVOpdSetAim {
    basic_op(op, vec![idx(2), idx(0), idx(4)])
}
// --------------------hint-------------------------
pub fn hint_reg_reg_any(name: &str, any: &str, op: &str) -> String {
    format!("{} t1, t2, {} (t1 = t2 {} {})", name, any, op, any)
}
pub fn hint_reg_reg_reg(name: &str, op: &str) -> String {
    hint_reg_reg_any(name, "t3", op)
}
pub fn hint_branch(name: &str, cmp: &str, sign: &str) -> String {
    format!(
        "{} t1, t2, label (if t1 {} t2 goto label){}",
        name, cmp, sign
    )
}
pub fn hint_branch_zero(name: &str, cmp: &str, sign: &str) -> String {
    format!("{} t1, label (if t1 {} 0 goto label){}", name, cmp, sign)
}
pub fn hint_csr(name: &str, op: &str, last_opd: &str) -> String {
    format!(
        "{} t1, csr, {} (t1 = csr; csr {}{})",
        name, last_opd, op, last_opd
    )
}
pub fn hint_set_comparison(op: &str, last_opd: &str, signed: &str) -> String {
    format!("t1 = (t2 {} {}){}", op, last_opd, signed)
}
// --------------------set-------------------------
pub fn opd_set(
    expect: Vec<RISCVExpectToken>,
    basic: Vec<RISCVOpdSetAim>,
    hit: String,
) -> RISCVOpdSet {
    RISCVOpdSet {
        hint: hit,
        tokens: expect,
        aim_basics: basic,
    }
}
pub fn opd_set_no_opd(op: ParserRISCVInstOp, name: &str) -> RISCVOpdSet {
    opd_set(vec![], vec![basic_op(op, vec![])], name.to_string())
}
