use once_cell::sync::Lazy;

use super::super::super::basic::interface::parser::ParserRISCVInstOp;
use super::super::super::basic::parser::oplist::*;
use super::super::constants::{RV32IInstruction, RV32IRegister};
use super::lexer::RV32IOpToken;
use crate::utility::enum_map::build_map_mut_data;

pub use super::super::super::basic::parser::oplist::RISCVOpdSet;

use RV32IRegister::*;

// --------------------set-------------------------
pub fn opd_set_load_mem(op: ParserRISCVInstOp, name: &str, unit: &str) -> Vec<RISCVOpdSet> {
    vec![
        opd_set(
            expect_opd(vec![Reg, Comma, Imm(I12), LParen, Reg, RParen]),
            vec![basic_op_024(op)],
            format!("{} t1, -0x1(t2) (t1 = {}mem[t2 + -0x1(i12)])", name, unit),
        ),
        opd_set(
            expect_opd(vec![Reg, Comma, LParen, Reg, RParen]),
            vec![basic_op(op, vec![idx(0), imm(0), idx(3)])],
            format!("{} t1, (t2) (t1 = {}mem[t2])", name, unit),
        ),
        opd_set(
            expect_reg_any(Imm(I12)),
            vec![basic_op(op, vec![idx(0), idx(2), reg(Zero)])],
            format!("{} t1, -0x1 (t1 = {}mem[-0x1(i12)])", name, unit),
        ),
        opd_set(
            expect_reg_any(Imm(I32)),
            vec![
                basic_op(
                    RV32IInstruction::Lui.into(),
                    vec![idx(0), idx_handler(2, idx_handler_imm_high)],
                ),
                basic_op(
                    op,
                    vec![idx(0), idx_handler(2, idx_handler_imm_low), idx(0)],
                ),
            ],
            format!(
                "{} t1, 0x100000 (t1 = 0x100000[12:31](i32); t1 = {}mem[t1 + 0x100000[0:11](i32)])",
                name, unit
            ),
        ),
        opd_set(
            expect_reg_any(Lbl),
            vec![
                basic_op(
                    RV32IInstruction::Auipc.into(),
                    vec![idx(0), idx_handler(2, idx_handler_lbl_delta_high)],
                ),
                basic_op(
                    op,
                    vec![
                        idx(0),
                        idx_handler(2, idx_handler_lbl_last_delta_low),
                        idx(0),
                    ],
                ),
            ],
            format!(
                "{} t1, label (t1 = pc + delta[12:31]; t1 = {}mem[t1 + delta[0:11]])",
                name, unit
            ),
        ),
    ]
}
pub fn opd_set_store_mem(op: ParserRISCVInstOp, name: &str, unit: &str) -> Vec<RISCVOpdSet> {
    vec![
        opd_set(
            expect_opd(vec![Reg, Comma, Imm(I12), LParen, Reg, RParen]),
            vec![basic_op_024(op)],
            format!("{} t1, -0x1(t2) (mem[t2 + -0x1(i12)] = {}t1)", name, unit),
        ),
        opd_set(
            expect_opd(vec![Reg, Comma, LParen, Reg, RParen]),
            vec![basic_op(op, vec![idx(0), imm(0), idx(3)])],
            format!("{} t1, (t2) (mem[t2] = {}t1)", name, unit),
        ),
        opd_set(
            expect_reg_any(Imm(I12)),
            vec![basic_op(op, vec![idx(0), idx(2), reg(Zero)])],
            format!("{} t1, -0x1 (mem[-0x1(i12)] = {}t1)", name, unit),
        ),
        opd_set(
            expect_opd(vec![Reg, Comma, Imm(I32), Comma, Reg]),
            vec![
                basic_op(
                    RV32IInstruction::Lui.into(),
                    vec![idx(4), idx_handler(2, idx_handler_imm_high)],
                ),
                basic_op(op, vec![idx(0), idx_handler(2, idx_handler_imm_low), idx(4)]),
            ],
            format!(
                "{} t1, 0x100000, t2 (t2 = 0x100000[12:31](i32); mem[t2 + 0x100000[0:11](i32)] = {}t1)",
                name, unit
            ),
        ),
        opd_set(
            expect_opd(vec![Reg, Comma, Lbl, Comma, Reg]),
            vec![
                basic_op(
                    RV32IInstruction::Auipc.into(),
                    vec![idx(4), idx_handler(2, idx_handler_lbl_delta_high)],
                ),
                basic_op(op, vec![idx(0), idx_handler(2, idx_handler_lbl_last_delta_low), idx(4)]),
            ],
            format!("{} t1, label, t2 (t2 = pc + delta[12:31]; mem[t2 + delta[0:11]] = {}t1)", name, unit),
        ),
    ]
}

