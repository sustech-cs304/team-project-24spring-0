use super::super::interface::parser::{
    ParserRISCVInstOp, ParserRISCVInstOpd, RISCVImmediate, RISCVRegister,
};
use super::lexer::RISCVOpToken;
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum RISCVImmediateType {
    U4,
    U5,
    U12,
    U20,
    U32,
    U64,
    I12,
    I20,
    I32,
    I64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum RISCVExpectToken {
    Comma,
    LParen,
    RParen,
    Reg,
    Csr,
    Imm(RISCVImmediateType),
    Lbl,
}

pub(super) struct RISCVOpdSetAimOpdIdx {
    pub idx: usize,
    pub handler: fn(ParserRISCVInstOpd) -> ParserRISCVInstOpd,
}

pub(super) enum RISCVOpdSetAimOpd {
    Idx(RISCVOpdSetAimOpdIdx),
    Val(ParserRISCVInstOpd),
}

pub(super) struct RISCVOpdSetAim {
    pub op: ParserRISCVInstOp,
    pub opds: Vec<RISCVOpdSetAimOpd>,
}

pub(super) struct RISCVOpdSet {
    pub hint: &'static str,
    pub tokens: Vec<RISCVExpectToken>,
    pub aim_basics: Vec<RISCVOpdSetAim>,
}

use RISCVExpectToken::*;
use RISCVImmediateType::*;
use RISCVRegister::*;
macro_rules! idx_def {
    ($idx:expr) => {
        RISCVOpdSetAimOpd::Idx(RISCVOpdSetAimOpdIdx {
            idx: $idx,
            handler: |opd| opd,
        })
    };
}
macro_rules! idx_low {
    ($idx:expr) => {
        RISCVOpdSetAimOpd::Idx(RISCVOpdSetAimOpdIdx {
            idx: $idx,
            handler: |opd| {
                if let ParserRISCVInstOpd::Imm(RISCVImmediate::Int(i)) = opd {
                    if i & 0x800 != 0 {
                        return ParserRISCVInstOpd::Imm(RISCVImmediate::Int(-(i & 0x7ff)));
                    } else {
                        return ParserRISCVInstOpd::Imm(RISCVImmediate::Int(i & 0x7ff));
                    }
                } else {
                    return opd;
                }
            },
        })
    };
}
macro_rules! idx_high {
    ($idx:expr) => {
        RISCVOpdSetAimOpd::Idx(RISCVOpdSetAimOpdIdx {
            idx: $idx,
            handler: |opd| {
                if let ParserRISCVInstOpd::Imm(RISCVImmediate::Int(i)) = opd {
                    if i & 0x800 != 0 {
                        return ParserRISCVInstOpd::Imm(RISCVImmediate::Int(
                            ((i as u32 + 0x0000_1000) >> 12) as i128,
                        ));
                    } else {
                        return ParserRISCVInstOpd::Imm(RISCVImmediate::Int(
                            ((i as u32) >> 12) as i128,
                        ));
                    }
                } else {
                    return opd;
                }
            },
        })
    };
}
macro_rules! val_reg {
    ($reg:expr) => {
        RISCVOpdSetAimOpd::Val(ParserRISCVInstOpd::Reg($reg))
    };
}
macro_rules! val_imm_i {
    ($imm:expr) => {
        RISCVOpdSetAimOpd::Val(ParserRISCVInstOpd::Imm(RISCVImmediate::Int($imm)))
    };
}
macro_rules! val_imm_f {
    ($imm:expr) => {
        RISCVOpdSetAimOpd::Val(ParserRISCVInstOpd::Imm(RISCVImmediate::Float($imm)))
    };
}

macro_rules! reg_reg_any_set_tmpl {
    ($inst:expr, $name:expr, $op:expr, $last_opd:expr, $last_opd_str:expr, $descr:expr, $opds:expr) => {
        RISCVOpdSet {
            hint: concat!($name, " t1, t2, ", $last_opd_str, " (", $descr, ")"),
            tokens: vec![Reg, Comma, Reg, Comma, $last_opd],
            aim_basics: vec![RISCVOpdSetAim {
                op: $inst,
                opds: $opds,
            }],
        }
    };
}
macro_rules! reg_reg_any_set {
    ($inst:expr, $name:expr, $op:expr, $last_opd:expr, $last_opd_str:expr) => {
        reg_reg_any_set!(
            $inst,
            $name,
            $op,
            $last_opd,
            $last_opd_str,
            concat!("t1 = t2 ", $op, " ", $last_opd_str)
        )
    };
    ($inst:expr, $name:expr, $op:expr, $last_opd:expr, $last_opd_str:expr, $descr:expr) => {
        reg_reg_any_set_tmpl!(
            $inst,
            $name,
            $op,
            $last_opd,
            $last_opd_str,
            $descr,
            vec![idx_def!(0), idx_def!(2), idx_def!(4)]
        )
    };
}
macro_rules! reg_reg_any_inv_set {
    ($inst:expr, $name:expr, $op:expr, $last_opd:expr, $last_opd_str:expr) => {
        reg_reg_any_set!(
            $inst,
            $name,
            $op,
            $last_opd,
            $last_opd_str,
            concat!("t1 = ", $last_opd_str, " ", $op, " t2")
        )
    };
    ($inst:expr, $name:expr, $op:expr, $last_opd:expr, $last_opd_str:expr, $descr:expr) => {
        reg_reg_any_set_tmpl!(
            $inst,
            $name,
            $op,
            $last_opd,
            $last_opd_str,
            $descr,
            vec![idx_def!(2), idx_def!(0), idx_def!(4)]
        )
    };
}
macro_rules! reg_reg_reg_set {
    ($inst:expr, $name:expr, $op:expr) => {
        reg_reg_any_set!($inst, $name, $op, Reg, "t3")
    };
    ($inst:expr, $name:expr, $op:expr, $reg_name:expr) => {
        reg_reg_any_set!($inst, $name, $op, Reg, $reg_name)
    };
    ($inst:expr, $name:expr, $op:expr, $reg_name:expr, $descr:expr) => {
        reg_reg_any_set!($inst, $name, $op, Reg, $reg_name, $descr)
    };
}
macro_rules! reg_reg_reg_inv_set {
    ($inst:expr, $name:expr, $op:expr) => {
        reg_reg_any_inv_set!($inst, $name, $op, Reg, "t3")
    };
    ($inst:expr, $name:expr, $op:expr, $reg_name:expr) => {
        reg_reg_any_inv_set!($inst, $name, $op, Reg, $reg_name)
    };
    ($inst:expr, $name:expr, $op:expr, $reg_name:expr, $descr:expr) => {
        reg_reg_any_inv_set!($inst, $name, $op, Reg, $reg_name, $descr)
    };
}
macro_rules! no_opd_set {
    ($inst:expr, $name:expr) => {
        RISCVOpdSet {
            hint: $name,
            tokens: vec![],
            aim_basics: vec![RISCVOpdSetAim {
                op: $inst,
                opds: vec![],
            }],
        }
    };
}
macro_rules! branch_vec_tmpl {
    ($inst:expr, $name:expr, $type:expr, $opds:expr, $sign:expr) => {
        vec![RISCVOpdSet {
            hint: concat!(
                $name,
                " t1, t2, label (if t1 ",
                $type,
                " t2 goto label)",
                $sign
            ),
            tokens: vec![Reg, Comma, Reg, Comma, Lbl],
            aim_basics: vec![RISCVOpdSetAim {
                op: $inst,
                opds: $opds,
            }],
        }]
    };
    ($inst:expr, $name:expr, $type:expr, $opds:expr, $sign:expr, $place_holder:expr) => {
        vec![RISCVOpdSet {
            hint: concat!($name, " t1, label (if t1 ", $type, " 0 goto label)", $sign),
            tokens: vec![Reg, Comma, Lbl],
            aim_basics: vec![RISCVOpdSetAim {
                op: $inst,
                opds: $opds,
            }],
        }]
    };
}
macro_rules! branch_vec {
    ($inst:expr, $name:expr, $type:expr, $sign:expr) => {
        branch_vec_tmpl!(
            $inst,
            $name,
            $type,
            vec![idx_def!(0), idx_def!(2), idx_def!(4)],
            $sign
        )
    };
}
macro_rules! branch_inv_vec {
    ($inst:expr, $name:expr, $type:expr, $sign:expr) => {
        branch_vec_tmpl!(
            $inst,
            $name,
            $type,
            vec![idx_def!(2), idx_def!(0), idx_def!(4)],
            $sign
        )
    };
}
macro_rules! branch_zero_vec {
    ($inst:expr, $name:expr, $type:expr, $sign:expr) => {
        branch_vec_tmpl!(
            $inst,
            $name,
            $type,
            vec![idx_def!(0), val_reg!(Zero), idx_def!(2)],
            $sign,
            ()
        )
    };
}
macro_rules! branch_zero_inv_vec {
    ($inst:expr, $name:expr, $type:expr, $sign:expr) => {
        branch_vec_tmpl!(
            $inst,
            $name,
            $type,
            vec![val_reg!(Zero), idx_def!(0), idx_def!(0)],
            $sign,
            ()
        )
    };
}
macro_rules! csr_vec {
    ($inst:expr, $name:expr, $type:expr, $last_opd:expr, $last_opd_str:expr) => {
        vec![RISCVOpdSet {
            hint: concat!(
                $name,
                " t1, csr, ",
                $last_opd_str,
                " (t1 = csr; csr ",
                $type,
                $last_opd_str
            ),
            tokens: vec![Reg, Comma, Csr, Comma, $last_opd],
            aim_basics: vec![RISCVOpdSetAim {
                op: $inst,
                opds: vec![idx_def!(0), idx_def!(2), idx_def!(4)],
            }],
        }]
    };
}
macro_rules! sl_mem_vec_tmpl {
    ($inst:expr, $name:expr, $type:expr, $dst0:expr, $dst1:expr, $dst2:expr, $dst3:expr, $dst4:expr, $src0:expr, $src1:expr, $src2:expr, $src3:expr,$src4:expr) => {
        vec![
            RISCVOpdSet {
                hint: concat!($name, " t1, -0x1(t2) (", $dst0, " = ", $type, $src0, ")"),
                tokens: vec![Reg, Comma, Imm(I12), LParen, Reg, RParen],
                aim_basics: vec![RISCVOpdSetAim {
                    op: $inst,
                    opds: vec![idx_def!(0), idx_def!(2), idx_def!(4)],
                }],
            },
            RISCVOpdSet {
                hint: concat!($name, " t1, (t2) (", $dst1, " = ", $type, $src1, ")"),
                tokens: vec![Reg, Comma, LParen, Reg, RParen],
                aim_basics: vec![RISCVOpdSetAim {
                    op: $inst,
                    opds: vec![idx_def!(0), val_imm_i!(0), idx_def!(3)],
                }],
            },
            RISCVOpdSet {
                hint: concat!($name, " t1, -0x1 (", $dst2, " = ", $type, $src2, ")"),
                tokens: vec![Reg, Comma, Imm(I12)],
                aim_basics: vec![RISCVOpdSetAim {
                    op: $inst,
                    opds: vec![idx_def!(0), idx_def!(2), val_reg!(Zero)],
                }],
            },
            RISCVOpdSet {
                hint: concat!(
                    $name,
                    " t1, 0x100000 (a0 = 0x100000[12:31](i32); ",
                    $dst3,
                    " = ",
                    $type,
                    $src3,
                    ")"
                ),
                tokens: vec![Reg, Comma, Imm(I32)],
                aim_basics: vec![
                    RISCVOpdSetAim {
                        op: ParserRISCVInstOp::Lui,
                        opds: vec![val_reg!(A0), idx_high!(2)],
                    },
                    RISCVOpdSetAim {
                        op: $inst,
                        opds: vec![idx_def!(0), idx_low!(2), val_reg!(A0)],
                    },
                ],
            },
            RISCVOpdSet {
                hint: concat!($name, " t1, label (", $dst4, " = ", $type, $src4, ")"),
                tokens: vec![Reg, Comma, Lbl],
                aim_basics: vec![RISCVOpdSetAim {
                    op: $inst,
                    opds: vec![idx_def!(0), idx_def!(2)],
                }],
            },
        ]
    };
}
macro_rules! load_mem_vec {
    ($inst:expr, $name:expr, $type:expr) => {
        sl_mem_vec_tmpl!(
            $inst,
            $name,
            $type,
            "t1",
            "t1",
            "t1",
            "t1",
            "t1",
            "mem[t2 + -0x1(i12)]",
            "mem[t2]",
            "mem[-0x1(i12)]",
            "mem[a0 + 0x100000[0:11](i32)]",
            "mem[label]"
        )
    };
}
macro_rules! store_mem_vec {
    ($inst:expr, $name:expr, $type:expr) => {
        sl_mem_vec_tmpl!(
            $inst,
            $name,
            $type,
            "mem[t2 + -0x1(i12)]",
            "mem[t2]",
            "mem[-0x1(i12)]",
            "mem[a0 + 0x100000[0:11](i32)]",
            "mem[label]",
            "t1",
            "t1",
            "t1",
            "t1",
            "t1"
        )
    };
}

lazy_static! {
    pub(super) static ref OP_LIST: HashMap<RISCVOpToken, Vec<RISCVOpdSet>> = HashMap::from([
        (
            RISCVOpToken::Add,
            vec![reg_reg_reg_set!(ParserRISCVInstOp::Add, "add", "+")]
        ),
        (
            RISCVOpToken::Addi,
            vec![
                reg_reg_any_set!(ParserRISCVInstOp::Addi, "addi", "+", Imm(I12), "-0x1(i12)"),
                reg_reg_any_set!(ParserRISCVInstOp::Addi, "addi", "+", Lbl, "label[0:11]"),
            ]
        ),
        (
            RISCVOpToken::And,
            vec![reg_reg_reg_set!(ParserRISCVInstOp::And, "and", "&")]
        ),
        (
            RISCVOpToken::Andi,
            vec![reg_reg_any_set!(
                ParserRISCVInstOp::Andi,
                "andi",
                "&",
                Imm(U12),
                "0x1(u12)"
            )]
        ),
        (
            RISCVOpToken::Auipc,
            vec![RISCVOpdSet {
                hint: "auipc t1, 0x1000 (t1 = pc + 0x1000(u20))",
                tokens: vec![Reg, Comma, Imm(U20)],
                aim_basics: vec![RISCVOpdSetAim {
                    op: ParserRISCVInstOp::Auipc,
                    opds: vec![idx_def!(0), idx_def!(2)],
                }],
            }]
        ),
        (
            RISCVOpToken::Beq,
            branch_vec!(ParserRISCVInstOp::Beq, "beq", "==", " (signed)")
        ),
        (
            RISCVOpToken::Bge,
            branch_vec!(ParserRISCVInstOp::Bge, "bge", ">=", " (signed)")
        ),
        (
            RISCVOpToken::Bgeu,
            branch_vec!(ParserRISCVInstOp::Bgeu, "bgeu", ">=", " (unsigned)")
        ),
        (
            RISCVOpToken::Blt,
            branch_vec!(ParserRISCVInstOp::Blt, "blt", "<", " (signed)")
        ),
        (
            RISCVOpToken::Bltu,
            branch_vec!(ParserRISCVInstOp::Bltu, "bltu", "<", " (unsigned)")
        ),
        (
            RISCVOpToken::Bne,
            branch_vec!(ParserRISCVInstOp::Bne, "bne", "!=", " (signed)")
        ),
        (
            RISCVOpToken::Csrrc,
            csr_vec!(ParserRISCVInstOp::Csrrc, "csrrc", "&= ~", Reg, "t2")
        ),
        (
            RISCVOpToken::Csrrci,
            csr_vec!(
                ParserRISCVInstOp::Csrrci,
                "csrrci",
                "&= ~",
                Imm(U5),
                "0x1(u5)"
            )
        ),
        (
            RISCVOpToken::Csrrs,
            csr_vec!(ParserRISCVInstOp::Csrrs, "csrrs", "|= ", Reg, "t2")
        ),
        (
            RISCVOpToken::Csrrsi,
            csr_vec!(
                ParserRISCVInstOp::Csrrsi,
                "csrrsi",
                "|= ",
                Imm(U5),
                "0x1(u5)"
            )
        ),
        (
            RISCVOpToken::Csrrw,
            csr_vec!(ParserRISCVInstOp::Csrrw, "csrrw", "= ", Reg, "t2")
        ),
        (
            RISCVOpToken::Csrrwi,
            csr_vec!(
                ParserRISCVInstOp::Csrrwi,
                "csrrwi",
                "= ",
                Imm(U5),
                "0x1(u5)"
            )
        ),
        (RISCVOpToken::Div, vec![]),
        (RISCVOpToken::Divu, vec![]),
        (
            RISCVOpToken::Ebreak,
            vec![no_opd_set!(ParserRISCVInstOp::Ebreak, "ebreak")]
        ),
        (
            RISCVOpToken::Ecall,
            vec![no_opd_set!(ParserRISCVInstOp::Ecall, "ecall")]
        ),
        (RISCVOpToken::FaddD, vec![]),
        (RISCVOpToken::FaddS, vec![]),
        (RISCVOpToken::FclassD, vec![]),
        (RISCVOpToken::FclassS, vec![]),
        (RISCVOpToken::FcvtDS, vec![]),
        (RISCVOpToken::FcvtDW, vec![]),
        (RISCVOpToken::FcvtDWu, vec![]),
        (RISCVOpToken::FcvtSD, vec![]),
        (RISCVOpToken::FcvtSW, vec![]),
        (RISCVOpToken::FcvtSWu, vec![]),
        (RISCVOpToken::FcvtWD, vec![]),
        (RISCVOpToken::FcvtWS, vec![]),
        (RISCVOpToken::FcvtWuD, vec![]),
        (RISCVOpToken::FcvtWuS, vec![]),
        (RISCVOpToken::FdivD, vec![]),
        (RISCVOpToken::FdivS, vec![]),
        (
            RISCVOpToken::Fence,
            vec![RISCVOpdSet {
                hint: "fence 0x1(u4), 0x1(u4)",
                tokens: vec![Imm(U4), Comma, Imm(U4)],
                aim_basics: vec![RISCVOpdSetAim {
                    op: ParserRISCVInstOp::Fence,
                    opds: vec![idx_def!(0), idx_def!(2)],
                }],
            }]
        ),
        (
            RISCVOpToken::FenceI,
            vec![no_opd_set!(ParserRISCVInstOp::FenceI, "fence.i")]
        ),
        (RISCVOpToken::FeqD, vec![]),
        (RISCVOpToken::FeqS, vec![]),
        (RISCVOpToken::Fld, vec![]),
        (RISCVOpToken::FleD, vec![]),
        (RISCVOpToken::FleS, vec![]),
        (RISCVOpToken::FltD, vec![]),
        (RISCVOpToken::FltS, vec![]),
        (RISCVOpToken::Flw, vec![]),
        (RISCVOpToken::FmaddD, vec![]),
        (RISCVOpToken::FmaddS, vec![]),
        (RISCVOpToken::FmaxD, vec![]),
        (RISCVOpToken::FmaxS, vec![]),
        (RISCVOpToken::FminD, vec![]),
        (RISCVOpToken::FminS, vec![]),
        (RISCVOpToken::FmsubD, vec![]),
        (RISCVOpToken::FmsubS, vec![]),
        (RISCVOpToken::FmulD, vec![]),
        (RISCVOpToken::FmulS, vec![]),
        (RISCVOpToken::FmvSX, vec![]),
        (RISCVOpToken::FmvXS, vec![]),
        (RISCVOpToken::FnmaddD, vec![]),
        (RISCVOpToken::FnmaddS, vec![]),
        (RISCVOpToken::FnmsubD, vec![]),
        (RISCVOpToken::FnmsubS, vec![]),
        (RISCVOpToken::Fsd, vec![]),
        (RISCVOpToken::FsgnjD, vec![]),
        (RISCVOpToken::FsgnjS, vec![]),
        (RISCVOpToken::FsgnjnD, vec![]),
        (RISCVOpToken::FsgnjnS, vec![]),
        (RISCVOpToken::FsgnjxD, vec![]),
        (RISCVOpToken::FsgnjxS, vec![]),
        (RISCVOpToken::FsqrtD, vec![]),
        (RISCVOpToken::FsqrtS, vec![]),
        (RISCVOpToken::FsubD, vec![]),
        (RISCVOpToken::FsubS, vec![]),
        (RISCVOpToken::Fsw, vec![]),
        (
            RISCVOpToken::Jal,
            vec![
                RISCVOpdSet {
                    hint: "jal label (ra = pc + 4; pc = label)",
                    tokens: vec![Lbl],
                    aim_basics: vec![RISCVOpdSetAim {
                        op: ParserRISCVInstOp::Jal,
                        opds: vec![val_reg!(Ra), idx_def!(0)],
                    }],
                },
                RISCVOpdSet {
                    hint: "jal t1, label (t1 = pc + 4; pc = label)",
                    tokens: vec![Reg, Comma, Lbl],
                    aim_basics: vec![RISCVOpdSetAim {
                        op: ParserRISCVInstOp::Jal,
                        opds: vec![idx_def!(0), idx_def!(2)],
                    }],
                },
            ]
        ),
        (
            RISCVOpToken::Jalr,
            vec![
                RISCVOpdSet {
                    hint: "jalr t1, t2, -0x1 (t1 = pc + 4; pc = t2 + -0x1(i12))",
                    tokens: vec![Reg, Comma, Reg, Comma, Imm(I12)],
                    aim_basics: vec![RISCVOpdSetAim {
                        op: ParserRISCVInstOp::Jalr,
                        opds: vec![idx_def!(0), idx_def!(2), idx_def!(4)],
                    }],
                },
                RISCVOpdSet {
                    hint: "jalr t0 (ra = pc + 4; pc = t0)",
                    tokens: vec![Reg],
                    aim_basics: vec![RISCVOpdSetAim {
                        op: ParserRISCVInstOp::Jalr,
                        opds: vec![val_reg!(Ra), idx_def!(0), val_imm_i!(0)],
                    }],
                },
                RISCVOpdSet {
                    hint: "jalr t1, -0x1 (ra = pc + 4; pc = t1 + -0x1(i12))",
                    tokens: vec![Reg, Comma, Imm(I12)],
                    aim_basics: vec![RISCVOpdSetAim {
                        op: ParserRISCVInstOp::Jalr,
                        opds: vec![val_reg!(Ra), idx_def!(0), idx_def!(2)],
                    }],
                },
                RISCVOpdSet {
                    hint: "jalr t1, -0x1(t2) (t1 = pc + 4; pc = t2 + -0x1(i12))",
                    tokens: vec![Reg, Comma, Imm(I12), LParen, Reg, RParen],
                    aim_basics: vec![RISCVOpdSetAim {
                        op: ParserRISCVInstOp::Jalr,
                        opds: vec![idx_def!(0), idx_def!(4), idx_def!(2)],
                    }],
                },
            ]
        ),
        (
            RISCVOpToken::Lb,
            load_mem_vec!(ParserRISCVInstOp::Lb, "lb", "(i8)")
        ),
        (
            RISCVOpToken::Lbu,
            load_mem_vec!(ParserRISCVInstOp::Lbu, "lbu", "(u8)")
        ),
        (
            RISCVOpToken::Lh,
            load_mem_vec!(ParserRISCVInstOp::Lh, "lh", "(i16)")
        ),
        (
            RISCVOpToken::Lhu,
            load_mem_vec!(ParserRISCVInstOp::Lhu, "lhu", "(u16)")
        ),
        (
            RISCVOpToken::Lui,
            vec![
                RISCVOpdSet {
                    hint: "lui t1, 0x1000 (t1 = 0x1000(u20))",
                    tokens: vec![Reg, Comma, Imm(U20)],
                    aim_basics: vec![RISCVOpdSetAim {
                        op: ParserRISCVInstOp::Lui,
                        opds: vec![idx_def!(0), idx_def!(2)],
                    }],
                },
                RISCVOpdSet {
                    hint: "lui t1, label (t1 = label)",
                    tokens: vec![Reg, Comma, Lbl],
                    aim_basics: vec![RISCVOpdSetAim {
                        op: ParserRISCVInstOp::Lui,
                        opds: vec![idx_def!(0), idx_def!(2)],
                    }],
                },
            ]
        ),
        (
            RISCVOpToken::Lw,
            load_mem_vec!(ParserRISCVInstOp::Lw, "lw", "")
        ),
        (RISCVOpToken::Mul, vec![]),
        (RISCVOpToken::Mulh, vec![]),
        (RISCVOpToken::Mulhsu, vec![]),
        (RISCVOpToken::Mulhu, vec![]),
        (
            RISCVOpToken::Or,
            vec![reg_reg_reg_set!(ParserRISCVInstOp::Or, "or", "|")]
        ),
        (
            RISCVOpToken::Ori,
            vec![reg_reg_any_set!(
                ParserRISCVInstOp::Ori,
                "ori",
                "|",
                Imm(U12),
                "0x1(u12)"
            )]
        ),
        (RISCVOpToken::Rem, vec![]),
        (RISCVOpToken::Remu, vec![]),
        (
            RISCVOpToken::Sb,
            store_mem_vec!(ParserRISCVInstOp::Sb, "sb", "(u8)")
        ),
        (
            RISCVOpToken::Sh,
            store_mem_vec!(ParserRISCVInstOp::Sh, "sh", "(u16)")
        ),
        (
            RISCVOpToken::Sll,
            vec![reg_reg_reg_set!(
                ParserRISCVInstOp::Sll,
                "sll",
                "<<",
                "t3[0:4]"
            )]
        ),
        (
            RISCVOpToken::Slli,
            vec![reg_reg_any_set!(
                ParserRISCVInstOp::Slli,
                "slli",
                "<<",
                Imm(U5),
                "0x1(u5)"
            )]
        ),
        (
            RISCVOpToken::Slt,
            vec![reg_reg_reg_set!(
                ParserRISCVInstOp::Slt,
                "slt",
                "",
                "t3",
                "t1 = (t2 < t3) (signed)"
            )]
        ),
        (
            RISCVOpToken::Slti,
            vec![reg_reg_any_set!(
                ParserRISCVInstOp::Slti,
                "slti",
                "",
                Imm(I12),
                "-0x1(i12)",
                "t1 = (t2 < -0x1(i12))"
            )]
        ),
        (
            RISCVOpToken::Sltiu,
            vec![reg_reg_any_set!(
                ParserRISCVInstOp::Sltiu,
                "sltiu",
                "",
                Imm(U12),
                "0x1(u12)",
                "t1 = (t2 < 0x1(u12))"
            )]
        ),
        (
            RISCVOpToken::Sltu,
            vec![reg_reg_reg_set!(
                ParserRISCVInstOp::Sltu,
                "sltu",
                "",
                "t3",
                "t1 = (t2 < t3) (unsigned)"
            )]
        ),
        (
            RISCVOpToken::Sra,
            vec![reg_reg_reg_set!(
                ParserRISCVInstOp::Sra,
                "sra",
                ">>",
                "t3[0:4]"
            )]
        ),
        (
            RISCVOpToken::Srai,
            vec![reg_reg_any_set!(
                ParserRISCVInstOp::Srai,
                "srai",
                ">>",
                Imm(U5),
                "0x1(u5)"
            )]
        ),
        (
            RISCVOpToken::Srl,
            vec![reg_reg_reg_set!(
                ParserRISCVInstOp::Srl,
                "srl",
                ">>",
                "t3[0:4]"
            )]
        ),
        (
            RISCVOpToken::Srli,
            vec![reg_reg_any_set!(
                ParserRISCVInstOp::Srli,
                "srli",
                ">>",
                Imm(U5),
                "0x1(u5)"
            )]
        ),
        (
            RISCVOpToken::Sub,
            vec![reg_reg_reg_set!(ParserRISCVInstOp::Sub, "sub", "-")]
        ),
        (
            RISCVOpToken::Sw,
            store_mem_vec!(ParserRISCVInstOp::Sw, "sw", "")
        ),
        (RISCVOpToken::Uret, vec![]),
        (RISCVOpToken::Wfi, vec![]),
        (
            RISCVOpToken::Xor,
            vec![reg_reg_reg_set!(ParserRISCVInstOp::Xor, "xor", "^")]
        ),
        (
            RISCVOpToken::Xori,
            vec![reg_reg_any_set!(
                ParserRISCVInstOp::Xori,
                "xori",
                "^",
                Imm(U12),
                "0x1(u12)"
            )]
        ),
        (
            RISCVOpToken::B,
            vec![RISCVOpdSet {
                hint: "b label (ra = pc + 4; pc = label)",
                tokens: vec![Lbl],
                aim_basics: vec![RISCVOpdSetAim {
                    op: ParserRISCVInstOp::Jal,
                    opds: vec![val_reg!(Ra), idx_def!(0)],
                }],
            }]
        ),
        (
            RISCVOpToken::Beqz,
            branch_zero_vec!(ParserRISCVInstOp::Beq, "beqz", "==", "")
        ),
        (
            RISCVOpToken::Bgez,
            branch_zero_vec!(ParserRISCVInstOp::Bge, "bgez", ">=", " (signed)")
        ),
        (
            RISCVOpToken::Bgt,
            branch_inv_vec!(ParserRISCVInstOp::Blt, "bgt", ">", " (signed)"),
        ),
        (
            RISCVOpToken::Bgtu,
            branch_inv_vec!(ParserRISCVInstOp::Bltu, "bgtu", ">", " (unsigned)"),
        ),
        (
            RISCVOpToken::Bgtz,
            branch_zero_inv_vec!(ParserRISCVInstOp::Blt, "bgtz", ">", " (signed)")
        ),
        (
            RISCVOpToken::Ble,
            branch_inv_vec!(ParserRISCVInstOp::Bge, "ble", "<=", " (signed)")
        ),
        (
            RISCVOpToken::Bleu,
            branch_inv_vec!(ParserRISCVInstOp::Bgeu, "bleu", "<=", " (unsigned)")
        ),
        (
            RISCVOpToken::Blez,
            branch_zero_inv_vec!(ParserRISCVInstOp::Bge, "blez", "<=", " (signed)")
        ),
        (
            RISCVOpToken::Bltz,
            branch_zero_vec!(ParserRISCVInstOp::Blt, "bltz", "<", " (signed)")
        ),
        (
            RISCVOpToken::Bnez,
            branch_zero_vec!(ParserRISCVInstOp::Bne, "bnez", "!=", "")
        ),
        (RISCVOpToken::Call, vec![]),
        (RISCVOpToken::Csrc, vec![]),
        (RISCVOpToken::Csrci, vec![]),
        (RISCVOpToken::Csrr, vec![]),
        (RISCVOpToken::Csrs, vec![]),
        (RISCVOpToken::Csrsi, vec![]),
        (RISCVOpToken::Csrw, vec![]),
        (RISCVOpToken::Csrwi, vec![]),
        (RISCVOpToken::FabsD, vec![]),
        (RISCVOpToken::FabsS, vec![]),
        (RISCVOpToken::FgeD, vec![]),
        (RISCVOpToken::FgeS, vec![]),
        (RISCVOpToken::FgtD, vec![]),
        (RISCVOpToken::FgtS, vec![]),
        (RISCVOpToken::FmvD, vec![]),
        (RISCVOpToken::FmvS, vec![]),
        (RISCVOpToken::FmvWX, vec![]),
        (RISCVOpToken::FmvXW, vec![]),
        (RISCVOpToken::FnegD, vec![]),
        (RISCVOpToken::FnegS, vec![]),
        (RISCVOpToken::Frcsr, vec![]),
        (RISCVOpToken::Frflags, vec![]),
        (RISCVOpToken::Frrm, vec![]),
        (RISCVOpToken::Frsr, vec![]),
        (RISCVOpToken::Fsflags, vec![]),
        (RISCVOpToken::Fsrm, vec![]),
        (RISCVOpToken::Fsrr, vec![]),
        (RISCVOpToken::J, vec![]),
        (RISCVOpToken::Jr, vec![]),
        (RISCVOpToken::La, vec![]),
        (RISCVOpToken::Li, vec![]),
        (RISCVOpToken::Mv, vec![]),
        (RISCVOpToken::Neg, vec![]),
        (
            RISCVOpToken::Nop,
            vec![RISCVOpdSet {
                hint: "nop",
                tokens: vec![],
                aim_basics: vec![RISCVOpdSetAim {
                    op: ParserRISCVInstOp::Addi,
                    opds: vec![val_reg!(Zero), val_reg!(Zero), val_imm_i!(0)],
                }],
            }]
        ),
        (RISCVOpToken::Not, vec![]),
        (RISCVOpToken::Rdcycle, vec![]),
        (RISCVOpToken::Rdcycleh, vec![]),
        (RISCVOpToken::Rdinstret, vec![]),
        (RISCVOpToken::Rdinstreth, vec![]),
        (RISCVOpToken::Rdtime, vec![]),
        (RISCVOpToken::Rdtimeh, vec![]),
        (RISCVOpToken::Ret, vec![]),
        (RISCVOpToken::Seqz, vec![]),
        (RISCVOpToken::SextB, vec![]),
        (RISCVOpToken::SextH, vec![]),
        (RISCVOpToken::Sgt, vec![]),
        (RISCVOpToken::Sgtu, vec![]),
        (RISCVOpToken::Sgtz, vec![]),
        (RISCVOpToken::Sltz, vec![]),
        (RISCVOpToken::Snez, vec![]),
        (RISCVOpToken::Tail, vec![]),
        (RISCVOpToken::ZextB, vec![]),
        (RISCVOpToken::ZextH, vec![]),
    ]);
}
