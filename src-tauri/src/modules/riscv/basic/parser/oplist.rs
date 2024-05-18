use super::super::interface::parser::{
    get_32u_high,
    get_32u_low,
    ParserRISCVImmediate,
    ParserRISCVInstOp,
    ParserRISCVInstOpd,
    ParserRISCVLabelHandler,
    ParserRISCVRegister,
    RISCVImmediate,
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
pub fn reg<T>(reg: T) -> RISCVOpdSetAimOpd
where
    ParserRISCVRegister: From<T>,
{
    RISCVOpdSetAimOpd::Val(ParserRISCVInstOpd::Reg(ParserRISCVRegister::from(reg)))
}
// --------------------imm-------------------------
pub fn imm(imm: RISCVImmediate) -> RISCVOpdSetAimOpd {
    RISCVOpdSetAimOpd::Val(ParserRISCVInstOpd::Imm(ParserRISCVImmediate::Imm(imm)))
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
// used with idx_handler_high, i = u20 << 12 + i12, get the i12 imm
pub fn idx_handler_imm_low(opd: ParserRISCVInstOpd) -> ParserRISCVInstOpd {
    if let ParserRISCVInstOpd::Imm(ParserRISCVImmediate::Imm(i)) = opd {
        ParserRISCVInstOpd::Imm(ParserRISCVImmediate::Imm(get_32u_low(i)))
    } else {
        opd
    }
}
// used with idx_handler_low, i = u20 << 12 + i12, get the u20 imm
pub fn idx_handler_imm_high(opd: ParserRISCVInstOpd) -> ParserRISCVInstOpd {
    if let ParserRISCVInstOpd::Imm(ParserRISCVImmediate::Imm(i)) = opd {
        ParserRISCVInstOpd::Imm(ParserRISCVImmediate::Imm(get_32u_high(i)))
    } else {
        opd
    }
}
pub fn idx_handler_lbl_low(opd: ParserRISCVInstOpd) -> ParserRISCVInstOpd {
    if let ParserRISCVInstOpd::Lbl(lbl) = opd {
        ParserRISCVInstOpd::Imm(ParserRISCVImmediate::Lbl((
            lbl,
            ParserRISCVLabelHandler::Low,
        )))
    } else {
        opd
    }
}
pub fn idx_handler_lbl_high(opd: ParserRISCVInstOpd) -> ParserRISCVInstOpd {
    if let ParserRISCVInstOpd::Lbl(lbl) = opd {
        ParserRISCVInstOpd::Imm(ParserRISCVImmediate::Lbl((
            lbl,
            ParserRISCVLabelHandler::High,
        )))
    } else {
        opd
    }
}
pub fn idx_handler_lbl_last_delta_low(opd: ParserRISCVInstOpd) -> ParserRISCVInstOpd {
    if let ParserRISCVInstOpd::Lbl(lbl) = opd {
        ParserRISCVInstOpd::Imm(ParserRISCVImmediate::Lbl((
            lbl,
            ParserRISCVLabelHandler::DeltaMinusOneLow,
        )))
    } else {
        opd
    }
}
pub fn idx_handler_lbl_delta_high(opd: ParserRISCVInstOpd) -> ParserRISCVInstOpd {
    if let ParserRISCVInstOpd::Lbl(lbl) = opd {
        ParserRISCVInstOpd::Imm(ParserRISCVImmediate::Lbl((
            lbl,
            ParserRISCVLabelHandler::DeltaHigh,
        )))
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
// basic_op op idx(0) idx(2)
pub fn basic_op_02(op: ParserRISCVInstOp) -> RISCVOpdSetAim {
    basic_op(op, vec![idx(0), idx(2)])
}
#[allow(dead_code)]
// basic_op op idx(2) idx(0)
pub fn basic_op_20(op: ParserRISCVInstOp) -> RISCVOpdSetAim {
    basic_op(op, vec![idx(2), idx(0)])
}
// basic_op op idx(0) idx(2) idx(4)
pub fn basic_op_024(op: ParserRISCVInstOp) -> RISCVOpdSetAim {
    basic_op(op, vec![idx(0), idx(2), idx(4)])
}
// basic_op op idx(0) idx(4) idx(2)
pub fn basic_op_042(op: ParserRISCVInstOp) -> RISCVOpdSetAim {
    basic_op(op, vec![idx(0), idx(4), idx(2)])
}
// basic_op op idx(2) idx(0) idx(4)
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
pub fn hint_set_comparison(name: &str, op: &str, last_opd: &str, signed: &str) -> String {
    format!(
        "{} t1, t2, {} (t1 = (t2 {} {}){})",
        name, last_opd, op, last_opd, signed
    )
}
pub fn hint_set_comparison_zero(name: &str, op: &str, signed: &str) -> String {
    format!("{} t1, t2 (t1 = (t2 {} 0){})", name, op, signed)
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
