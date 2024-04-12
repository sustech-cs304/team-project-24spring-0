use super::super::interface::parser::ParserRISCVInstOp;
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

pub(super) struct RISCVOpdSetAim {
    pub op: ParserRISCVInstOp,
    pub opds_idx: Vec<usize>,
}

pub(super) struct RISCVOpdSet {
    pub hint: &'static str,
    pub tokens: Vec<RISCVExpectToken>,
    pub aim_basics: Vec<RISCVOpdSetAim>,
}

use RISCVExpectToken::*;
use RISCVImmediateType::*;

lazy_static! {
    pub(super) static ref OP_LIST: HashMap<RISCVOpToken, Vec<RISCVOpdSet>> = HashMap::from([
        (
            RISCVOpToken::Add,
            vec![RISCVOpdSet {
                hint: "add t1, t2, t3 (t1 = t2 + t3)",
                tokens: vec![Reg, Comma, Reg, Comma, Reg],
                aim_basics: vec![RISCVOpdSetAim {
                    op: ParserRISCVInstOp::Add,
                    opds_idx: vec![0, 2, 4],
                },],
            },],
        ),
        (
            RISCVOpToken::Addi,
            vec![
                RISCVOpdSet {
                    hint: "addi t1, t2, -0x1 (t1 = t2 - 0x1(i12))",
                    tokens: vec![Reg, Comma, Reg, Comma, Imm(I12)],
                    aim_basics: vec![RISCVOpdSetAim {
                        op: ParserRISCVInstOp::Addi,
                        opds_idx: vec![0, 2, 4],
                    },],
                },
                RISCVOpdSet {
                    hint: "addi t1, t2, label (t1 = t2 + label(lower 12 bits))",
                    tokens: vec![Reg, Comma, Reg, Comma, Lbl],
                    aim_basics: vec![RISCVOpdSetAim {
                        op: ParserRISCVInstOp::Addi,
                        opds_idx: vec![0, 2, 4],
                    },],
                },
            ],
        ),
        (
            RISCVOpToken::And,
            vec![RISCVOpdSet {
                hint: "and t1, t2, t3 (t1 = t2 & t3)",
                tokens: vec![Reg, Comma, Reg, Comma, Reg],
                aim_basics: vec![RISCVOpdSetAim {
                    op: ParserRISCVInstOp::And,
                    opds_idx: vec![0, 2, 4],
                },],
            },],
        ),
        (
            RISCVOpToken::Andi,
            vec![RISCVOpdSet {
                hint: "andi t1, t2, 0x1 (t1 = t2 & 0x1(u12))",
                tokens: vec![Reg, Comma, Reg, Comma, Imm(U12)],
                aim_basics: vec![RISCVOpdSetAim {
                    op: ParserRISCVInstOp::Andi,
                    opds_idx: vec![0, 2, 4],
                },],
            },],
        ),
        (
            RISCVOpToken::Auipc,
            vec![RISCVOpdSet {
                hint: "auipc t1, 0x1000 (t1 = pc + 0x1000(u20))",
                tokens: vec![Reg, Comma, Imm(U20)],
                aim_basics: vec![RISCVOpdSetAim {
                    op: ParserRISCVInstOp::Auipc,
                    opds_idx: vec![0, 2],
                },],
            },],
        ),
        (
            RISCVOpToken::Beq,
            vec![RISCVOpdSet {
                hint: "beq t1, t2, label (if t1 == t2 goto label)",
                tokens: vec![Reg, Comma, Reg, Comma, Lbl],
                aim_basics: vec![RISCVOpdSetAim {
                    op: ParserRISCVInstOp::Beq,
                    opds_idx: vec![0, 2, 4],
                },],
            },],
        ),
        (
            RISCVOpToken::Bge,
            vec![RISCVOpdSet {
                hint: "bge t1, t2, label (if t1 >= t2 goto label)",
                tokens: vec![Reg, Comma, Reg, Comma, Lbl],
                aim_basics: vec![RISCVOpdSetAim {
                    op: ParserRISCVInstOp::Bge,
                    opds_idx: vec![0, 2, 4],
                },],
            },],
        ),
        (
            RISCVOpToken::Bgeu,
            vec![RISCVOpdSet {
                hint: "bgeu t1, t2, label (if t1 >= t2 goto label)(unsigned)",
                tokens: vec![Reg, Comma, Reg, Comma, Lbl],
                aim_basics: vec![RISCVOpdSetAim {
                    op: ParserRISCVInstOp::Bgeu,
                    opds_idx: vec![0, 2, 4],
                },],
            },],
        ),
        (
            RISCVOpToken::Blt,
            vec![RISCVOpdSet {
                hint: "blt t1, t2, label (if t1 < t2 goto label)",
                tokens: vec![Reg, Comma, Reg, Comma, Lbl],
                aim_basics: vec![RISCVOpdSetAim {
                    op: ParserRISCVInstOp::Blt,
                    opds_idx: vec![0, 2, 4],
                },],
            },],
        ),
        (
            RISCVOpToken::Bltu,
            vec![RISCVOpdSet {
                hint: "bltu t1, t2, label (if t1 < t2 goto label)(unsigned)",
                tokens: vec![Reg, Comma, Reg, Comma, Lbl],
                aim_basics: vec![RISCVOpdSetAim {
                    op: ParserRISCVInstOp::Bltu,
                    opds_idx: vec![0, 2, 4],
                },],
            },],
        ),
        (
            RISCVOpToken::Bne,
            vec![RISCVOpdSet {
                hint: "bne t1, t2, label (if t1 != t2 goto label)",
                tokens: vec![Reg, Comma, Reg, Comma, Lbl],
                aim_basics: vec![RISCVOpdSetAim {
                    op: ParserRISCVInstOp::Bne,
                    opds_idx: vec![0, 2, 4],
                },],
            },],
        ),
        (
            RISCVOpToken::Csrrc,
            vec![RISCVOpdSet {
                hint: "csrrc t1, csr, t2 (t1 = csr; csr &= ~t2)",
                tokens: vec![Reg, Comma, Csr, Comma, Reg],
                aim_basics: vec![RISCVOpdSetAim {
                    op: ParserRISCVInstOp::Csrrc,
                    opds_idx: vec![0, 2, 4],
                },],
            },],
        ),
        (
            RISCVOpToken::Csrrci,
            vec![RISCVOpdSet {
                hint: "csrrci t1, csr, 0x1 (t1 = csr; csr &= ~0x1(u5))",
                tokens: vec![Reg, Comma, Csr, Comma, Imm(U5)],
                aim_basics: vec![RISCVOpdSetAim {
                    op: ParserRISCVInstOp::Csrrci,
                    opds_idx: vec![0, 2, 4],
                },],
            },],
        ),
        (
            RISCVOpToken::Csrrs,
            vec![RISCVOpdSet {
                hint: "csrrs t1, csr, t2 (t1 = csr; csr |= t2)",
                tokens: vec![Reg, Comma, Csr, Comma, Reg],
                aim_basics: vec![RISCVOpdSetAim {
                    op: ParserRISCVInstOp::Csrrs,
                    opds_idx: vec![0, 2, 4],
                },],
            },],
        ),
        (
            RISCVOpToken::Csrrsi,
            vec![RISCVOpdSet {
                hint: "csrrsi t1, csr, 0x1 (t1 = csr; csr |= 0x1(u5))",
                tokens: vec![Reg, Comma, Csr, Comma, Imm(U5)],
                aim_basics: vec![RISCVOpdSetAim {
                    op: ParserRISCVInstOp::Csrrsi,
                    opds_idx: vec![0, 2, 4],
                },],
            },],
        ),
        (
            RISCVOpToken::Csrrw,
            vec![RISCVOpdSet {
                hint: "csrrw t1, csr, t2 (t1 = csr; csr = t2)",
                tokens: vec![Reg, Comma, Csr, Comma, Reg],
                aim_basics: vec![RISCVOpdSetAim {
                    op: ParserRISCVInstOp::Csrrw,
                    opds_idx: vec![0, 2, 4],
                },],
            },],
        ),
        (
            RISCVOpToken::Csrrwi,
            vec![RISCVOpdSet {
                hint: "csrrwi t1, csr, 0x1 (t1 = csr; csr = 0x1(u5))",
                tokens: vec![Reg, Comma, Csr, Comma, Imm(U5)],
                aim_basics: vec![RISCVOpdSetAim {
                    op: ParserRISCVInstOp::Csrrwi,
                    opds_idx: vec![0, 2, 4],
                },],
            },],
        ),
        (RISCVOpToken::Div, vec![]),
        (RISCVOpToken::Divu, vec![]),
        (
            RISCVOpToken::Ebreak,
            vec![RISCVOpdSet {
                hint: "ebreak",
                tokens: vec![],
                aim_basics: vec![RISCVOpdSetAim {
                    op: ParserRISCVInstOp::Ebreak,
                    opds_idx: vec![],
                },],
            }],
        ),
        (
            RISCVOpToken::Ecall,
            vec![RISCVOpdSet {
                hint: "ecall",
                tokens: vec![],
                aim_basics: vec![RISCVOpdSetAim {
                    op: ParserRISCVInstOp::Ecall,
                    opds_idx: vec![],
                },],
            }],
        ),
        (RISCVOpToken::FaddD, vec![],),
        (RISCVOpToken::FaddS, vec![],),
        (RISCVOpToken::FclassD, vec![],),
        (RISCVOpToken::FclassS, vec![],),
        (RISCVOpToken::FcvtDS, vec![],),
        (RISCVOpToken::FcvtDW, vec![],),
        (RISCVOpToken::FcvtDWu, vec![],),
        (RISCVOpToken::FcvtSD, vec![],),
        (RISCVOpToken::FcvtSW, vec![],),
        (RISCVOpToken::FcvtSWu, vec![],),
        (RISCVOpToken::FcvtWD, vec![],),
        (RISCVOpToken::FcvtWS, vec![],),
        (RISCVOpToken::FcvtWuD, vec![],),
        (RISCVOpToken::FcvtWuS, vec![],),
        (RISCVOpToken::FdivD, vec![],),
        (RISCVOpToken::FdivS, vec![],),
        (
            RISCVOpToken::Fence,
            vec![RISCVOpdSet {
                hint: "fence 0x1(u4), 0x1(u4)",
                tokens: vec![Imm(U4), Comma, Imm(U4)],
                aim_basics: vec![RISCVOpdSetAim {
                    op: ParserRISCVInstOp::Fence,
                    opds_idx: vec![0, 2],
                },],
            }],
        ),
        (
            RISCVOpToken::FenceI,
            vec![RISCVOpdSet {
                hint: "fence.i",
                tokens: vec![],
                aim_basics: vec![RISCVOpdSetAim {
                    op: ParserRISCVInstOp::FenceI,
                    opds_idx: vec![],
                },],
            }],
        ),
        (RISCVOpToken::FeqD, vec![],),
        (RISCVOpToken::FeqS, vec![],),
        (RISCVOpToken::Fld, vec![],),
        (RISCVOpToken::FleD, vec![],),
        (RISCVOpToken::FleS, vec![],),
        (RISCVOpToken::FltD, vec![],),
        (RISCVOpToken::FltS, vec![],),
        (RISCVOpToken::Flw, vec![],),
        (RISCVOpToken::FmaddD, vec![],),
        (RISCVOpToken::FmaddS, vec![],),
        (RISCVOpToken::FmaxD, vec![],),
        (RISCVOpToken::FmaxS, vec![],),
        (RISCVOpToken::FminD, vec![],),
        (RISCVOpToken::FminS, vec![],),
        (RISCVOpToken::FmsubD, vec![],),
        (RISCVOpToken::FmsubS, vec![],),
        (RISCVOpToken::FmulD, vec![],),
        (RISCVOpToken::FmulS, vec![],),
        (RISCVOpToken::FmvSX, vec![],),
        (RISCVOpToken::FmvXS, vec![],),
        (RISCVOpToken::FnmaddD, vec![],),
        (RISCVOpToken::FnmaddS, vec![],),
        (RISCVOpToken::FnmsubD, vec![],),
        (RISCVOpToken::FnmsubS, vec![],),
        (RISCVOpToken::Fsd, vec![],),
        (RISCVOpToken::FsgnjD, vec![],),
        (RISCVOpToken::FsgnjS, vec![],),
        (RISCVOpToken::FsgnjnD, vec![],),
        (RISCVOpToken::FsgnjnS, vec![],),
        (RISCVOpToken::FsgnjxD, vec![],),
        (RISCVOpToken::FsgnjxS, vec![],),
        (RISCVOpToken::FsqrtD, vec![],),
        (RISCVOpToken::FsqrtS, vec![],),
        (RISCVOpToken::FsubD, vec![],),
        (RISCVOpToken::FsubS, vec![],),
        (RISCVOpToken::Fsw, vec![],),
        (
            RISCVOpToken::Jal,
            vec![
                RISCVOpdSet {
                    hint: "jal label (a0 = pc + 4; pc = label)",
                    tokens: vec![Lbl],
                    aim_basics: vec![RISCVOpdSetAim {
                        op: ParserRISCVInstOp::Jal,
                        opds_idx: vec![0],
                    }],
                },
                RISCVOpdSet {
                    hint: "jal t1, label (t1 = pc + 4; pc = label)",
                    tokens: vec![Reg, Comma, Lbl],
                    aim_basics: vec![RISCVOpdSetAim {
                        op: ParserRISCVInstOp::Jal,
                        opds_idx: vec![0, 2],
                    }],
                },
            ],
        ),
        (RISCVOpToken::Jalr, vec![],),
        (RISCVOpToken::Lb, vec![],),
        (RISCVOpToken::Lbu, vec![],),
        (RISCVOpToken::Lh, vec![],),
        (RISCVOpToken::Lhu, vec![],),
        (RISCVOpToken::Lui, vec![],),
        (RISCVOpToken::Lw, vec![],),
        (RISCVOpToken::Mul, vec![],),
        (RISCVOpToken::Mulh, vec![],),
        (RISCVOpToken::Mulhsu, vec![],),
        (RISCVOpToken::Mulhu, vec![],),
        (RISCVOpToken::Or, vec![],),
        (RISCVOpToken::Ori, vec![],),
        (RISCVOpToken::Rem, vec![],),
        (RISCVOpToken::Remu, vec![],),
        (RISCVOpToken::Sb, vec![],),
        (RISCVOpToken::Sh, vec![],),
        (RISCVOpToken::Sll, vec![],),
        (RISCVOpToken::Slli, vec![],),
        (RISCVOpToken::Slt, vec![],),
        (RISCVOpToken::Slti, vec![],),
        (RISCVOpToken::Sltiu, vec![],),
        (RISCVOpToken::Sltu, vec![],),
        (RISCVOpToken::Sra, vec![],),
        (RISCVOpToken::Srai, vec![],),
        (RISCVOpToken::Srl, vec![],),
        (RISCVOpToken::Srli, vec![],),
        (RISCVOpToken::Sub, vec![],),
        (RISCVOpToken::Sw, vec![],),
        (RISCVOpToken::Uret, vec![],),
        (RISCVOpToken::Wfi, vec![],),
        (RISCVOpToken::Xor, vec![],),
        (RISCVOpToken::Xori, vec![],),
        (RISCVOpToken::B, vec![],),
        (RISCVOpToken::Beqz, vec![],),
        (RISCVOpToken::Bgez, vec![],),
        (RISCVOpToken::Bgt, vec![],),
        (RISCVOpToken::Bgtu, vec![],),
        (RISCVOpToken::Bgtz, vec![],),
        (RISCVOpToken::Ble, vec![],),
        (RISCVOpToken::Bleu, vec![],),
        (RISCVOpToken::Blez, vec![],),
        (RISCVOpToken::Bltz, vec![],),
        (RISCVOpToken::Bnez, vec![],),
        (RISCVOpToken::Call, vec![],),
        (RISCVOpToken::Csrc, vec![],),
        (RISCVOpToken::Csrci, vec![],),
        (RISCVOpToken::Csrr, vec![],),
        (RISCVOpToken::Csrs, vec![],),
        (RISCVOpToken::Csrsi, vec![],),
        (RISCVOpToken::Csrw, vec![],),
        (RISCVOpToken::Csrwi, vec![],),
        (RISCVOpToken::FabsD, vec![],),
        (RISCVOpToken::FabsS, vec![],),
        (RISCVOpToken::FgeD, vec![],),
        (RISCVOpToken::FgeS, vec![],),
        (RISCVOpToken::FgtD, vec![],),
        (RISCVOpToken::FgtS, vec![],),
        (RISCVOpToken::FmvD, vec![],),
        (RISCVOpToken::FmvS, vec![],),
        (RISCVOpToken::FmvWX, vec![],),
        (RISCVOpToken::FmvXW, vec![],),
        (RISCVOpToken::FnegD, vec![],),
        (RISCVOpToken::FnegS, vec![],),
        (RISCVOpToken::Frcsr, vec![],),
        (RISCVOpToken::Frflags, vec![],),
        (RISCVOpToken::Frrm, vec![],),
        (RISCVOpToken::Frsr, vec![],),
        (RISCVOpToken::Fsflags, vec![],),
        (RISCVOpToken::Fsrm, vec![],),
        (RISCVOpToken::Fsrr, vec![],),
        (RISCVOpToken::J, vec![],),
        (RISCVOpToken::Jr, vec![],),
        (RISCVOpToken::La, vec![],),
        (RISCVOpToken::Li, vec![],),
        (RISCVOpToken::Mv, vec![],),
        (RISCVOpToken::Neg, vec![],),
        (RISCVOpToken::Nop, vec![],),
        (RISCVOpToken::Not, vec![],),
        (RISCVOpToken::Rdcycle, vec![],),
        (RISCVOpToken::Rdcycleh, vec![],),
        (RISCVOpToken::Rdinstret, vec![],),
        (RISCVOpToken::Rdinstreth, vec![],),
        (RISCVOpToken::Rdtime, vec![],),
        (RISCVOpToken::Rdtimeh, vec![],),
        (RISCVOpToken::Ret, vec![],),
        (RISCVOpToken::Seqz, vec![],),
        (RISCVOpToken::SextB, vec![],),
        (RISCVOpToken::SextH, vec![],),
        (RISCVOpToken::Sgt, vec![],),
        (RISCVOpToken::Sgtu, vec![],),
        (RISCVOpToken::Sgtz, vec![],),
        (RISCVOpToken::Sltz, vec![],),
        (RISCVOpToken::Snez, vec![],),
        (RISCVOpToken::Tail, vec![],),
        (RISCVOpToken::ZextB, vec![],),
        (RISCVOpToken::ZextH, vec![],),
    ]);
}