pub static OP_LIST: Lazy<Vec<Vec<RISCVOpdSet>>> = Lazy::new(|| {
    let mut op_def = [
        (
            RV32IOpToken::Add,
            vec![opd_set(
                expect_reg_reg_reg(),
                vec![basic_op_024(RV32IInstruction::Add.into())],
                hint_reg_reg_reg("add", "+"),
            )],
        ),
        (
            RV32IOpToken::Addi,
            vec![
                opd_set(
                    expect_reg_reg_any(Imm(I12)),
                    vec![basic_op_024(RV32IInstruction::Addi.into())],
                    hint_reg_reg_any("addi", "-0x1(i12)", "+"),
                ),
                opd_set(
                    expect_reg_reg_any(Lbl),
                    vec![basic_op(
                        RV32IInstruction::Addi.into(),
                        vec![idx(0), idx(2), idx_handler(4, idx_handler_lbl_low)],
                    )],
                    hint_reg_reg_any("addi", "label[0:11]", "+"),
                ),
            ],
        ),
        (
            RV32IOpToken::And,
            vec![opd_set(
                expect_reg_reg_reg(),
                vec![basic_op_024(RV32IInstruction::And.into())],
                hint_reg_reg_reg("and", "&"),
            )],
        ),
        (
            RV32IOpToken::Andi,
            vec![opd_set(
                expect_reg_reg_any(Imm(I12)),
                vec![basic_op_024(RV32IInstruction::Andi.into())],
                hint_reg_reg_any("andi", "-0x1(i12)", "&"),
            )],
        ),
        (
            RV32IOpToken::Auipc,
            vec![opd_set(
                expect_reg_any(Imm(U20)),
                vec![basic_op_02(RV32IInstruction::Auipc.into())],
                "auipc t1, 0x1000 (t1 = pc + 0x1000(u20))".to_string(),
            )],
        ),
        (
            RV32IOpToken::Beq,
            vec![opd_set(
                expect_reg_reg_any(Lbl),
                vec![basic_op_024(RV32IInstruction::Beq.into())],
                hint_branch("beq", "==", " (signed)"),
            )],
        ),
        (
            RV32IOpToken::Bge,
            vec![opd_set(
                expect_reg_reg_any(Lbl),
                vec![basic_op_024(RV32IInstruction::Bge.into())],
                hint_branch("bge", ">=", " (signed)"),
            )],
        ),
        (
            RV32IOpToken::Bgeu,
            vec![opd_set(
                expect_reg_reg_any(Lbl),
                vec![basic_op_024(RV32IInstruction::Bgeu.into())],
                hint_branch("bgeu", ">=", " (unsigned)"),
            )],
        ),
        (
            RV32IOpToken::Blt,
            vec![opd_set(
                expect_reg_reg_any(Lbl),
                vec![basic_op_024(RV32IInstruction::Blt.into())],
                hint_branch("blt", "<", " (signed)"),
            )],
        ),
        (
            RV32IOpToken::Bltu,
            vec![opd_set(
                expect_reg_reg_any(Lbl),
                vec![basic_op_024(RV32IInstruction::Bltu.into())],
                hint_branch("bltu", "<", " (unsigned)"),
            )],
        ),
        (
            RV32IOpToken::Bne,
            vec![opd_set(
                expect_reg_reg_any(Lbl),
                vec![basic_op_024(RV32IInstruction::Bne.into())],
                hint_branch("bne", "!=", " (signed)"),
            )],
        ),
        (
            RV32IOpToken::Csrrc,
            vec![opd_set(
                expect_csr(Reg),
                vec![basic_op_024(RV32IInstruction::Csrrc.into())],
                hint_csr("csrrc", "&= ~", "t2"),
            )],
        ),
        (
            RV32IOpToken::Csrrci,
            vec![opd_set(
                expect_csr(Imm(U5)),
                vec![basic_op_024(RV32IInstruction::Csrrci.into())],
                hint_csr("csrrci", "&= ~", "0x1(u5)"),
            )],
        ),
        (
            RV32IOpToken::Csrrs,
            vec![opd_set(
                expect_csr(Reg),
                vec![basic_op_024(RV32IInstruction::Csrrs.into())],
                hint_csr("csrrs", "|=", "t2"),
            )],
        ),
        (
            RV32IOpToken::Csrrsi,
            vec![opd_set(
                expect_csr(Imm(U5)),
                vec![basic_op_024(RV32IInstruction::Csrrsi.into())],
                hint_csr("csrrsi", "|=", "0x1(u5)"),
            )],
        ),
        (
            RV32IOpToken::Csrrw,
            vec![opd_set(
                expect_csr(Reg),
                vec![basic_op_024(RV32IInstruction::Csrrw.into())],
                hint_csr("csrrw", "=", "t2"),
            )],
        ),
        (
            RV32IOpToken::Csrrwi,
            vec![opd_set(
                expect_csr(Imm(U5)),
                vec![basic_op_024(RV32IInstruction::Csrrwi.into())],
                hint_csr("csrrwi", "=", "0x1(u5)"),
            )],
        ),
        (RV32IOpToken::Div, vec![]),
        (RV32IOpToken::Divu, vec![]),
        (
            RV32IOpToken::Ebreak,
            vec![opd_set_no_opd(RV32IInstruction::Ebreak.into(), "ebreak")],
        ),
        (
            RV32IOpToken::Ecall,
            vec![opd_set_no_opd(RV32IInstruction::Ecall.into(), "ecall")],
        ),
        (
            RV32IOpToken::Fence,
            vec![opd_set(
                expect_opd(vec![Imm(U4), Comma, Imm(U4)]),
                vec![basic_op_02(RV32IInstruction::Fence.into())],
                "fence 0x1(u4), 0x1(u4)".to_string(),
            )],
        ),
        (
            RV32IOpToken::FenceI,
            vec![opd_set_no_opd(RV32IInstruction::FenceI.into(), "fence.i")],
        ),
        (
            RV32IOpToken::Jal,
            vec![
                opd_set(
                    expect_opd(vec![Lbl]),
                    vec![basic_op(
                        RV32IInstruction::Jal.into(),
                        vec![reg(Ra), idx(0)],
                    )],
                    "jal label (ra = pc + 4; pc += delta(i20))".to_string(),
                ),
                opd_set(
                    expect_reg_any(Lbl),
                    vec![basic_op_02(RV32IInstruction::Jal.into())],
                    "jal t1, label (t1 = pc + 4; pc += delta(i20))".to_string(),
                ),
            ],
        ),
        (
            RV32IOpToken::Jalr,
            vec![
                opd_set(
                    expect_reg_reg_any(Imm(I12)),
                    vec![basic_op_024(RV32IInstruction::Jalr.into())],
                    "jalr t1, t2, -0x1(i12) (t1 = pc + 4; pc = t2 + -0x1(i12))".to_string(),
                ),
                opd_set(
                    expect_opd(vec![Reg]),
                    vec![basic_op(
                        RV32IInstruction::Jalr.into(),
                        vec![reg(Ra), idx(0), imm(0)],
                    )],
                    "jalr t0 (ra = pc + 4; pc = t0)".to_string(),
                ),
                opd_set(
                    expect_reg_any(Imm(I12)),
                    vec![basic_op(
                        RV32IInstruction::Jalr.into(),
                        vec![reg(Ra), idx(0), idx(2)],
                    )],
                    "jalr t1, -0x1 (ra = pc + 4; pc = t1 + -0x1(i12))".to_string(),
                ),
                opd_set(
                    expect_opd(vec![Reg, Comma, Imm(I12), LParen, Reg, RParen]),
                    vec![basic_op_042(RV32IInstruction::Jalr.into())],
                    "jalr t1, -0x1(t2) (t1 = pc + 4; pc = t2 + -0x1(i12))".to_string(),
                ),
            ],
        ),
        (
            RV32IOpToken::Lb,
            opd_set_load_mem(RV32IInstruction::Lb.into(), "lb", "(i8)"),
        ),
        (
            RV32IOpToken::Lbu,
            opd_set_load_mem(RV32IInstruction::Lbu.into(), "lbu", "(u8)"),
        ),
        (
            RV32IOpToken::Lh,
            opd_set_load_mem(RV32IInstruction::Lh.into(), "lh", "(i16)"),
        ),
        (
            RV32IOpToken::Lhu,
            opd_set_load_mem(RV32IInstruction::Lhu.into(), "lhu", "(u16)"),
        ),
        (
            RV32IOpToken::Lui,
            vec![
                opd_set(
                    expect_reg_any(Imm(U20)),
                    vec![basic_op_02(RV32IInstruction::Lui.into())],
                    "lui t1, 0x1000 (t1 = 0x1000(u20))".to_string(),
                ),
                opd_set(
                    expect_reg_any(Lbl),
                    vec![basic_op(
                        RV32IInstruction::Lui.into(),
                        vec![idx(0), idx_handler(2, idx_handler_lbl_high)],
                    )],
                    "lui t1, label (t1 = label[12:31])".to_string(),
                ),
            ],
        ),
        (
            RV32IOpToken::Lw,
            opd_set_load_mem(RV32IInstruction::Lw.into(), "lw", ""),
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
                hint_reg_reg_reg("or", "|"),
            )],
        ),
        (
            RV32IOpToken::Ori,
            vec![opd_set(
                expect_reg_reg_any(Imm(I12)),
                vec![basic_op_024(RV32IInstruction::Ori.into())],
                hint_reg_reg_any("ori", "-0x1(i12)", "|"),
            )],
        ),
        (RV32IOpToken::Rem, vec![]),
        (RV32IOpToken::Remu, vec![]),
        (
            RV32IOpToken::Sb,
            opd_set_store_mem(RV32IInstruction::Sb.into(), "sb", "(u8)"),
        ),
        (
            RV32IOpToken::Sh,
            opd_set_store_mem(RV32IInstruction::Sh.into(), "sh", "(u16)"),
        ),
        (
            RV32IOpToken::Sll,
            vec![opd_set(
                expect_reg_reg_reg(),
                vec![basic_op_024(RV32IInstruction::Sll.into())],
                hint_reg_reg_any("sll", "<<", "t3[0:4]"),
            )],
        ),
        (
            RV32IOpToken::Slli,
            vec![opd_set(
                expect_reg_reg_any(Imm(U5)),
                vec![basic_op_024(RV32IInstruction::Slli.into())],
                hint_reg_reg_any("slli", "<<", "0x1(u5)"),
            )],
        ),
        (
            RV32IOpToken::Slt,
            vec![opd_set(
                expect_reg_reg_reg(),
                vec![basic_op_024(RV32IInstruction::Slt.into())],
                hint_set_comparison("slt", "<", "t3", " (signed)"),
            )],
        ),
        (
            RV32IOpToken::Slti,
            vec![opd_set(
                expect_reg_reg_any(Imm(I12)),
                vec![basic_op_024(RV32IInstruction::Slti.into())],
                hint_set_comparison("slti", "<", "-0x1(i12)", " (signed)"),
            )],
        ),
        (
            RV32IOpToken::Sltiu,
            vec![opd_set(
                expect_reg_reg_any(Imm(U12)),
                vec![basic_op_024(RV32IInstruction::Sltiu.into())],
                hint_set_comparison("sltiu", "<", "0x1(u12)", " (unsigned)"),
            )],
        ),
        (
            RV32IOpToken::Sltu,
            vec![opd_set(
                expect_reg_reg_reg(),
                vec![basic_op_024(RV32IInstruction::Sltu.into())],
                hint_set_comparison("sltu", "<", "t3", " (unsigned)"),
            )],
        ),
        (
            RV32IOpToken::Sra,
            vec![opd_set(
                expect_reg_reg_reg(),
                vec![basic_op_024(RV32IInstruction::Sra.into())],
                hint_reg_reg_any("sra", ">>", "t3[0:4]"),
            )],
        ),
        (
            RV32IOpToken::Srai,
            vec![opd_set(
                expect_reg_reg_any(Imm(U5)),
                vec![basic_op_024(RV32IInstruction::Srai.into())],
                hint_reg_reg_any("srai", ">>", "0x1(u5)"),
            )],
        ),
        (
            RV32IOpToken::Srl,
            vec![opd_set(
                expect_reg_reg_reg(),
                vec![basic_op_024(RV32IInstruction::Srl.into())],
                hint_reg_reg_any("srl", ">>", "t3[0:4]"),
            )],
        ),
        (
            RV32IOpToken::Srli,
            vec![opd_set(
                expect_reg_reg_any(Imm(U5)),
                vec![basic_op_024(RV32IInstruction::Srli.into())],
                hint_reg_reg_any("srli", ">>", "0x1(u5)"),
            )],
        ),
        (
            RV32IOpToken::Sub,
            vec![opd_set(
                expect_reg_reg_reg(),
                vec![basic_op_024(RV32IInstruction::Sub.into())],
                hint_reg_reg_reg("sub", "-"),
            )],
        ),
        (
            RV32IOpToken::Sw,
            opd_set_store_mem(RV32IInstruction::Sw.into(), "sw", ""),
        ),
        (RV32IOpToken::Uret, vec![]),
        (RV32IOpToken::Wfi, vec![]),
        (
            RV32IOpToken::Xor,
            vec![opd_set(
                expect_reg_reg_reg(),
                vec![basic_op_024(RV32IInstruction::Xor.into())],
                hint_reg_reg_reg("xor", "^"),
            )],
        ),
        (
            RV32IOpToken::Xori,
            vec![opd_set(
                expect_reg_reg_any(Imm(I12)),
                vec![basic_op_024(RV32IInstruction::Xori.into())],
                hint_reg_reg_any("xori", "-0x1(i12)", "^"),
            )],
        ),
        (
            RV32IOpToken::B,
            vec![opd_set(
                expect_opd(vec![Lbl]),
                vec![basic_op(
                    RV32IInstruction::Jal.into(),
                    vec![reg(Zero), idx(0)],
                )],
                "b label (pc += delta(i20))".to_string(),
            )],
        ),
        (
            RV32IOpToken::Beqz,
            vec![opd_set(
                expect_reg_any(Lbl),
                vec![basic_op(
                    RV32IInstruction::Beq.into(),
                    vec![idx(0), reg(Zero), idx(2)],
                )],
                hint_branch_zero("beqz", "==", ""),
            )],
        ),
        (
            RV32IOpToken::Bgez,
            vec![opd_set(
                expect_reg_any(Lbl),
                vec![basic_op(
                    RV32IInstruction::Bge.into(),
                    vec![idx(0), reg(Zero), idx(2)],
                )],
                hint_branch_zero("bgez", ">=", " (signed)"),
            )],
        ),
        (
            RV32IOpToken::Bgt,
            vec![opd_set(
                expect_reg_reg_any(Lbl),
                vec![basic_op_204(RV32IInstruction::Blt.into())],
                hint_branch("bgt", ">", " (signed)"),
            )],
        ),
        (
            RV32IOpToken::Bgtu,
            vec![opd_set(
                expect_reg_reg_any(Lbl),
                vec![basic_op_204(RV32IInstruction::Bltu.into())],
                hint_branch("bgtu", ">", " (unsigned)"),
            )],
        ),
        (
            RV32IOpToken::Bgtz,
            vec![opd_set(
                expect_reg_any(Lbl),
                vec![basic_op(
                    RV32IInstruction::Blt.into(),
                    vec![reg(Zero), idx(0), idx(2)],
                )],
                hint_branch_zero("bgtz", ">", " (signed)"),
            )],
        ),
        (
            RV32IOpToken::Ble,
            vec![opd_set(
                expect_reg_reg_any(Lbl),
                vec![basic_op_204(RV32IInstruction::Bge.into())],
                hint_branch("ble", "<=", " (signed)"),
            )],
        ),
        (
            RV32IOpToken::Bleu,
            vec![opd_set(
                expect_reg_reg_any(Lbl),
                vec![basic_op_204(RV32IInstruction::Bgeu.into())],
                hint_branch("bleu", "<=", " (unsigned)"),
            )],
        ),
        (
            RV32IOpToken::Blez,
            vec![opd_set(
                expect_reg_any(Lbl),
                vec![basic_op(
                    RV32IInstruction::Bge.into(),
                    vec![reg(Zero), idx(0), idx(2)],
                )],
                hint_branch_zero("blez", "<=", " (signed)"),
            )],
        ),
        (
            RV32IOpToken::Bltz,
            vec![opd_set(
                expect_reg_any(Lbl),
                vec![basic_op(
                    RV32IInstruction::Blt.into(),
                    vec![idx(0), reg(Zero), idx(2)],
                )],
                hint_branch_zero("bltz", "<", " (signed)"),
            )],
        ),
        (
            RV32IOpToken::Bnez,
            vec![opd_set(
                expect_reg_any(Lbl),
                vec![basic_op(
                    RV32IInstruction::Bne.into(),
                    vec![idx(0), reg(Zero), idx(2)],
                )],
                hint_branch_zero("bnez", "!=", ""),
            )],
        ),
        (
            RV32IOpToken::Call,
            vec![opd_set(
                expect_opd(vec![Lbl]),
                vec![
                    basic_op(
                        RV32IInstruction::Auipc.into(),
                        vec![reg(T1), idx_handler(0, idx_handler_lbl_delta_high)],
                    ),
                    basic_op(
                        RV32IInstruction::Jalr.into(),
                        vec![
                            reg(Ra),
                            reg(T1),
                            idx_handler(0, idx_handler_lbl_last_delta_low),
                        ],
                    ),
                ],
                "call label (t1 = pc + delta[12:31]; ra = pc + 4; pc = t1 + delta[0:11])"
                    .to_string(),
            )],
        ),
        (
            RV32IOpToken::Csrc,
            vec![opd_set(
                expect_opd(vec![Reg, Comma, Csr]),
                vec![basic_op(
                    RV32IInstruction::Csrrc.into(),
                    vec![reg(Zero), idx(2), idx(0)],
                )],
                "csrc t1, csr (csr &= ~ t1)".to_string(),
            )],
        ),
        (
            RV32IOpToken::Csrci,
            vec![opd_set(
                expect_opd(vec![Csr, Comma, Imm(U5)]),
                vec![basic_op(
                    RV32IInstruction::Csrrci.into(),
                    vec![reg(Zero), idx(0), idx(2)],
                )],
                "csrci csr, 0x1(u5) (csr &= ~ 0x1(u5))".to_string(),
            )],
        ),
        (
            RV32IOpToken::Csrr,
            vec![opd_set(
                expect_opd(vec![Reg, Comma, Csr]),
                vec![basic_op(
                    RV32IInstruction::Csrrs.into(),
                    vec![idx(0), idx(2), reg(Zero)],
                )],
                "csrr t1, csr (t1 = csr)".to_string(),
            )],
        ),
        (
            RV32IOpToken::Csrs,
            vec![opd_set(
                expect_opd(vec![Reg, Comma, Csr]),
                vec![basic_op(
                    RV32IInstruction::Csrrs.into(),
                    vec![reg(Zero), idx(2), idx(0)],
                )],
                "csrs t1, csr (csr |= t1)".to_string(),
            )],
        ),
        (
            RV32IOpToken::Csrsi,
            vec![opd_set(
                expect_opd(vec![Csr, Comma, Imm(U5)]),
                vec![basic_op(
                    RV32IInstruction::Csrrsi.into(),
                    vec![reg(Zero), idx(0), idx(2)],
                )],
                "csrsi csr, 0x1(u5) (csr |= 0x1(u5))".to_string(),
            )],
        ),
        (
            RV32IOpToken::Csrw,
            vec![opd_set(
                expect_opd(vec![Reg, Comma, Csr]),
                vec![basic_op(
                    RV32IInstruction::Csrrw.into(),
                    vec![reg(Zero), idx(2), idx(0)],
                )],
                "csrw t1, csr (csr = t1)".to_string(),
            )],
        ),
        (
            RV32IOpToken::Csrwi,
            vec![opd_set(
                expect_opd(vec![Csr, Comma, Imm(U5)]),
                vec![basic_op(
                    RV32IInstruction::Csrrwi.into(),
                    vec![reg(Zero), idx(0), idx(2)],
                )],
                "csrwi csr, 0x1(u5) (csr = 0x1(u5))".to_string(),
            )],
        ),
        (
            RV32IOpToken::J,
            vec![opd_set(
                expect_opd(vec![Lbl]),
                vec![basic_op(
                    RV32IInstruction::Jal.into(),
                    vec![reg(Zero), idx(0)],
                )],
                "j label (pc += delta(i20))".to_string(),
            )],
        ),
        (
            RV32IOpToken::Jr,
            vec![
                opd_set(
                    expect_opd(vec![Reg]),
                    vec![basic_op(
                        RV32IInstruction::Jalr.into(),
                        vec![reg(Zero), idx(0), imm(0)],
                    )],
                    "jr t0 (pc = t0)".to_string(),
                ),
                opd_set(
                    expect_reg_any(Imm(I12)),
                    vec![basic_op(
                        RV32IInstruction::Jalr.into(),
                        vec![reg(Zero), idx(0), idx(2)],
                    )],
                    "jr -0x1 (pc = t1 + -0x1(i12))".to_string(),
                ),
            ],
        ),
        (
            RV32IOpToken::La,
            vec![opd_set(
                expect_reg_any(Lbl),
                vec![
                    basic_op(
                        RV32IInstruction::Auipc.into(),
                        vec![idx(0), idx_handler(2, idx_handler_lbl_delta_high)],
                    ),
                    basic_op(
                        RV32IInstruction::Addi.into(),
                        vec![
                            idx(0),
                            idx(0),
                            idx_handler(2, idx_handler_lbl_last_delta_low),
                        ],
                    ),
                ],
                "la t1, label (t1 = label[12:31]; t1 = t1 + label[0:11])".to_string(),
            )],
        ),
        (
            RV32IOpToken::Li,
            vec![
                opd_set(
                    expect_reg_any(Imm(I12)),
                    vec![basic_op(
                        RV32IInstruction::Addi.into(),
                        vec![idx(0), reg(Zero), idx(2)],
                    )],
                    "li t1, -0x1 (t1 = -0x1(i12))".to_string(),
                ),
                opd_set(
                    expect_reg_any(Imm(I32)),
                    vec![
                        basic_op(
                            RV32IInstruction::Lui.into(),
                            vec![idx(0), idx_handler(2, idx_handler_imm_high)],
                        ),
                        basic_op(
                            RV32IInstruction::Addi.into(),
                            vec![idx(0), idx(0), idx_handler(2, idx_handler_imm_low)],
                        ),
                    ],
                    "li t1, 0x100000 (t1 = 0x100000[12:31](i32); t1 = t1 + 0x100000[0:11](i32))"
                        .to_string(),
                ),
            ],
        ),
        (
            RV32IOpToken::Mv,
            vec![opd_set(
                expect_reg_reg(),
                vec![basic_op(
                    RV32IInstruction::Add.into(),
                    vec![idx(0), reg(Zero), idx(2)],
                )],
                "mv t1, t2 (t1 = t2)".to_string(),
            )],
        ),
        (
            RV32IOpToken::Neg,
            vec![opd_set(
                expect_reg_reg(),
                vec![basic_op(
                    RV32IInstruction::Sub.into(),
                    vec![idx(0), reg(Zero), idx(2)],
                )],
                "neg t1, t2 (t1 = -t2)".to_string(),
            )],
        ),
        (
            RV32IOpToken::Nop,
            vec![opd_set(
                expect_opd(vec![]),
                vec![basic_op(
                    RV32IInstruction::Addi.into(),
                    vec![reg(Zero), reg(Zero), imm(0)],
                )],
                "nop".to_string(),
            )],
        ),
        (
            RV32IOpToken::Not,
            vec![opd_set(
                expect_reg_reg(),
                vec![basic_op(
                    RV32IInstruction::Xori.into(),
                    vec![idx(0), idx(2), imm(-1)],
                )],
                "not t1, t2 (t1 = ~t2)".to_string(),
            )],
        ),
        (RV32IOpToken::Rdcycle, vec![]),
        (RV32IOpToken::Rdcycleh, vec![]),
        (RV32IOpToken::Rdinstret, vec![]),
        (RV32IOpToken::Rdinstreth, vec![]),
        (RV32IOpToken::Rdtime, vec![]),
        (RV32IOpToken::Rdtimeh, vec![]),
        (
            RV32IOpToken::Ret,
            vec![opd_set(
                expect_opd(vec![]),
                vec![basic_op(
                    RV32IInstruction::Jalr.into(),
                    vec![reg(Zero), reg(Ra), imm(0)],
                )],
                "ret".to_string(),
            )],
        ),
        (
            RV32IOpToken::Seqz,
            vec![opd_set(
                expect_reg_reg(),
                vec![basic_op(
                    RV32IInstruction::Sltiu.into(),
                    vec![idx(0), idx(2), imm(1)],
                )],
                hint_set_comparison_zero("seqz", "==", ""),
            )],
        ),
        (
            RV32IOpToken::SextB,
            vec![opd_set(
                expect_reg_reg(),
                vec![
                    basic_op(RV32IInstruction::Slli.into(), vec![idx(0), idx(2), imm(24)]),
                    basic_op(RV32IInstruction::Srai.into(), vec![idx(0), idx(0), imm(24)]),
                ],
                "sext.b t1, t2 (t1 = (i8)t2[0:7])".to_string(),
            )],
        ),
        (
            RV32IOpToken::SextH,
            vec![opd_set(
                expect_reg_reg(),
                vec![
                    basic_op(RV32IInstruction::Slli.into(), vec![idx(0), idx(2), imm(16)]),
                    basic_op(RV32IInstruction::Srai.into(), vec![idx(0), idx(0), imm(16)]),
                ],
                "sext.h t1, t2 (t1 = (i16)t2[0:15])".to_string(),
            )],
        ),
        (
            RV32IOpToken::Sgt,
            vec![opd_set(
                expect_reg_reg_reg(),
                vec![basic_op_042(RV32IInstruction::Slt.into())],
                hint_set_comparison("sgt", ">", "t3", " (signed)"),
            )],
        ),
        (
            RV32IOpToken::Sgtu,
            vec![opd_set(
                expect_reg_reg_reg(),
                vec![basic_op_042(RV32IInstruction::Sltu.into())],
                hint_set_comparison("sgtu", ">", "t3", " (unsigned)"),
            )],
        ),
        (
            RV32IOpToken::Sgtz,
            vec![opd_set(
                expect_reg_reg(),
                vec![basic_op(
                    RV32IInstruction::Slt.into(),
                    vec![idx(0), reg(Zero), idx(2)],
                )],
                hint_set_comparison_zero("sgtz", ">", " (signed)"),
            )],
        ),
        (
            RV32IOpToken::Sltz,
            vec![opd_set(
                expect_reg_reg(),
                vec![basic_op(
                    RV32IInstruction::Slt.into(),
                    vec![idx(0), idx(2), reg(Zero)],
                )],
                hint_set_comparison_zero("sltz", "<", " (signed)"),
            )],
        ),
        (
            RV32IOpToken::Snez,
            vec![opd_set(
                expect_reg_reg(),
                vec![basic_op(
                    RV32IInstruction::Sltu.into(),
                    vec![idx(0), reg(Zero), idx(2)],
                )],
                hint_set_comparison_zero("snez", "!=", ""),
            )],
        ),
        (
            RV32IOpToken::Tail,
            vec![opd_set(
                expect_opd(vec![Lbl]),
                vec![
                    basic_op(
                        RV32IInstruction::Auipc.into(),
                        vec![reg(T1), idx_handler(0, idx_handler_lbl_delta_high)],
                    ),
                    basic_op(
                        RV32IInstruction::Jalr.into(),
                        vec![
                            reg(Zero),
                            reg(T1),
                            idx_handler(0, idx_handler_lbl_last_delta_low),
                        ],
                    ),
                ],
                "tail label (t1 = pc + delta[12:31]; pc = t1 + delta[0:11])".to_string(),
            )],
        ),
        (
            RV32IOpToken::ZextB,
            vec![opd_set(
                expect_reg_reg(),
                vec![
                    basic_op(RV32IInstruction::Slli.into(), vec![idx(0), idx(2), imm(24)]),
                    basic_op(RV32IInstruction::Srli.into(), vec![idx(0), idx(0), imm(24)]),
                ],
                "sext.b t1, t2 (t1 = (u8)t2[0:7])".to_string(),
            )],
        ),
        (
            RV32IOpToken::ZextH,
            vec![opd_set(
                expect_reg_reg(),
                vec![
                    basic_op(RV32IInstruction::Slli.into(), vec![idx(0), idx(2), imm(16)]),
                    basic_op(RV32IInstruction::Srli.into(), vec![idx(0), idx(0), imm(16)]),
                ],
                "sext.h t1, t2 (t1 = (u16)t2[0:15])".to_string(),
            )],
        ),
    ];
    build_map_mut_data(&mut op_def, |def| (def.0, std::mem::take(&mut def.1)))
});
