use super::super::super::basic::interface::parser::ParserRISCVInstOp;
use super::super::super::basic::parser::oplist::*;
use super::super::constants::{RV32IInstruction, RV32IRegister};
use super::lexer::RV32IOpToken;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub use super::super::super::basic::parser::oplist::RISCVOpdSet;

use RV32IRegister::*;

// --------------------set-------------------------
pub fn opd_set_sl_mem(
    op: ParserRISCVInstOp,
    name: &str,
    unit: &str,
    src: [&str; 5],
    dst: [&str; 5],
) -> Vec<RISCVOpdSet> {
    vec![
        opd_set(
            expect_opd(vec![Reg, Comma, Imm(I12), LParen, Reg, RParen]),
            vec![basic_op_024(op)],
            format!("{} t1, -0x1(t2) ({} = {}{})", name, dst[0], unit, src[0]),
        ),
        opd_set(
            expect_opd(vec![Reg, Comma, LParen, Reg, RParen]),
            vec![basic_op(op, vec![idx(0), imm_i(0), idx(3)])],
            format!("{} t1, (t2) ({} = {}{})", name, dst[1], unit, src[1]),
        ),
        opd_set(
            expect_reg_any(Imm(I12)),
            vec![basic_op(op, vec![idx(0), idx(2), reg(Zero)])],
            format!("{} t1, -0x1 ({} = {}{})", name, dst[2], unit, src[2]),
        ),
        opd_set(
            expect_reg_any(Imm(I32)),
            vec![
                basic_op(
                    RV32IInstruction::Lui.into(),
                    vec![reg(A0), idx_handler(2, idx_handler_high)],
                ),
                basic_op(op, vec![idx(0), idx_handler(2, idx_handler_low), reg(A0)]),
            ],
            format!(
                "{} t1, 0x100000 (a0 = 0x100000[12:31](i32); {} = {}{})",
                name, dst[3], unit, src[3]
            ),
        ),
        opd_set(
            expect_reg_any(Lbl),
            vec![basic_op_024(op)],
            format!("{} t1, label ({} = {}{})", name, dst[4], unit, src[4]),
        ),
    ]
}
const SL_MEM_REG: [&str; 5] = ["t1", "t1", "t1", "t1", "t1"];
const SL_MEM_MEM: [&str; 5] = [
    "mem[t2 + -0x1(i12)]",
    "mem[t2]",
    "mem[-0x1(i12)]",
    "mem[a0 + 0x100000[0:11](i32)]",
    "mem[label]",
];
pub fn opd_set_load_mem(op: ParserRISCVInstOp, name: &str, unit: &str) -> Vec<RISCVOpdSet> {
    opd_set_sl_mem(op, name, unit, SL_MEM_MEM, SL_MEM_REG)
}
pub fn opd_set_store_mem(op: ParserRISCVInstOp, name: &str, unit: &str) -> Vec<RISCVOpdSet> {
    opd_set_sl_mem(op, name, unit, SL_MEM_REG, SL_MEM_MEM)
}

