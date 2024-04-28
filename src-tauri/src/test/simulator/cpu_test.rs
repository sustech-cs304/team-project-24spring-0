use crate::interface::assembler::{Instruction, Operand};
use crate::modules::riscv::rv32i::constants::{RISCVImmediate, RV32IInstruction, RV32IRegister};
use crate::simulator::cpu;

fn test_cpu() {
    let mut cpu = cpu::CPU::new(16);
    let add_ins = Instruction {
        op: RV32IInstruction::Add,
        ins: vec![
            Operand::Reg(RV32IRegister::T1),
            Operand::Reg(RV32IRegister::T2),
            Operand::Reg(RV32IRegister::T3),
        ],
    };
    cpu.execute(add_ins).unwrap();
    let sub_ins = Instruction {
        op: RV32IInstruction::Sub,
        ins: vec![
            Operand::Reg(RV32IRegister::T1),
            Operand::Reg(RV32IRegister::T2),
            Operand::Reg(RV32IRegister::T3),
        ],
    };
    let xor_ins = Instruction {
        op: RV32IInstruction::Xor,
        ins: vec![
            Operand::Reg(RV32IRegister::A0),
            Operand::Reg(RV32IRegister::A1),
            Operand::Reg(RV32IRegister::A2),
        ],
    };
    let or_ins = Instruction {
        op: RV32IInstruction::Or,
        ins: vec![
            Operand::Reg(RV32IRegister::A0),
            Operand::Reg(RV32IRegister::A1),
            Operand::Reg(RV32IRegister::A2),
        ],
    };
    let and_ins = Instruction {
        op: RV32IInstruction::And,
        ins: vec![
            Operand::Reg(RV32IRegister::A0),
            Operand::Reg(RV32IRegister::A1),
            Operand::Reg(RV32IRegister::A2),
        ],
    };
    let sll_ins = Instruction {
        op: RV32IInstruction::Sll,
        ins: vec![
            Operand::Reg(RV32IRegister::A0),
            Operand::Reg(RV32IRegister::A1),
            Operand::Reg(RV32IRegister::A2),
        ],
    };
    let srl_ins = Instruction {
        op: RV32IInstruction::Srl,
        ins: vec![
            Operand::Reg(RV32IRegister::A0),
            Operand::Reg(RV32IRegister::A1),
            Operand::Reg(RV32IRegister::A2),
        ],
    };
    let sra_ins = Instruction {
        op: RV32IInstruction::Sra,
        ins: vec![
            Operand::Reg(RV32IRegister::A0),
            Operand::Reg(RV32IRegister::A1),
            Operand::Reg(RV32IRegister::A2),
        ],
    };
    let slt_ins = Instruction {
        op: RV32IInstruction::Slt,
        ins: vec![
            Operand::Reg(RV32IRegister::A0),
            Operand::Reg(RV32IRegister::A1),
            Operand::Reg(RV32IRegister::A2),
        ],
    };
    let sltu_ins = Instruction {
        op: RV32IInstruction::Sltu,
        ins: vec![
            Operand::Reg(RV32IRegister::A0),
            Operand::Reg(RV32IRegister::A1),
            Operand::Reg(RV32IRegister::A2),
        ],
    };
    // B-type
    let beq_ins = Instruction {
        op: RV32IInstruction::Beq,
        ins: vec![
            Operand::Reg(RV32IRegister::T1),
            Operand::Reg(RV32IRegister::T2),
            Operand::Operator(32),
        ],
    };
    let bne_ins = Instruction {
        op: RV32IInstruction::Bne,
        ins: vec![
            Operand::Reg(RV32IRegister::A0),
            Operand::Reg(RV32IRegister::A1),
            Operand::Operator(32),
        ],
    };
    let blt_ins = Instruction {
        op: RV32IInstruction::Blt,
        ins: vec![
            Operand::Reg(RV32IRegister::A0),
            Operand::Reg(RV32IRegister::A1),
            Operand::Operator(32),
        ],
    };
    let bge_ins = Instruction {
        op: RV32IInstruction::Bge,
        ins: vec![
            Operand::Reg(RV32IRegister::A0),
            Operand::Reg(RV32IRegister::A1),
            Operand::Operator(32),
        ],
    };
    let bltu_ins = Instruction {
        op: RV32IInstruction::Bltu,
        ins: vec![
            Operand::Reg(RV32IRegister::A0),
            Operand::Reg(RV32IRegister::A1),
            Operand::Operator(32),
        ],
    };
    let bgeu_ins = Instruction {
        op: RV32IInstruction::Bgeu,
        ins: vec![
            Operand::Reg(RV32IRegister::A0),
            Operand::Reg(RV32IRegister::A1),
            Operand::Operator(32),
        ],
    };
    // J-type
    let jal_ins = Instruction {
        op: RV32IInstruction::Jal,
        ins: vec![Operand::Reg(RV32IRegister::T1), Operand::Operator(32)],
    };
    let jalr_ins = Instruction {
        op: RV32IInstruction::Jalr,
        ins: vec![
            Operand::Reg(RV32IRegister::T1),
            Operand::Reg(RV32IRegister::T2),
            Operand::Operator(32),
        ],
    };
    let lui_ins = Instruction {
        op: RV32IInstruction::Lui,
        ins: vec![Operand::Reg(RV32IRegister::T1), Operand::Operator(32)],
    };
    // I-type
    let addi_ins = Instruction {
        op: RV32IInstruction::Addi,
        ins: vec![
            Operand::Reg(RV32IRegister::A0),
            Operand::Reg(RV32IRegister::A6),
            Operand::Operator(20),
        ],
    };
    let xori_ins = Instruction {
        op: RV32IInstruction::Xori,
        ins: vec![
            Operand::Reg(RV32IRegister::A0),
            Operand::Reg(RV32IRegister::A6),
            Operand::Operator(20),
        ],
    };
    let ori_ins = Instruction {
        op: RV32IInstruction::Ori,
        ins: vec![
            Operand::Reg(RV32IRegister::A0),
            Operand::Reg(RV32IRegister::A6),
            Operand::Operator(20),
        ],
    };
    let andi_ins = Instruction {
        op: RV32IInstruction::Andi,
        ins: vec![
            Operand::Reg(RV32IRegister::A2),
            Operand::Reg(RV32IRegister::A4),
            Operand::Operator(20),
        ],
    };
    let sltiu_ins = Instruction {
        op: RV32IInstruction::Sltiu,
        ins: vec![
            Operand::Reg(RV32IRegister::T0),
            Operand::Reg(RV32IRegister::T1),
            Operand::Operator(20),
        ],
    };
    let slti_ins = Instruction {
        op: RV32IInstruction::Slti,
        ins: vec![
            Operand::Reg(RV32IRegister::T3),
            Operand::Reg(RV32IRegister::A3),
            Operand::Operator(20),
        ],
    };
    let slli_ins = Instruction {
        op: RV32IInstruction::Slli,
        ins: vec![
            Operand::Reg(RV32IRegister::T3),
            Operand::Reg(RV32IRegister::A3),
            Operand::Operator(20),
        ],
    };
    let srli_ins = Instruction {
        op: RV32IInstruction::Srli,
        ins: vec![
            Operand::Reg(RV32IRegister::T3),
            Operand::Reg(RV32IRegister::A3),
            Operand::Operator(20),
        ],
    };
    let srai_ins = Instruction {
        op: RV32IInstruction::Srai,
        ins: vec![
            Operand::Reg(RV32IRegister::T3),
            Operand::Reg(RV32IRegister::A3),
            Operand::Operator(20),
        ],
    };
    // L-type
    let lb_ins = Instruction {
        op: RV32IInstruction::Lb,
        ins: vec![Operand::Reg(RV32IRegister::A0), Operand::Operator(20)],
    };
    let lh_ins = Instruction {
        op: RV32IInstruction::Lh,
        ins: vec![Operand::Reg(RV32IRegister::A1), Operand::Operator(20)],
    };
}

fn main() {
    test_cpu();
}