lazy_static! {
    pub static ref OP_LIST: HashMap<RV32IOpToken, Vec<RISCVOpdSet>> = HashMap::from([
        (
            RV32IOpToken::Add,
            vec![opd_set(
                expect_reg_reg_reg(),
                vec![basic_op_024(RV32IInstruction::Add.into())],
                hint_reg_reg_reg("add", "+")
            )]
        ),
        (
            RV32IOpToken::Addi,
            vec![
                opd_set(
                    expect_reg_reg_any(Imm(I12)),
                    vec![basic_op_024(RV32IInstruction::Addi.into())],
                    hint_reg_reg_any("addi", "-0x1(i12)", "+")
                ),
                opd_set(
                    expect_reg_reg_any(Lbl),
                    vec![basic_op_024(RV32IInstruction::Addi.into())],
                    hint_reg_reg_any("addi", "label[0:11]", "+")
                ),
            ]
        ),
        (
            RV32IOpToken::And,
            vec![opd_set(
                expect_reg_reg_reg(),
                vec![basic_op_024(RV32IInstruction::And.into())],
                hint_reg_reg_reg("and", "&")
            )]
        ),
        (
            RV32IOpToken::Andi,
            vec![opd_set(
                expect_reg_reg_any(Imm(U12)),
                vec![basic_op_024(RV32IInstruction::Andi.into())],
                hint_reg_reg_any("andi", "0x1(u12)", "&")
            )]
        ),
        (
            RV32IOpToken::Auipc,
            vec![opd_set(
                expect_reg_any(Imm(U20)),
                vec![basic_op_02(RV32IInstruction::Auipc.into())],
                "auipc t1, 0x1000 (t1 = pc + 0x1000(u20))".to_string()
            )]
        ),
        (
            RV32IOpToken::Beq,
            vec![opd_set(
                expect_reg_reg_any(Lbl),
                vec![basic_op_024(RV32IInstruction::Beq.into())],
                hint_branch("beq", "==", " (signed)")
            )]
        ),
        (
            RV32IOpToken::Bge,
            vec![opd_set(
                expect_reg_reg_any(Lbl),
                vec![basic_op_024(RV32IInstruction::Bge.into())],
                hint_branch("bge", ">=", " (signed)")
            )]
        ),
        (
            RV32IOpToken::Bgeu,
            vec![opd_set(
                expect_reg_reg_any(Lbl),
                vec![basic_op_024(RV32IInstruction::Bgeu.into())],
                hint_branch("bgeu", ">=", " (unsigned)")
            )]
        ),
        (
            RV32IOpToken::Blt,
            vec![opd_set(
                expect_reg_reg_any(Lbl),
                vec![basic_op_024(RV32IInstruction::Blt.into())],
                hint_branch("blt", "<", " (signed)")
            )]
        ),
        (
            RV32IOpToken::Bltu,
            vec![opd_set(
                expect_reg_reg_any(Lbl),
                vec![basic_op_024(RV32IInstruction::Bltu.into())],
                hint_branch("bltu", "<", " (unsigned)")
            )]
        ),
        (
            RV32IOpToken::Bne,
            vec![opd_set(
                expect_reg_reg_any(Lbl),
                vec![basic_op_024(RV32IInstruction::Bne.into())],
                hint_branch("bne", "!=", " (signed)")
            )]
        ),
        (
            RV32IOpToken::Csrrc,
            vec![opd_set(
                expect_csr(Reg),
                vec![basic_op_024(RV32IInstruction::Csrrc.into())],
                hint_csr("csrrc", "&= ~", "t2")
            )]
        ),
        (
            RV32IOpToken::Csrrci,
            vec![opd_set(
                expect_csr(Imm(U5)),
                vec![basic_op_024(RV32IInstruction::Csrrci.into())],
                hint_csr("csrrci", "&= ~", "0x1(u5)")
            )]
        ),
        (
            RV32IOpToken::Csrrs,
            vec![opd_set(
                expect_csr(Reg),
                vec![basic_op_024(RV32IInstruction::Csrrs.into())],
                hint_csr("csrrs", "|=", "t2")
            )]
        ),
        (
            RV32IOpToken::Csrrsi,
            vec![opd_set(
                expect_csr(Imm(U5)),
                vec![basic_op_024(RV32IInstruction::Csrrsi.into())],
                hint_csr("csrrsi", "|=", "0x1(u5)")
            )]
        ),
        (
            RV32IOpToken::Csrrw,
            vec![opd_set(
                expect_csr(Reg),
                vec![basic_op_024(RV32IInstruction::Csrrw.into())],
                hint_csr("csrrw", "=", "t2")
            )]
        ),
        (
            RV32IOpToken::Csrrwi,
            vec![opd_set(
                expect_csr(Imm(U5)),
                vec![basic_op_024(RV32IInstruction::Csrrwi.into())],
                hint_csr("csrrwi", "=", "0x1(u5)")
            )]
        ),
        (RV32IOpToken::Div, vec![]),
        (RV32IOpToken::Divu, vec![]),
        (
            RV32IOpToken::Ebreak,
            vec![opd_set_no_opd(RV32IInstruction::Ebreak.into(), "ebreak")]
        ),
        (
            RV32IOpToken::Ecall,
            vec![opd_set_no_opd(RV32IInstruction::Ecall.into(), "ecall")]
        ),
        (RV32IOpToken::FaddD, vec![]),
        (RV32IOpToken::FaddS, vec![]),
        (RV32IOpToken::FclassD, vec![]),
        (RV32IOpToken::FclassS, vec![]),
        (RV32IOpToken::FcvtDS, vec![]),
        (RV32IOpToken::FcvtDW, vec![]),
        (RV32IOpToken::FcvtDWu, vec![]),
        (RV32IOpToken::FcvtSD, vec![]),
        (RV32IOpToken::FcvtSW, vec![]),
        (RV32IOpToken::FcvtSWu, vec![]),
        (RV32IOpToken::FcvtWD, vec![]),
        (RV32IOpToken::FcvtWS, vec![]),
        (RV32IOpToken::FcvtWuD, vec![]),
        (RV32IOpToken::FcvtWuS, vec![]),
        (RV32IOpToken::FdivD, vec![]),
        (RV32IOpToken::FdivS, vec![]),
        (
            RV32IOpToken::Fence,
            vec![opd_set(
                expect_opd(vec![Imm(U4), Comma, Imm(U4)]),
                vec![basic_op_02(RV32IInstruction::Fence.into())],
                "fence 0x1(u4), 0x1(u4)".to_string()
            )]
        ),
        (
            RV32IOpToken::FenceI,
            vec![opd_set_no_opd(RV32IInstruction::FenceI.into(), "fence.i")]
        ),
        (RV32IOpToken::FeqD, vec![]),
        (RV32IOpToken::FeqS, vec![]),
        (RV32IOpToken::Fld, vec![]),
        (RV32IOpToken::FleD, vec![]),
        (RV32IOpToken::FleS, vec![]),
        (RV32IOpToken::FltD, vec![]),
        (RV32IOpToken::FltS, vec![]),
        (RV32IOpToken::Flw, vec![]),
        (RV32IOpToken::FmaddD, vec![]),
        (RV32IOpToken::FmaddS, vec![]),
        (RV32IOpToken::FmaxD, vec![]),
        (RV32IOpToken::FmaxS, vec![]),
        (RV32IOpToken::FminD, vec![]),
        (RV32IOpToken::FminS, vec![]),
        (RV32IOpToken::FmsubD, vec![]),
        (RV32IOpToken::FmsubS, vec![]),
        (RV32IOpToken::FmulD, vec![]),
        (RV32IOpToken::FmulS, vec![]),
        (RV32IOpToken::FmvSX, vec![]),
        (RV32IOpToken::FmvXS, vec![]),
        (RV32IOpToken::FnmaddD, vec![]),
        (RV32IOpToken::FnmaddS, vec![]),
        (RV32IOpToken::FnmsubD, vec![]),
        (RV32IOpToken::FnmsubS, vec![]),
        (RV32IOpToken::Fsd, vec![]),
        (RV32IOpToken::FsgnjD, vec![]),
        (RV32IOpToken::FsgnjS, vec![]),
        (RV32IOpToken::FsgnjnD, vec![]),
        (RV32IOpToken::FsgnjnS, vec![]),
        (RV32IOpToken::FsgnjxD, vec![]),
        (RV32IOpToken::FsgnjxS, vec![]),
        (RV32IOpToken::FsqrtD, vec![]),
        (RV32IOpToken::FsqrtS, vec![]),
        (RV32IOpToken::FsubD, vec![]),
        (RV32IOpToken::FsubS, vec![]),
        (RV32IOpToken::Fsw, vec![]),
        (
            RV32IOpToken::Jal,
            vec![
                opd_set(
                    expect_opd(vec![Lbl]),
                    vec![basic_op(
                        RV32IInstruction::Jal.into(),
                        vec![reg(Ra), idx(0)]
                    )],
                    "jal label (ra = pc + 4; pc = label)".to_string()
                ),
                opd_set(
                    expect_reg_any(Lbl),
                    vec![basic_op_02(RV32IInstruction::Jal.into())],
                    "jal t1, label (t1 = pc + 4; pc = label)".to_string()
                )
            ]
        ),
        (
            RV32IOpToken::Jalr,
            vec![
                opd_set(
                    expect_reg_reg_any(Imm(I12)),
                    vec![basic_op_024(RV32IInstruction::Jalr.into())],
                    "jalr t1, t2, -0x1(i12) (t1 = pc + 4; pc = t2 + -0x1(i12))".to_string()
                ),
                opd_set(
                    expect_opd(vec![Reg]),
                    vec![basic_op(
                        RV32IInstruction::Jalr.into(),
                        vec![reg(Ra), idx(0), imm_i(0)]
                    )],
                    "jalr t0 (ra = pc + 4; pc = t0)".to_string()
                ),
                opd_set(
                    expect_reg_any(Imm(I12)),
                    vec![basic_op(
                        RV32IInstruction::Jalr.into(),
                        vec![reg(Ra), idx(0), idx(2)]
                    )],
                    "jalr t1, -0x1 (ra = pc + 4; pc = t1 + -0x1(i12))".to_string()
                ),
                opd_set(
                    expect_opd(vec![Reg, Comma, Imm(I12), LParen, Reg, RParen]),
                    vec![basic_op_042(RV32IInstruction::Jalr.into())],
                    "jalr t1, -0x1(t2) (t1 = pc + 4; pc = t2 + -0x1(i12))".to_string()
                )
            ]
        ),
        (
            RV32IOpToken::Lb,
            opd_set_load_mem(RV32IInstruction::Lb.into(), "lb", "(i8)")
        ),
        (
            RV32IOpToken::Lbu,
            opd_set_load_mem(RV32IInstruction::Lbu.into(), "lbu", "(u8)")
        ),
        (
            RV32IOpToken::Lh,
            opd_set_load_mem(RV32IInstruction::Lh.into(), "lh", "(i16)")
        ),
        (
            RV32IOpToken::Lhu,
            opd_set_load_mem(RV32IInstruction::Lhu.into(), "lhu", "(u16)")
        ),
        (
            RV32IOpToken::Lui,
            vec![
                opd_set(
                    expect_reg_any(Imm(U20)),
                    vec![basic_op_02(RV32IInstruction::Lui.into())],
                    "lui t1, 0x1000 (t1 = 0x1000(u20))".to_string()
                ),
                opd_set(
                    expect_reg_any(Lbl),
                    vec![basic_op_02(RV32IInstruction::Lui.into())],
                    "lui t1, label (t1 = label)".to_string()
                )
            ]
        ),
        (
            RV32IOpToken::Lw,
            opd_set_load_mem(RV32IInstruction::Lw.into(), "lw", "")
        ),
        (RV32IOpToken::Mul, vec![]),
        (RV32IOpToken::Mulh, vec![]),
        (RV32IOpToken::Mulhsu, vec![]),
        (RV32IOpToken::Mulhu, vec![]),
        (
            RV32IOpToken::Or,
            vec![opd_set(
                expect_reg_reg_reg(),
                vec![basic_op_024(RV32IInstruction::Or.into())],
                hint_reg_reg_reg("or", "|")
            ),]
        ),
        (
            RV32IOpToken::Ori,
            vec![opd_set(
                expect_reg_reg_any(Imm(U12)),
                vec![basic_op_024(RV32IInstruction::Ori.into())],
                hint_reg_reg_any("ori", "0x1(u12)", "|")
            )]
        ),
        (RV32IOpToken::Rem, vec![]),
        (RV32IOpToken::Remu, vec![]),
        (
            RV32IOpToken::Sb,
            opd_set_store_mem(RV32IInstruction::Sb.into(), "sb", "(u8)")
        ),
        (
            RV32IOpToken::Sh,
            opd_set_store_mem(RV32IInstruction::Sh.into(), "sh", "(u16)")
        ),
        (
            RV32IOpToken::Sll,
            vec![opd_set(
                expect_reg_reg_reg(),
                vec![basic_op_024(RV32IInstruction::Sll.into())],
                hint_reg_reg_any("sll", "<<", "t3[0:4]")
            )]
        ),
        (
            RV32IOpToken::Slli,
            vec![opd_set(
                expect_reg_reg_any(Imm(U5)),
                vec![basic_op_024(RV32IInstruction::Slli.into())],
                hint_reg_reg_any("slli", "<<", "0x1(u5)")
            )]
        ),
        (
            RV32IOpToken::Slt,
            vec![opd_set(
                expect_reg_reg_reg(),
                vec![basic_op_024(RV32IInstruction::Slt.into())],
                hint_set_comparison("<", "t3", " (signed)")
            )]
        ),
        (
            RV32IOpToken::Slti,
            vec![opd_set(
                expect_reg_reg_any(Imm(I12)),
                vec![basic_op_024(RV32IInstruction::Slti.into())],
                hint_set_comparison("<", "-0x1(i12)", " (signed)")
            )]
        ),
        (
            RV32IOpToken::Sltiu,
            vec![opd_set(
                expect_reg_reg_any(Imm(U12)),
                vec![basic_op_024(RV32IInstruction::Sltiu.into())],
                hint_set_comparison("<", "0x1(u12)", " (unsigned)")
            )]
        ),
        (
            RV32IOpToken::Sltu,
            vec![opd_set(
                expect_reg_reg_reg(),
                vec![basic_op_024(RV32IInstruction::Sltu.into())],
                hint_set_comparison("<", "t3", " (unsigned)")
            )]
        ),
        (
            RV32IOpToken::Sra,
            vec![opd_set(
                expect_reg_reg_reg(),
                vec![basic_op_024(RV32IInstruction::Sra.into())],
                hint_reg_reg_any("sra", ">>", "t3[0:4]")
            )]
        ),
        (
            RV32IOpToken::Srai,
            vec![opd_set(
                expect_reg_reg_any(Imm(U5)),
                vec![basic_op_024(RV32IInstruction::Srai.into())],
                hint_reg_reg_any("srai", ">>", "0x1(u5)")
            )]
        ),
        (
            RV32IOpToken::Srl,
            vec![opd_set(
                expect_reg_reg_reg(),
                vec![basic_op_024(RV32IInstruction::Srl.into())],
                hint_reg_reg_any("srl", ">>", "t3[0:4]")
            )]
        ),
        (
            RV32IOpToken::Srli,
            vec![opd_set(
                expect_reg_reg_any(Imm(U5)),
                vec![basic_op_024(RV32IInstruction::Srli.into())],
                hint_reg_reg_any("srli", ">>", "0x1(u5)")
            )]
        ),
        (
            RV32IOpToken::Sub,
            vec![opd_set(
                expect_reg_reg_reg(),
                vec![basic_op_024(RV32IInstruction::Sub.into())],
                hint_reg_reg_reg("sub", "-")
            )]
        ),
        (
            RV32IOpToken::Sw,
            opd_set_store_mem(RV32IInstruction::Sw.into(), "sw", "")
        ),
        (RV32IOpToken::Uret, vec![]),
        (RV32IOpToken::Wfi, vec![]),
        (
            RV32IOpToken::Xor,
            vec![opd_set(
                expect_reg_reg_reg(),
                vec![basic_op_024(RV32IInstruction::Xor.into())],
                hint_reg_reg_reg("xor", "^")
            )]
        ),
        (
            RV32IOpToken::Xori,
            vec![opd_set(
                expect_reg_reg_any(Imm(U12)),
                vec![basic_op_024(RV32IInstruction::Xori.into())],
                hint_reg_reg_any("xori", "0x1(u12)", "^")
            )]
        ),
        (
            RV32IOpToken::B,
            vec![opd_set(
                expect_opd(vec![Lbl]),
                vec![basic_op(
                    RV32IInstruction::Jal.into(),
                    vec![reg(Ra), idx(0)]
                )],
                "b label (ra = pc + 4; pc = label)".to_string()
            )]
        ),
        (
            RV32IOpToken::Beqz,
            vec![opd_set(
                expect_reg_any(Lbl),
                vec![basic_op(
                    RV32IInstruction::Beq.into(),
                    vec![idx(0), reg(Zero), idx(2)]
                )],
                hint_branch_zero("beqz", "==", "")
            )]
        ),
        (
            RV32IOpToken::Bgez,
            vec![opd_set(
                expect_reg_any(Lbl),
                vec![basic_op(
                    RV32IInstruction::Bge.into(),
                    vec![idx(0), reg(Zero), idx(2)]
                )],
                hint_branch_zero("bgez", ">=", " (signed)")
            )]
        ),
        (
            RV32IOpToken::Bgt,
            vec![opd_set(
                expect_reg_reg_any(Lbl),
                vec![basic_op_204(RV32IInstruction::Blt.into())],
                hint_branch("bgt", ">", " (signed)")
            )]
        ),
        (
            RV32IOpToken::Bgtu,
            vec![opd_set(
                expect_reg_reg_any(Lbl),
                vec![basic_op_204(RV32IInstruction::Bltu.into())],
                hint_branch("bgtu", ">", " (unsigned)")
            )]
        ),
        (
            RV32IOpToken::Bgtz,
            vec![opd_set(
                expect_reg_any(Lbl),
                vec![basic_op(
                    RV32IInstruction::Blt.into(),
                    vec![reg(Zero), idx(0), idx(2)]
                )],
                hint_branch_zero("bgtz", ">", " (signed)")
            )]
        ),
        (
            RV32IOpToken::Ble,
            vec![opd_set(
                expect_reg_reg_any(Lbl),
                vec![basic_op_204(RV32IInstruction::Bge.into())],
                hint_branch("ble", "<=", " (signed)")
            )]
        ),
        (
            RV32IOpToken::Bleu,
            vec![opd_set(
                expect_reg_reg_any(Lbl),
                vec![basic_op_204(RV32IInstruction::Bgeu.into())],
                hint_branch("bleu", "<=", " (unsigned)")
            )]
        ),
        (
            RV32IOpToken::Blez,
            vec![opd_set(
                expect_reg_any(Lbl),
                vec![basic_op(
                    RV32IInstruction::Bge.into(),
                    vec![reg(Zero), idx(0), idx(2)]
                )],
                hint_branch_zero("blez", "<=", " (signed)")
            )]
        ),
        (
            RV32IOpToken::Bltz,
            vec![opd_set(
                expect_reg_any(Lbl),
                vec![basic_op(
                    RV32IInstruction::Blt.into(),
                    vec![idx(0), reg(Zero), idx(2)]
                )],
                hint_branch_zero("bltz", "<", " (signed)")
            )]
        ),
        (
            RV32IOpToken::Bnez,
            vec![opd_set(
                expect_reg_any(Lbl),
                vec![basic_op(
                    RV32IInstruction::Bne.into(),
                    vec![idx(0), reg(Zero), idx(2)]
                )],
                hint_branch_zero("bnez", "!=", "")
            )]
        ),
        (RV32IOpToken::Call, vec![]),
        (RV32IOpToken::Csrc, vec![]),
        (RV32IOpToken::Csrci, vec![]),
        (RV32IOpToken::Csrr, vec![]),
        (RV32IOpToken::Csrs, vec![]),
        (RV32IOpToken::Csrsi, vec![]),
        (RV32IOpToken::Csrw, vec![]),
        (RV32IOpToken::Csrwi, vec![]),
        (RV32IOpToken::FabsD, vec![]),
        (RV32IOpToken::FabsS, vec![]),
        (RV32IOpToken::FgeD, vec![]),
        (RV32IOpToken::FgeS, vec![]),
        (RV32IOpToken::FgtD, vec![]),
        (RV32IOpToken::FgtS, vec![]),
        (RV32IOpToken::FmvD, vec![]),
        (RV32IOpToken::FmvS, vec![]),
        (RV32IOpToken::FmvWX, vec![]),
        (RV32IOpToken::FmvXW, vec![]),
        (RV32IOpToken::FnegD, vec![]),
        (RV32IOpToken::FnegS, vec![]),
        (RV32IOpToken::Frcsr, vec![]),
        (RV32IOpToken::Frflags, vec![]),
        (RV32IOpToken::Frrm, vec![]),
        (RV32IOpToken::Frsr, vec![]),
        (RV32IOpToken::Fsflags, vec![]),
        (RV32IOpToken::Fsrm, vec![]),
        (RV32IOpToken::Fsrr, vec![]),
        (RV32IOpToken::J, vec![]),
        (RV32IOpToken::Jr, vec![]),
        (RV32IOpToken::La, vec![]),
        (RV32IOpToken::Li, vec![]),
        (RV32IOpToken::Mv, vec![]),
        (RV32IOpToken::Neg, vec![]),
        (
            RV32IOpToken::Nop,
            vec![opd_set(
                expect_opd(vec![]),
                vec![basic_op(
                    RV32IInstruction::Addi.into(),
                    vec![reg(Zero), reg(Zero), imm_i(0)]
                )],
                "nop".to_string()
            )]
        ),
        (RV32IOpToken::Not, vec![]),
        (RV32IOpToken::Rdcycle, vec![]),
        (RV32IOpToken::Rdcycleh, vec![]),
        (RV32IOpToken::Rdinstret, vec![]),
        (RV32IOpToken::Rdinstreth, vec![]),
        (RV32IOpToken::Rdtime, vec![]),
        (RV32IOpToken::Rdtimeh, vec![]),
        (RV32IOpToken::Ret, vec![]),
        (RV32IOpToken::Seqz, vec![]),
        (RV32IOpToken::SextB, vec![]),
        (RV32IOpToken::SextH, vec![]),
        (RV32IOpToken::Sgt, vec![]),
        (RV32IOpToken::Sgtu, vec![]),
        (RV32IOpToken::Sgtz, vec![]),
        (RV32IOpToken::Sltz, vec![]),
        (RV32IOpToken::Snez, vec![]),
        (RV32IOpToken::Tail, vec![]),
        (RV32IOpToken::ZextB, vec![]),
        (RV32IOpToken::ZextH, vec![]),
    ]);
}